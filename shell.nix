# This is not made to be 100% reproducible, just to make testing easier
{ pkgs ? import <nixpkgs> {}, editor ? ""  } :

pkgs.mkShell {

    buildInputs = [

        pkgs.R
        pkgs.bash
        pkgs.clang
        pkgs.clisp
        pkgs.clojure
        pkgs.cmakeMinimal
        pkgs.coffeescript
        pkgs.dash
        pkgs.dmd
        pkgs.dotty
        pkgs.gawk
        pkgs.gcc
        pkgs.gfortran
        pkgs.ghc
        pkgs.go
        pkgs.groovy
        pkgs.guile
        pkgs.j
        pkgs.jre_minimal
        pkgs.julia
        pkgs.kotlin
        pkgs.ldc
        pkgs.lua
        pkgs.nasm
        pkgs.nodePackages.ts-node
        pkgs.nodejs
        pkgs.ocaml
        pkgs.perl
        pkgs.php
        pkgs.powershell
        pkgs.python3
        pkgs.ruby
        pkgs.rustc
        pkgs.scala
        pkgs.yasm
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
