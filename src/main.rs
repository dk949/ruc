use std::collections::HashMap;
use std::convert::identity;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::process::Stdio;

#[derive(PartialEq, Copy, Clone)]
enum Codes {
    InternalError = -1,
    Ok = 0,
    ArgumentError = 1,
    LanguageError = 2,
    RunnerError = 3,
    DependencyError = 4,
    EditorError = 5,
    FileError = 6,
    CodeError = 7,
}

type Error<T> = Result<T, Codes>;

fn exit(code: Codes) -> ! {
    process::exit(code as i32)
}

fn add_internal_error(code: Codes) {
    if code == Codes::InternalError {
        print!("Internal error: ")
    }
}

macro_rules! die {
    ($code:expr, $($args:tt)*) => {{
        add_internal_error($code);
        println!($($args)*);
        exit($code)
    }}
}

macro_rules! dier {
    ($code:expr, $($args:tt)*) => {{
        add_internal_error($code);
        println!($($args)*);
        return Err($code)
    }}
}

macro_rules! dieo {
    ($code:expr, $($args:tt)*) => {{
        add_internal_error($code);
        println!($($args)*);
        return $code
    }}
}

const HLINE: &str = "――――――――――――――――――――――――――";

#[derive(PartialEq, Clone, Copy)]
enum Hist {
    Use,
    Temp,
    New,
}

#[derive(PartialEq, Clone, Copy)]
enum List {
    None,
    Langs,
    Aliases,
    Runners,
}

struct Args {
    hist: Hist,
    list: List,
    editor: String,
    compiler_args: Vec<String>,
    prog_args: Vec<String>,
    runner: Option<String>,
    lang: String,
}

fn parse_args() -> Args {
    fn help() -> ! {
        die!(
            Codes::Ok,
            r#"usage: {} LANG [OPTIONS]

            Open the EDITOR. Write some code. Have it executed.

            positional arguments:
            LANG                    language to be ran

            options:

            -r, --runner            select which runner to use

            -e, --editor            specify name of the editor to use.
                                    by default uses the EDITOR environment
                                    variable

            -t, --temp              ignore history and use default snippet
            -n, --new-history       reset current language history to default
            -u, --use-histoty       use the history file (default)

            -l, --ls                list available languages
            -a, --aliases           list available aliases
                --list-runners      list available runners. if a language is
                                    specified, only runners for that language are
                                    listed

            --args ARGS             space separated list of arguments to be passed
                                    to the compiler or the interpreter.
            --argv ARGS             space separated list of arguments to be
                                    passed to the executed program

            -h, --help              show this help message and exit
            -v, --version           print program version


            Notes:
                Between -t, -n and -u, the last option specified will be used

                Between -l, -a and --list-runners, the last option specified will be used

            Exit codes:
                -1: Internal error
                 0: OK
                 1: Argument error
                 2: Language error
                 3: Runner error
                 4: Dependency error
                 5: Editor error
                 6: File error
                 7: Code error
            "#,
            Path::new(&env::args().next().unwrap())
                .file_name()
                .unwrap()
                .to_string_lossy()
        )
    }
    let mut hist = Hist::Use;
    let mut list = List::None;
    let mut editor = String::new();
    let mut compiler_args = Vec::new();
    let mut prog_args = Vec::new();
    let mut runner = None;
    let mut lang = String::new();

    let mut args = env::args();
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--temp" => hist = Hist::Temp,
            "-n" | "--new-history" => hist = Hist::New,
            "-u" | "--use-history" => hist = Hist::Use,
            "-l" | "--ls" => list = List::Langs,
            "-a" | "--aliases" => list = List::Aliases,
            "--list-runners" => list = List::Runners,
            "-v" | "--version" => {
                println!(env!("CARGO_PKG_VERSION"));
                exit(Codes::Ok);
            }
            "-h" | "--help" => help(),
            "--args" | "--argv" => {
                let split = args
                    .next()
                    .unwrap_or_else(|| {
                        die!(
                            Codes::ArgumentError,
                            "Expected a list of space separated arguments after '{arg}'"
                        )
                    })
                    .split(' ')
                    .map(str::to_string)
                    .collect();
                if arg == "--args" {
                    compiler_args = split;
                } else {
                    prog_args = split;
                }
            }
            flag @ "-r" | flag @ "--runner" => {
                runner = Some(args.next().unwrap_or_else(|| {
                    die!(
                        Codes::ArgumentError,
                        "Expected a runner name after '{flag}'"
                    )
                }));
            }
            flag @ "-e" | flag @ "--editor" => {
                editor = args.next().unwrap_or_else(|| {
                    die!(Codes::ArgumentError, "Expected editor name after '{flag}'")
                });
            }
            _ => {
                if lang.is_empty() {
                    if arg.starts_with('-') {
                        die!(Codes::ArgumentError, "Unknown flag '{}'", arg)
                    }
                    lang = arg;
                } else {
                    die!(Codes::ArgumentError, "Expected exactly one language")
                }
            }
        }
    }
    if lang.is_empty() && list == List::None {
        die!(Codes::ArgumentError, "Expected exactly one language")
    }

    return Args {
        hist,
        list,
        editor,
        compiler_args,
        prog_args,
        runner,
        lang,
    };
}

fn list(kind: List, lang: &str, aliases: &Aliases, runners: &Runners) -> Error<()> {
    match kind {
        List::None => return Ok(()),
        List::Langs => {
            println!("Avaliable languages:\n{}", HLINE);
            for lang in LANGS {
                println!("    {lang}");
            }
        }
        List::Aliases => {
            println!("Avaliable aliases:\n{}", HLINE);
            for (alias, lang) in aliases {
                println!("    {alias} : {lang}");
            }
        }
        List::Runners => {
            print!("Avaliable runners");
            if !lang.is_empty() {
                print!(" for {}", lang);
            }
            println!(":\n{}", HLINE);
            if lang.is_empty() {
                for runner in runners.runners {
                    print!("    {} : ", runner.name);
                    let mut it = runner.supported_langs.iter();
                    print!(
                        "{}",
                        it.next().ok_or_else(|| dieo!(
                            Codes::InternalError,
                            "Runner has no supported languages"
                        ))?
                    );
                    for l in it {
                        print!(", {}", l);
                    }
                    println!("");
                }
            } else {
                for runner in runners.runners {
                    if runner.supported_langs.contains(&lang) {
                        println!("{}", runner.name);
                    }
                }
            }
        }
    }
    Err(Codes::Ok)
}

const INDENT: usize = 4;

fn output_printer(v: &Vec<u8>, indent: usize) {
    let str = String::from_utf8_lossy(v);
    for line in str.split('\n') {
        println!("{:indent$}", line);
    }
}

fn check_status(exe: &[&str], res: &process::Output, code: Codes) -> Error<()> {
    check_status_impl(exe, res, code, None)
}

fn check_status_template(
    exe: &[&str],
    res: &process::Output,
    code: Codes,
    conf: &template::Conf,
) -> Error<()> {
    check_status_impl(exe, res, code, Some(conf))
}

fn check_status_impl(
    exe: &[&str],
    res: &process::Output,
    code: Codes,
    conf: Option<&template::Conf>,
) -> Error<()> {
    if res.status.success() {
        Ok(())
    } else {
        let mut out_name = String::new();

        let exe = exe
            .iter()
            .map(|s| {
                if let Some(conf) = conf {
                    template::sub(s, conf, &mut out_name)
                } else {
                    Ok(s.to_string())
                }
            })
            .fold(Ok(String::new()), |acc, s| {
                if let Ok(s) = s {
                    if s.is_empty() {
                        acc
                    } else {
                        Ok(acc? + &s + " ")
                    }
                } else {
                    s
                }
            })?;
        if let Some(code) = res.status.code() {
            println!("{exe} exited with '{code}'");
        } else {
            println!("{exe} was closed by a signal");
        }
        if res.stdout.len() > 0 {
            print!("stdout:\n\n\n");
            output_printer(&res.stdout, INDENT);
        }
        if res.stderr.len() > 0 {
            print!("\nstderr:\n\n\n");
            output_printer(&res.stderr, INDENT);
        }
        Err(code)
    }
}

fn find_exe<'a>(dep: &'a str) -> Option<&'a str> {
    // https://stackoverflow.com/a/37499032
    if env::var_os("PATH")
        .and_then(|paths| {
            env::split_paths(&paths).find_map(|dir| {
                let full_path = dir.join(&dep);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
        })
        .is_some()
    {
        Some(dep)
    } else {
        None
    }
}

mod template {
    use super::*;

    pub(crate) struct Rep<'a> {
        pub string: &'a str,
        pub is_out: bool,
    }
    impl<'a> Rep<'a> {
        pub(crate) fn new(s: &'a str) -> Self {
            Rep {
                string: s,
                is_out: false,
            }
        }
        pub(crate) fn out(s: &'a str) -> Self {
            Rep {
                string: s,
                is_out: true,
            }
        }
    }
    // NOTE: It used to be possible for this function to produce an error.
    //       Whilst it is no longer possible for it to do so, the signature remains Error<()> so
    //       that in the future an error could be generated if needed.
    fn get_out_name<'a>(
        line: &'a str,
        subbed: &'a str,
        start_idx: usize,
        template_len: usize,
        sub_len: usize,
        out_name: &mut String,
    ) -> Error<()> {
        let out = if let Some(end_idx) = line[(start_idx + template_len)..].find(' ') {
            &subbed[start_idx..end_idx + start_idx + sub_len]
        } else {
            &subbed[start_idx..]
        };
        *out_name = out.to_string();
        Ok(())
    }

    pub(crate) type Conf<'a> = HashMap<&'a str, Rep<'a>>;
    // TODO(dk949): change String to String|&str
    pub(crate) fn sub<'a>(inp: &'a str, conf: &Conf<'a>, out_name: &mut String) -> Error<String> {
        let mut modified: Option<String> = None;
        for (find, replace) in conf {
            if let Some(idx) = inp.find(find) {
                modified = match modified {
                    Some(modified) => {
                        let new_modified = modified.replace(find, replace.string);
                        if replace.is_out {
                            get_out_name(
                                &modified,
                                &new_modified,
                                idx,
                                find.len(),
                                replace.string.len(),
                                out_name,
                            )?;
                        }
                        Some(new_modified)
                    }
                    None => {
                        let new_modified = inp.replace(find, replace.string);
                        if replace.is_out {
                            get_out_name(
                                inp,
                                &new_modified,
                                idx,
                                find.len(),
                                replace.string.len(),
                                out_name,
                            )?;
                        }
                        Some(new_modified)
                    }
                };
            }
        }
        if let Some(modified) = modified {
            Ok(modified)
        } else {
            Ok(inp.to_owned())
        }
    }
}

#[derive(Clone)]
struct Runner {
    name: &'static str,
    extension: &'static str,
    exe_idx: usize,
    exe_deps: &'static [&'static str],
    other_deps: &'static [&'static [&'static str]],
    supported_langs: &'static [&'static str],
    default_for: &'static [&'static str],
    setup: &'static [&'static [&'static str]],
    exe_args_pre: &'static [&'static str],
    exe_args_post: &'static [&'static str],
    teardown: &'static [&'static [&'static str]],
}

enum Exe {
    Str(&'static str),
    Native,
}

impl Exe {
    fn as_str(self: &Self) -> &'static str {
        match self {
            Exe::Str(s) => s,
            Exe::Native => "",
        }
    }
}

impl Runner {
    const NATIVE: usize = std::usize::MAX;
    fn get_exe(self: &Self) -> Error<Exe> {
        if self.exe_idx == Self::NATIVE {
            if !self.exe_args_pre.is_empty() {
                dier!(
                    Codes::InternalError,
                    "Expected no 'pre' args for a native executable"
                )
            }
            Ok(Exe::Native)
        } else {
            Ok(Exe::Str(
                self.exe_deps
                    .get(self.exe_idx)
                    .ok_or_else(|| dieo!(Codes::InternalError, ""))?,
            ))
        }
    }
    fn check_dep_list<'a>(list: &'a [&'a str]) -> Option<usize> {
        let mut idx = 0;
        let dep = list.iter().find_map(|s| {
            idx += 1;
            find_exe(*s)
        });
        dep.map(|_| idx - 1)
    }
    fn check_deps(self: &mut Self) -> Result<(), &'static [&'static str]> {
        self.exe_idx = if self.exe_deps.is_empty() {
            Self::NATIVE
        } else {
            Self::check_dep_list(self.exe_deps).ok_or(self.exe_deps)?
        };
        for deps in self.other_deps {
            Self::check_dep_list(*deps).ok_or(*deps)?;
        }
        Ok(())
    }

    fn run_aux(
        cmds: &[&[&str]],
        copmiler_args: &[String],
        conf: &template::Conf,
        out_name: &mut String,
    ) -> Error<()> {
        if cmds.len() == 0 {
            return Ok(());
        }

        fn exec(cmd: &[&str], conf: &template::Conf, out_name: &mut String) -> Error<()> {
            let mut it = cmd.iter();
            let exe = it
                .next()
                .ok_or_else(|| dieo!(Codes::InternalError, "Unexpected empty command"))?;
            let res = Command::new(template::sub(exe, conf, out_name)?)
                .args(
                    it.map(|s| template::sub(s, conf, out_name))
                        .collect::<Error<Vec<_>>>()?,
                )
                .output()
                .to_code(exe)?;
            check_status_template(cmd, &res, Codes::CodeError, conf)?;
            Ok(())
        }

        let mut cmds_it = cmds.iter().take(cmds.len() - 1);
        while let Some(cmd) = cmds_it.next() {
            exec(cmd, conf, out_name)?;
        }
        let cmd = cmds.last().ok_or_else(|| {
            dieo!(
                Codes::InternalError,
                "Expected to have exactly one element in cmds_it"
            )
        })?;

        exec(
            &[
                *cmd,
                &copmiler_args.iter().map(String::as_str).collect::<Vec<_>>(),
            ]
            .concat(),
            conf,
            out_name,
        )?;

        Ok(())
    }

    fn run_exe(self: &Self, file: &Path, args: &[String]) -> Error<()> {
        let exe = self.get_exe()?;
        let is_native = matches!(exe, Exe::Native);
        let mut cmd = Command::new(if is_native {
            file.to_str_or_die()?
        } else {
            exe.as_str()
        });
        let res = if is_native {
            cmd.args(self.exe_args_post)
        } else {
            cmd.args(self.exe_args_pre.iter())
                .arg(file)
                .args(self.exe_args_post.iter())
        }
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .to_code(if is_native {
            file.to_str_or_die()?
        } else {
            exe.as_str()
        })?;

        check_status(
            &[
                &[exe.as_str()],
                self.exe_args_pre,
                &[&file.to_str_or_die()?],
                self.exe_args_post,
            ]
            .concat(),
            &res,
            Codes::CodeError,
        )?;
        Ok(())
    }

    // TODO(dk949): make it possible to refer to teh main executable and other dependencies through
    //              template strings.
    fn run(
        self: &Self,
        lang: &str,
        file: &Path,
        compiler_args: &[String],
        prog_args: &[String],
    ) -> Error<()> {
        use template::*;
        let out_file = cache_file_name(&env::temp_dir(), lang, "output_file", "");
        let conf = Conf::from([
            ("%INPUT_FILE%", Rep::new(file.to_str_or_die()?)),
            ("%OUTPUT_FILE%", Rep::out(out_file.to_str_or_die()?)),
        ]);
        let mut out_name = String::new();
        Self::run_aux(self.setup, compiler_args, &conf, &mut out_name)?;
        self.run_exe(
            if !out_name.is_empty() {
                Path::new(&out_name)
            } else {
                file
            },
            prog_args,
        )
        .unwrap_or(());
        Self::run_aux(self.teardown, &[], &conf, &mut out_name)?;
        Ok(())
    }
}

struct Runners {
    runners: &'static [Runner],
}

impl Runners {
    fn new() -> Self {
        Runners {
            runners: include!(concat!(env!("OUT_DIR"), "/runners_list")),
        }
    }
    fn get<'a>(self: &Self, name: &'a str, lang: &'a str) -> Option<Error<&'a Runner>> {
        for r in self.runners {
            if r.name == name {
                if r.supported_langs.contains(&lang) {
                    return Some(Ok(r));
                }
            }
        }
        None
    }

    fn runner_for_lang<'a>(self: &Self, lang: &'a str) -> Error<&Runner> {
        for r in self.runners {
            if r.default_for.contains(&lang) {
                return Ok(r);
            }
        }
        Err(Codes::InternalError)
    }

    fn determine<'a>(self: &Self, user_runner: Option<&'a String>, lang: &'a str) -> Error<Runner> {
        let runner = if let Some(runner) = user_runner {
            if let Some(r) = self.get(runner.as_str(), lang) {
                r.or_else(|c| {
                    dier!(
                        c,
                        "Specified runner '{runner}' cannot be used with '{lang}'"
                    )
                })?
            } else {
                dier!(Codes::RunnerError, "Unsupported runner '{runner}'")
            }
        } else {
            self.runner_for_lang(lang)
                .or_else(|c| dier!(c, "Could not find default runner for '{lang}'"))?
        };

        let mut runner = (*runner).clone();
        runner.check_deps().or_else(Self::missing)?;
        Ok(runner)
    }

    fn missing<'a>(deps: &'a [&'a str]) -> Error<()> {
        if deps.is_empty() {
            dier!(
                Codes::InternalError,
                "Empty dependency list has a missing dependency"
            );
        }
        print!("Could not execute due to missing dependencies: ");
        let mut it = deps.iter();
        print!("{}", it.next().unwrap());
        for dep in it {
            print!(" or {}", dep);
        }
        println!("");

        Err(Codes::DependencyError)
    }
}

include!(concat!(env!("OUT_DIR"), "/lang_macro"));

include!(concat!(env!("OUT_DIR"), "/lang_list"));

type Aliases = HashMap<&'static str, &'static str>;
fn aliases() -> Aliases {
    include!(concat!(env!("OUT_DIR"), "/alias_map"))
}

type Snippets = HashMap<&'static str, &'static str>;
fn snippets() -> Snippets {
    include!(concat!(env!("OUT_DIR"), "/snippet_map"))
}
fn get_snippet<'a>(snippets: &'a Snippets, lang: &'a str) -> Error<&'a str> {
    Ok(*snippets.get(lang).ok_or_else(|| {
        dieo!(Codes::InternalError, "Could not find snippet for {lang}");
    })?)
}

fn determine_lang<'a>(args: &'a Args, aliases: &'a Aliases) -> Error<&'a str> {
    if args.lang.is_empty() {
        if args.list == List::None {
            dier!(Codes::InternalError, "Language is empty")
        }
        Ok(args.lang.as_str())
    } else if LANGS.contains(&args.lang.as_str()) {
        Ok(args.lang.as_str())
    } else if let Some(alias) = aliases.get(args.lang.as_str()) {
        Ok(alias)
    } else {
        dier!(Codes::LanguageError, "Unsupported language '{}'", args.lang)
    }
}

trait ToErrorCode<T> {
    fn to_code(self: Self, context: &str) -> Error<T>;
}

impl ToErrorCode<fs::File> for io::Result<fs::File> {
    fn to_code(self: Self, filename: &str) -> Error<fs::File> {
        self.or_else(|e| dier!(Codes::FileError, "Could not create '{filename}': {e}"))
    }
}

impl ToErrorCode<process::Output> for io::Result<process::Output> {
    fn to_code(self: Self, exe: &str) -> Error<process::Output> {
        self.or_else(|e| dier!(Codes::EditorError, "{exe} could not be started: {e}"))
    }
}

impl ToErrorCode<()> for io::Result<()> {
    fn to_code(self: Self, _: &str) -> Error<()> {
        self.or_else(|e| dier!(Codes::FileError, "Could not create a temporary file: {e}"))
    }
}

trait ToStrOrDie {
    fn to_str_or_die(self: &Self) -> Error<&str>;
}

impl ToStrOrDie for Path {
    fn to_str_or_die(self: &Self) -> Error<&str> {
        self.to_str()
            .ok_or_else(|| dieo!(Codes::InternalError, "Could not convert path to str"))
    }
}

const CACHE_DIR: &str = "ruc_cache";

fn add_prefix(lang: &str, runner: &str) -> String {
    let mut out = String::from(lang);
    out.extend(['_'].iter());
    out.extend(runner.chars());

    return out;
}

fn cache_file_name(dir: &PathBuf, lang: &str, runner: &str, extension: &str) -> PathBuf {
    let mut file = dir.join(add_prefix(lang, runner));
    file.set_extension(extension);
    return file;
}

fn cache_file_path(lang: &str, runner: &str, extension: &str) -> Error<PathBuf> {
    let cache = dirs::cache_dir()
        .ok_or_else(|| dieo!(Codes::InternalError, "Could not locate cache directory"))?;
    let cache_dir = cache.join(CACHE_DIR);
    fs::create_dir_all(&cache_dir).or_else(|e| {
        dier!(
            Codes::FileError,
            "Could not create cache direcotry '{}': {e}",
            cache_dir.to_string_lossy()
        )
    })?;
    Ok(cache_file_name(&cache_dir, lang, runner, extension))
}

static mut TEMP_FILE: Option<PathBuf> = None;

fn setup_hist<'a>(
    hist: Hist,
    lang: &'a str,
    runner: &str,
    extension: &str,
    snippet: &'a str,
) -> Error<PathBuf> {
    let (mut file, path) = match hist {
        Hist::Temp => {
            let path = cache_file_name(&env::temp_dir(), lang, runner, extension);
            let file = fs::File::create(&path).to_code(&path.to_string_lossy())?;
            // This is fine
            unsafe {
                TEMP_FILE = Some(path.clone());
            }
            (file, path)
        }
        Hist::Use | Hist::New => {
            let cache_path = cache_file_path(lang, runner, extension)?;
            if hist == Hist::Use && cache_path.exists() {
                return if cache_path.is_file() {
                    Ok(cache_path)
                } else {
                    dier!(
                        Codes::FileError,
                        "Cannot open history file '{}'. Path exists and is not a regular file",
                        cache_path.to_string_lossy()
                    )
                };
            } else {
                let file = fs::File::create(&cache_path).to_code(&cache_path.to_string_lossy())?;
                (file, cache_path)
            }
        }
    };

    writeln!(file, "{}", snippet)
        .to_code(format!("Could not write to file '{}'", path.to_string_lossy()).as_str())?;
    return Ok(path);
}

fn cleanup_temp() {
    if let Some(temp) = unsafe { &TEMP_FILE } {
        fs::remove_file(temp).unwrap_or_else(|e| {
            println!(
                "Warning: could not remove temporary file {}: {e}",
                temp.to_string_lossy()
            )
        });
    }
}

fn editor(user_editor: &String) -> Error<String> {
    let editor = if user_editor.is_empty() {
        env::var_os("EDITOR")
            .ok_or_else(|| dieo!(
                    Codes::EditorError,
                    "Could not determine which editor to use, try setting the EDITOR environment variable or using the -e flag."
                    )
                )?
            .to_string_lossy()
            .to_string()
    } else {
        user_editor.clone()
    };

    find_exe(&editor)
        .ok_or_else(|| dieo!(Codes::EditorError, "Editor '{editor}' is not in PATH",))?;

    Ok(editor)
}

fn run_editor(editor: &str, file: &str) -> Error<()> {
    let res = Command::new(editor)
        .arg(file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .to_code(editor)?;

    check_status(&[editor], &res, Codes::EditorError)?;
    println!("Editor exited successfully\n{}\n", HLINE);
    Ok(())
}

fn program(args: Args) -> Error<()> {
    let aliases = aliases();
    let runners = Runners::new();
    let snippets = snippets();
    let lang = determine_lang(&args, &aliases)?;

    list(args.list, lang, &aliases, &runners)?;

    let runner = runners.determine(args.runner.as_ref(), lang)?;
    let snippet = get_snippet(&snippets, lang)?;
    let hist_path = setup_hist(args.hist, lang, runner.name, runner.extension, snippet)?;
    let editor = editor(&args.editor)?;

    run_editor(&editor, &hist_path.as_os_str().to_string_lossy())?;
    runner.run(&lang, &hist_path, &args.compiler_args, &args.prog_args)?;
    Ok(())
}

fn main() {
    let exit_code = program(parse_args()).map_or_else(identity, |_| Codes::Ok);
    cleanup_temp();
    exit(exit_code)
}
