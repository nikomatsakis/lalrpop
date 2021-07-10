#!/bin/bash
#
# This script runs on Actions to prep the CI as needed.

MDBOOK_VER=0.4.7

if command -v mdbook >/dev/null 2>&1; then
    echo "mdbook already installed at $(command -v mdbook)"
else
    echo "installing mdbook"
    mkdir mdbook
    curl -Lf https://github.com/rust-lang/mdBook/releases/download/v${MDBOOK_VER}/mdbook-v${MDBOOK_VER}-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=./mdbook
    echo `pwd`/mdbook >> $GITHUB_PATH
fi

