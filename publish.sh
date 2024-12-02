#!/bin/bash

PROJECTS=(
    float-pigment-consistent-bincode
    float-pigment-mlp
    float-pigment-css-macro
    float-pigment-css
    float-pigment-layout
    float-pigment-forest-macro
    float-pigment-forest
    float-pigment
)

if [ "$1" == "" ]; then
    echo "Missing version. Usage: $0 [VERSION]"
    exit -1
fi

VERSION="$1"

# run tests
if cargo test; then
    echo 'Cargo test done.'
else
    echo 'Cargo test failed! Abort.'
    exit -1
fi

# update C++ headers
if cargo run --bin float_pigment_cpp_binding_gen_tool; then
    echo 'float_pigment_cpp_binding_gen_tool done.'
else
    echo 'float_pigment_cpp_binding_gen_tool failed! Abort.'
    exit -1
fi

# run strict clippy
if cargo clippy -- -D warnings; then
    echo 'Clippy check done.'
else
    echo 'Clippy check failed! Abort.'
    exit -1
fi
if cargo clippy --no-default-features -- -D warnings; then
    echo 'Clippy check (no-default-features) done.'
else
    echo 'Clippy check (no-default-features) failed! Abort.'
    exit -1
fi
if cargo clippy --all-features -- -D warnings; then
    echo 'Clippy check (all-features) done.'
else
    echo 'Clippy check (all-features) failed! Abort.'
    exit -1
fi

# run fmt (may change files and cause next git steps failed)
if cargo fmt; then
    echo 'Cargo fmt done.'
else
    echo 'Cargo fmt failed! Abort.'
    exit -1
fi

# git operations
if test -z '$(git status --porcelain)'; then
    echo 'Git status OK.'

    # update version fields in cargo.toml
    if sed -i '' -E "s/version = \"[^\"]+\"/version = \"${VERSION}\"/" Cargo.toml; then
        echo 'Modified versions in Cargo.toml.'
    else
        echo 'Failed to modify version in Cargo.toml! Abort.'
        exit -1
    fi

    # generate a new commit and tag it
    if git add Cargo.toml && git commit -m "chore: update version to publish"; then
        echo 'Generated a new version commit.'
    else
        echo 'Failed to commit! Abort.'
        exit -1
    fi
    if git tag "v${VERSION}"; then
        echo 'Git tag done.'
    else
        echo 'Git tag failed! Abort.'
        exit -1
    fi

    # push to origin
    if git push && git push --tags; then
        echo 'Git tag pushed.'
    else
        echo 'Git tag push failed! Abort.'
        exit -1
    fi
else
    echo 'Git working tree is not clean! Abort.'
    exit -1
fi

# cargo publish
echo "Ready to publish version ${VERSION}."
for PROJECT in "${PROJECTS[@]}"; do
    echo ""
    echo "Publishing ${PROJECT}..."
    cargo publish --no-verify -p "${PROJECT}"
done
