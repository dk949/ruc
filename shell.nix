# This is not made to be 100% reproducible, just to make testing easier
{ pkgs ? import <nixpkgs> {}, editor ? ""  } :

pkgs.mkShell {

    buildInputs = [
        pkgs.bash
        pkgs.clojure
        pkgs.coffeescript
        pkgs.dash
        pkgs.julia
        pkgs.lua
        pkgs.nodejs
        pkgs.perl
        pkgs.php
        pkgs.powershell
        pkgs.python3
        pkgs.ruby
        pkgs.zsh
        pkgs.R

        pkgs.vim
        pkgs.nano
        pkgs.gedit
    ];

    shellHook = ''
        [ -n "$RUC_EDITOR" ] && {
            echo Using "$RUC_EDITOR" as the editor
            export EDITOR="$RUC_EDITOR"
        }
    '';

    RUC_EDITOR = editor;
}
