# This is not made to be 100% reproducible, just to make testing easier
{ pkgs ? import <nixpkgs> {}, editor ? ""  } :

pkgs.mkShell {

    buildInputs = [

        pkgs.R
        pkgs.bash
        pkgs.clisp
        pkgs.clojure
        pkgs.coffeescript
        pkgs.dash
        pkgs.dmd
        pkgs.dotty
        pkgs.gawk
        pkgs.ghc
        pkgs.go
        pkgs.guile
        pkgs.jre_minimal
        pkgs.julia
        pkgs.ldc
        pkgs.lua
        pkgs.nodejs
        pkgs.ocaml
        pkgs.perl
        pkgs.php
        pkgs.powershell
        pkgs.python3
        pkgs.ruby
        pkgs.scala
        pkgs.zig
        pkgs.zsh

        pkgs.vim
        pkgs.nano
        pkgs.gedit
    ];

    shellHook = ''
        [ -n "$RUC_EDITOR" ] && {
            echo Using "$RUC_EDITOR" as the editor
            export EDITOR="$RUC_EDITOR"
        }
        PATH="$PWD/scripts/:$PATH"
    '';

    RUC_EDITOR = editor;
}
