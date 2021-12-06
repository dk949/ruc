# Runc

Short for run code

## How to use?

* `runc LANG` will open the system editor with a quick-start code snippet (where
  applicable). Write code in the specified `LANG`. Closing the editor will
  execute the code.
* `runc -l` will list all available languages
* `runc -a` will list all available aliases, i.e. alternative names which can be
  used as `LANG`

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

| Status                  | Langauge   | Status   | Langauge     | Status          | Language  | Status     | Language          |
|-------------------------|------------|----------|--------------|-----------------|-----------|------------|-------------------|
| :ballot_box_with_check: | C          | :hammer: | Clojure      | :man_shrugging: | Coq       | :no_entry: | Emacs Lisp        |
| :ballot_box_with_check: | C#         | :hammer: | CoffeeScript | :man_shrugging: | DM        | :no_entry: | F#                |
| :ballot_box_with_check: | C++        | :hammer: | Dart         | :man_shrugging: | Elixir    | :no_entry: | Jsonnet           |
| :ballot_box_with_check: | D          | :hammer: | Elm          | :man_shrugging: | Erlang    | :no_entry: | MATLAB            |
| :ballot_box_with_check: | Go         | :hammer: | Fortran      | :man_shrugging: | Julia     | :no_entry: | NASL              |
| :ballot_box_with_check: | Haskell    | :hammer: | Groovy       | :man_shrugging: | Smalltalk | :no_entry: | Nix               |
| :ballot_box_with_check: | Java       | :hammer: | Kotlin       | :man_shrugging: | Crystal   | :no_entry: | Objective-C       |
| :ballot_box_with_check: | JavaScript | :hammer: | OCaml        |                 |           | :no_entry: | Objective-C++     |
| :ballot_box_with_check: | Lua        | :hammer: | PHP          |                 |           | :no_entry: | Puppet            |
| :ballot_box_with_check: | Perl       | :hammer: | PowerShell   |                 |           | :no_entry: | Roff              |
| :ballot_box_with_check: | Python     | :hammer: | PureScript   |                 |           | :no_entry: | Swift             |
| :ballot_box_with_check: | Ruby       | :hammer: | R            |                 |           | :no_entry: | SystemVerilog     |
| :ballot_box_with_check: | Rust       | :hammer: | Scala        |                 |           | :no_entry: | Visual Basic .NET |
| :ballot_box_with_check: | Shell      | :hammer: | Vala         |                 |           | :no_entry: | TSQL              |
| :ballot_box_with_check: | TypeScript | :hammer: | Vim script   |                 |           |            |                   |
|                         |            | :hammer: | WebAssembly  |                 |           |            |                   |