# This is not made to be 100% reproducible, just to make testing easier
{ pkgs ? import <nixpkgs> {}, editor ? ""  } :


pkgs.mkShell {

    buildInputs = [

        pkgs.R
            pkgs.bash
            pkgs.clojure
            pkgs.dash
            pkgs.lua
            pkgs.nodejs
            pkgs.perl
            pkgs.php
            pkgs.powershell
            pkgs.python3
            pkgs.ruby
            pkgs.zsh

            pkgs.vim
            pkgs.nano
            pkgs.gedit
    ];

    EDITOR = editor

}
