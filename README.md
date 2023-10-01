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
