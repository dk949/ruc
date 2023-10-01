use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::{env, fs};
// HashMap::from([
//     (LANGS![0], include_str!(concat!("snippets/", LANGS![0]))),
//     (LANGS![1], include_str!(concat!("snippets/", LANGS![1]))),
//     (LANGS![2], include_str!(concat!("snippets/", LANGS![2]))),
//     (LANGS![3], include_str!(concat!("snippets/", LANGS![3]))),
//     (LANGS![4], include_str!(concat!("snippets/", LANGS![4]))),
//     (LANGS![5], include_str!(concat!("snippets/", LANGS![5]))),
//     (LANGS![6], include_str!(concat!("snippets/", LANGS![6]))),
//     (LANGS![7], include_str!(concat!("snippets/", LANGS![7]))),
//     (LANGS![8], include_str!(concat!("snippets/", LANGS![8]))),
//     (LANGS![9], include_str!(concat!("snippets/", LANGS![9]))),
//     (LANGS![10], include_str!(concat!("snippets/", LANGS![10]))),
//     (LANGS![11], include_str!(concat!("snippets/", LANGS![11]))),
//     (LANGS![12], include_str!(concat!("snippets/", LANGS![12]))),
//     (LANGS![13], include_str!(concat!("snippets/", LANGS![13]))),
//     (LANGS![14], include_str!(concat!("snippets/", LANGS![14]))),
//     (LANGS![15], include_str!(concat!("snippets/", LANGS![15]))),
//     (LANGS![16], include_str!(concat!("snippets/", LANGS![16]))),
//     (LANGS![17], include_str!(concat!("snippets/", LANGS![17]))),
//     (LANGS![18], include_str!(concat!("snippets/", LANGS![18]))),
//     (LANGS![19], include_str!(concat!("snippets/", LANGS![19]))),
//     (LANGS![20], include_str!(concat!("snippets/", LANGS![20]))),
//     (LANGS![21], include_str!(concat!("snippets/", LANGS![21]))),
//     (LANGS![22], include_str!(concat!("snippets/", LANGS![22]))),
//     (LANGS![23], include_str!(concat!("snippets/", LANGS![23]))),
//     (LANGS![24], include_str!(concat!("snippets/", LANGS![24]))),
//     (LANGS![25], include_str!(concat!("snippets/", LANGS![25]))),
// ])
//

fn main() {
    println!("cargo:rerun-if-changed=src/snippets/");
    println!("cargo:rerun-if-changed=src/runners/");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let snippets_dir = Path::new(&out_dir).join("snippets");
    let runners_dir = Path::new(&out_dir).join("runners");
    let mut lang_macro = fs::File::create(Path::new(&out_dir).join("lang_macro")).unwrap();
    let mut lang_list = fs::File::create(Path::new(&out_dir).join("lang_list")).unwrap();
    let mut snippet_map = fs::File::create(Path::new(&out_dir).join("snippet_map")).unwrap();
    let mut runners_list = fs::File::create(Path::new(&out_dir).join("runners_list")).unwrap();

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

        // include!("runners/python"),
        writeln!(runners_list, "include!(\"{runner_name}\"), ",).unwrap();
    }

    runners_list.write_all(b"]\n").unwrap();
}
