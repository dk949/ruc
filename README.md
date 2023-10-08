# Ruc

Short for run code

## How to use?

* `make install` to install.
  * configure install directory in `config.mk`
  * or by using the `DESTDIR` and `PREFIX` environment variables
* `ruc LANG` will open the system editor with a quick-start code snippet (where
  applicable). Write code in the specified `LANG`. Closing the editor will
  execute the code.
* `ruc --help` for other options

### History

* by default the edited file is cached
  * so that using `ruc LANG` with the same language will bring back the same
    file
* `ruc LANG -n` will clear old file and just use the default snippet
* `ruc LANG -t` will not use the history file for the current invocation and
  will not cache current invocation
  * the next `ruc LANG` will use the previous cache file

## Why?

* For quickly testing something without needing to set up a whole dev
  environment.
* Easier to use then a REPL, but hopefully just as immediate.
* Some languages don't have REPLs

## Current and future language support

* :ballot_box_with_check: : Done!
* :hammer: : Still working on it
* :man_shrugging: : not sure if will work on
* :no_entry: : Not feasible/applicable

Most languages came from
[here](https://madnight.github.io/githut/#/pull_requests/2021/3).

| Status                  | Langauge        | Status   | Langauge   | Status          | Language  | Status     | Language          |
|-------------------------|-----------------|----------|------------|-----------------|-----------|------------|-------------------|
| :ballot_box_with_check: | Asm (nasm/yasm) | :hammer: | Clojure    | :man_shrugging: | Coq       | :no_entry: | Emacs Lisp        |
| :ballot_box_with_check: | Bash            | :hammer: | Dart       | :man_shrugging: | DM        | :no_entry: | F#                |
| :ballot_box_with_check: | C               | :hammer: | Elm        | :man_shrugging: | Elixir    | :no_entry: | Jsonnet           |
| :ballot_box_with_check: | C#              | :hammer: | Groovy     | :man_shrugging: | Erlang    | :no_entry: | MATLAB            |
| :ballot_box_with_check: | C++             | :hammer: | Kotlin     | :man_shrugging: | Julia     | :no_entry: | NASL              |
| :ballot_box_with_check: | Cmake           | :hammer: | PowerShell | :man_shrugging: | Smalltalk | :no_entry: | Nix               |
| :ballot_box_with_check: | CoffeeScript    | :hammer: | R          | :man_shrugging: | Crystal   | :no_entry: | Objective-C       |
| :ballot_box_with_check: | D               | :hammer: | Vala       | :man_shrugging: | APL       | :no_entry: | Objective-C++     |
| :ballot_box_with_check: | Dash            | :hammer: | V          |                 |           | :no_entry: | Puppet            |
| :ballot_box_with_check: | Fortran         |          |            |                 |           | :no_entry: | Swift             |
| :ballot_box_with_check: | Go              |          |            |                 |           | :no_entry: | SystemVerilog     |
| :ballot_box_with_check: | Haskell         |          |            |                 |           | :no_entry: | Visual Basic .NET |
| :ballot_box_with_check: | Java            |          |            |                 |           | :no_entry: | TSQL              |
| :ballot_box_with_check: | JavaScript      |          |            |                 |           | :no_entry: | Vim script        |
| :ballot_box_with_check: | Lua             |          |            |                 |           | :no_entry: |                   |
| :ballot_box_with_check: | Ocaml           |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Perl            |          |            |                 |           |            |                   |
| :ballot_box_with_check: | PHP             |          |            |                 |           |            |                   |
| :ballot_box_with_check: | PureScript      |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Python          |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Ruby            |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Rust            |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Shell           |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Scala           |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Scheme          |          |            |                 |           |            |                   |
| :ballot_box_with_check: | TypeScript      |          |            |                 |           |            |                   |
| :ballot_box_with_check: | WebAssembly     |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Zig             |          |            |                 |           |            |                   |
| :ballot_box_with_check: | Zsh             |          |            |                 |           |            |                   |


## Development

You will need the [rust toolchain](https://www.rust-lang.org/tools/install) to
compile the project.

Running `cargo build` should build everything.

This project uses a pre-commit hook in order to check that the project version
in `Cargo.toml` has been updated, please run the following to enable the hook

```sh
git config --local core.hooksPath .githooks/
```

The simplest way to add a language is by running the `scripts/add` script or by
adding/modifying file in `src/runners/` and `src/snippets/`.


### Runner

A runner is a rust struct with the following structure (see notes below for
explanation):

```rust
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
```

* `name`: Name of this runner.
    * Does not have to be the same as language name.
    * This name will be used with the `-r` flag to select this runner.
* `extension`: File extension the runner operates on.
    * Without the leading period.
* `exe_idx`: Reserved, set to 0.
* `exe_deps`: List of executables the runner can use.
    * Will check the presence of these in order and use the first one found on
      the system. Or raise an error if none were found.
    * Leave this empty if running a native executable.
* `other_deps` List of lists of other dependencies required by the runner.
    * Like `exe_deps` each inner list is a list of alternatives that can be used.
    * E.g. `&[&["foo", "bar"], &["baz"]]` will look for `(foo or bar) and baz`.
    * For compiled languages, specify the compiler dependencies here.
* `supported_langs`: List of languages this runner supports.
    * The languages must have a corresponding `snippets` file
      ([see below](#snippets)).
* `default_for`: List of languages this is the default runner for.
    * The languages must have a corresponding `snippets` file
      ([see below](#snippets)).
* `setup`: List of commands to run before invoking the executable.
    * Note that a "command" does not refer to a shell command, but an executable
      name with a list of arguments.
    * E.g.
        * `&[&["echo", "hello"], &["echo", "world"]]` will work as it just
        invokes the `echo` executable.
        * `&[&["echo", "hello", ">", "some_file"]]` will not work (it will print
        `hello > some_file`).
    * If you must use a shell, use `&[&["sh", "-c", "echo 'hello' > some_file"]]`.
    * See notes on [templating](#templating) below.
* `exe_args_pre`: List of arguments passed to the interpreter before passing the
                  file to execute.
   * For native executables leave this empty.
* `exe_args_post`: List of arguments passed to the interpreter or the native
                   executable after the file name.
* `teardown`: Like setup, but runs after the executable has exited.
    * For compiled languages use this to clean up any files created during
      `setup`.
    * Note: if `setup` succeeds,  `teardown` will run, even if the executable
      fails.

### Templating

When using compiled languages is will likely be necessary to refer to the name
of the source file and the compiled executable. The template strings
`%INPUT_FILE%` and `%OUTPUT_FILE%` may be used in both `setup` and `teardown`
(but not `exe_args_pre` or `exe_args_post`).

If `%OUTPUT_FILE%` is referred to, it will be used instead of the `%INPUT_FILE%`
after `setup`.

These strings will be substituted regardless of any white space or punctuation.

It is safe (and advised) to delete the `%OUTPUT_FILE%` in `teardown` if it was
created in `setup`.

_Note:_ Other template strings (e.g. `%SOME_TEMPLATE_STRING%`) will not get
replaced or raise a warning.

### Snippets

A snippet file has to exist for each language referred to by a runner. A snippet
file is a file who's name is the same as the language, containing any
boilerplate code which may be required for a program in this language.

A snippet may be empty.
