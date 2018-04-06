#!/bin/sh
#
# A script to bump the version number on all Cargo.toml files etc in
# an atomic fashion.

if [ "$1" == "" ]; then
    echo "Usage: version.sh <new-version-number>"
    exit 1
fi

VERSION=$(
    ls lalrpop*/Cargo.toml | \
        xargs grep "# LALRPOP$" | \
        perl -p -e 's/.*version = "([0-9.]+)" # LALRPOP$/$1/' |
        sort |
        uniq)

if [ $(echo $VERSION | wc -w) != 1 ]; then
    echo "Error: inconsistent versions detected across Cargo.toml files!"
    echo "$VERSION"
    exit 1
fi

echo "Found consistent version $VERSION"

perl -p -i -e 's/version *= *"[0-9.]+" # LALRPOP/version = "'$1'" # LALRPOP/' \
     $(ls lalrpop*/Cargo.toml)

perl -p -i -e 's/version *= *"'$VERSION'"/version = "'$1'"/' \
     $(find doc -name Cargo.toml)

perl -p -i -e 's/^lalrpop([\-a-z]*) *= *"[0-9.]+"/lalrpop\1 = "'$1'"/' \
    doc/src/quick_start_guide.md doc/src/tutorial/001_adding_lalrpop.md
