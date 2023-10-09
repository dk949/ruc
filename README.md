# Ruc

Short for run code

## Table of contents

* [How to use?](#how-to-use?)
  * [History](#history)
  * [Choosing a runner](#choosing-a-runner)
* [Why?](#why?)
* [Development](#development)
  * [Runner](#runner)
  * [Templating](#templating)
  * [Snippets](#snippets)
  * [Aliases](#aliases)
* [Language support](#language-support)

## How to use?

* `cargo install --path .` to install.
  * By default will install in `$HOME/.local/bin`.
  * Override prefix with `--root PREFIX`
* `ruc LANG` will open the system editor with a quick-start code snippet (where
  applicable). Write code in the specified `LANG`. Closing the editor will
  execute the code.
* `ruc --help` for other options
* You can also check the [Language support](#language-support) section for
  a list of supported languages.

### History

* by default the edited file is cached
  * so that using `ruc LANG` with the same language will bring back the same
    file
* `ruc LANG -n` will clear old file and just use the default snippet
* `ruc LANG -t` will not use the history file for the current invocation and
  will not cache current invocation
  * the next `ruc LANG` will use the previous cache file

### Choosing a runner

* Some languages may have multiple runners
    * E.g. using different compilers or interpreters.
* You can check which runners are available with `--list-runners` and select a
  runner with `-r`.

## Why?

* For quickly testing something without needing to set up a whole dev
  environment.
* Easier to use then a REPL, but hopefully just as immediate.
* Some languages don't have REPLs

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

If the `%OUTPUT_FILE%` string is followed by a non-whitespace suffix, this
suffix is also appended to the substituted string. E.g. if `%OUTPUT_FILE%` is
substituted for `name_of_output_file`, then `%OUTPUT_FILE%.jar` will become
`name_of_output_file.jar`.

_Note:_ The last suffix used will be the one passed to the executable, i.e.
intermediate files can be generated with other suffixes.

It is safe (and advised) to delete the `%OUTPUT_FILE%` in `teardown` if it was
created in `setup`.

_Note:_ Other template strings (e.g. `%SOME_TEMPLATE_STRING%`) will not get
replaced or raise a warning.

### Snippets

A snippet file has to exist for each language referred to by a runner. A snippet
file is a file who's name is the same as the language, containing any
boilerplate code which may be required for a program in this language.

A snippet may be empty.


### Aliases

The `src/aliases` file contains a list of aliases for languages in the format
`alias : language`. These aliases can be used in pace of the language name. E.g.
adding `js : javascript` makes `ruc js` equivalent to `ruc javascript`.

## Language support

* :ballot_box_with_check: : Done!
* :hammer: : Still working on it
* :man_shrugging: : Possibly planned for the future

Most languages came from
[here](https://madnight.github.io/githut/#/pull_requests/2023/2).

| Language                 | Status                  |
| ------------------------ | ----------------------- |
| Ada                      | :hammer:                |
| Agda                     | :man_shrugging:         |
| Assembly (GAS)           | :ballot_box_with_check: |
| Assembly (fasm)          | :hammer:                |
| Assembly (nasm/yasm)     | :ballot_box_with_check: |
| Awk                      | :ballot_box_with_check: |
| Batchfile                | :man_shrugging:         |
| Boo                      | :man_shrugging:         |
| C                        | :ballot_box_with_check: |
| C#                       | :ballot_box_with_check: |
| C++                      | :ballot_box_with_check: |
| CMake                    | :ballot_box_with_check: |
| COBOL                    | :man_shrugging:         |
| Clojure                  | :ballot_box_with_check: |
| CoffeeScript             | :ballot_box_with_check: |
| Common Lisp              | :ballot_box_with_check: |
| Coq                      | :man_shrugging:         |
| Crystal                  | :man_shrugging:         |
| Cuda                     | :man_shrugging:         |
| Cython                   | :hammer:                |
| D                        | :ballot_box_with_check: |
| Dart                     | :hammer:                |
| Eiffel                   | :man_shrugging:         |
| Elixir                   | :hammer:                |
| Elm                      | :man_shrugging:         |
| Erlang                   | :man_shrugging:         |
| F#                       | :man_shrugging:         |
| F*                       | :man_shrugging:         |
| Fortran                  | :ballot_box_with_check: |
| Go                       | :ballot_box_with_check: |
| Groovy                   | :ballot_box_with_check: |
| Hack                     | :man_shrugging:         |
| Haskell                  | :ballot_box_with_check: |
| Haxe                     | :man_shrugging:         |
| Idris                    | :man_shrugging:         |
| J                        | :ballot_box_with_check: |
| Java                     | :ballot_box_with_check: |
| JavaScript               | :ballot_box_with_check: |
| Julia                    | :ballot_box_with_check: |
| Kotlin                   | :ballot_box_with_check: |
| Kotlinscript             | :ballot_box_with_check: |
| LLVM                     | :hammer:                |
| Lua                      | :ballot_box_with_check: |
| MLIR                     | :hammer:                |
| Makefile                 | :man_shrugging:         |
| NewLisp                  | :man_shrugging:         |
| Nim                      | :hammer:                |
| Nix                      | :hammer:                |
| OCaml                    | :ballot_box_with_check: |
| Objective-C              | :man_shrugging:         |
| PHP                      | :ballot_box_with_check: |
| Perl                     | :ballot_box_with_check: |
| PowerShell               | :ballot_box_with_check: |
| Prolog                   | :man_shrugging:         |
| PureScript               | :man_shrugging:         |
| Python                   | :ballot_box_with_check: |
| R                        | :ballot_box_with_check: |
| Racket                   | :man_shrugging:         |
| Reason                   | :man_shrugging:         |
| Ruby                     | :ballot_box_with_check: |
| Rust                     | :ballot_box_with_check: |
| Scala2                   | :ballot_box_with_check: |
| Scala3                   | :ballot_box_with_check: |
| Scheme                   | :ballot_box_with_check: |
| Shell                    | :ballot_box_with_check: |
| Smalltalk                | :man_shrugging:         |
| Swift                    | :hammer:                |
| Tcl                      | :man_shrugging:         |
| TypeScript               | :ballot_box_with_check: |
| Vala                     | :man_shrugging:         |
| Visual Basic             | :hammer:                |
| Vue                      | :man_shrugging:         |
| WebAssembly              | :hammer:                |
| Zig                      | :ballot_box_with_check: |

