use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=src/snippets/");
    println!("cargo:rerun-if-changed=src/runners/");
    println!("cargo:rerun-if-changed=src/aliases");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let snippets_dir = Path::new(&out_dir).join("snippets");
    let runners_dir = Path::new(&out_dir).join("runners");

    let mut lang_macro = fs::File::create(Path::new(&out_dir).join("lang_macro")).unwrap();
    let mut lang_list = fs::File::create(Path::new(&out_dir).join("lang_list")).unwrap();
    let mut snippet_map = fs::File::create(Path::new(&out_dir).join("snippet_map")).unwrap();
    let mut runners_list = fs::File::create(Path::new(&out_dir).join("runners_list")).unwrap();
    let mut alias_map = fs::File::create(Path::new(&out_dir).join("alias_map")).unwrap();

    fs::create_dir_all(&snippets_dir).unwrap();
    fs::create_dir_all(&runners_dir).unwrap();

    lang_macro.write_all(b"macro_rules! LANGS {\n").unwrap();
    lang_list
        .write_all(b"const LANGS: [&str; LANGS!(len)] = [\n")
        .unwrap();
    snippet_map.write_all(b"HashMap::from([\n").unwrap();

    let snippets = fs::read_dir("src/snippets/")
        .unwrap()
        .map(|de| de.unwrap().path())
        .collect::<Vec<_>>();

    for (i, snippet) in snippets.iter().enumerate() {
        let snippet_name = snippet.file_name().unwrap().to_str().unwrap();
        let new_snippet = &snippets_dir
            .parent()
            .unwrap()
            .join(snippet.iter().skip(1).collect::<PathBuf>());
        fs::copy(snippet, new_snippet).unwrap();

        writeln!(lang_macro, "({i}) => {{ \"{snippet_name}\" }};").unwrap();
        writeln!(
            lang_macro,
            "(check, \"{snippet_name}\") => {{ \"{snippet_name}\" }};"
        )
        .unwrap();

        writeln!(lang_list, "LANGS![{i}], ").unwrap();

        writeln!(
            snippet_map,
            "(LANGS![{i}], include_str!(concat!(\"snippets/\", LANGS![{i}]))),"
        )
        .unwrap();
    }

    writeln!(lang_macro, "(len) => {{ {} }}", snippets.len()).unwrap();
    lang_macro.write_all(b"}\n").unwrap();

    lang_list.write_all(b"];\n").unwrap();

    snippet_map.write_all(b"])\n").unwrap();

    runners_list.write_all(b"&[\n").unwrap();

    let runners = fs::read_dir("src/runners/")
        .unwrap()
        .map(|de| de.unwrap().path())
        .collect::<Vec<_>>();

    for runner in runners.iter() {
        let runner_name = runner.iter().skip(1).collect::<PathBuf>();
        let runner_name = runner_name.to_str().unwrap();
        let new_runner = &runners_dir
            .parent()
            .unwrap()
            .join(runner.iter().skip(1).collect::<PathBuf>());
        fs::copy(runner, new_runner).unwrap();

        writeln!(runners_list, "include!(\"{runner_name}\"), ",).unwrap();
    }

    runners_list.write_all(b"]\n").unwrap();

    alias_map.write_all(b"HashMap::from([\n").unwrap();
    for alias in String::from_utf8(fs::read("src/aliases").unwrap())
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.split(':').map(|elem| elem.trim()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
    {
        eprintln!("alias = {alias:?}");
        writeln!(
            alias_map,
            "(\"{}\", LANGS!(check, \"{}\")),",
            alias[0], alias[1]
        )
        .unwrap();
    }
    alias_map.write_all(b"])\n").unwrap();
}
