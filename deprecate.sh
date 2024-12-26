#!/bin/bash

if [ "$1" == "" ]; then
    echo "Missing version. Usage: $0 [VERSION]"
    exit -1
fi

VERSION="$1"
PROJECTS=$(egrep '^[ \t]*"(.+)",$' Cargo.toml | sed -E 's/^[ \t]*"(.+)",$/\1/g')

# cargo yank
echo "Ready to deprecate version ${VERSION}."
for PROJECT in $PROJECTS; do
    echo ""
    echo "Deprecating ${PROJECT}..."
    cargo yank --version "${VERSION}" "${PROJECT}"
done

# npm deprecate
npm deprecate "float-pigment-css@${VERSION}" --registry https://registry.npmjs.org -m "sync with cargo yank"
