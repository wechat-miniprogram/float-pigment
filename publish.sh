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

if cargo clippy -- -D warnings; then
    echo 'Clippy check done.'
else
    echo 'Clippy check failed! Abort.'
    exit -1
fi

if cargo clippy --no-default-features; then
    echo 'Clippy check (no-default-features) done.'
else
    echo 'Clippy check (no-default-features) failed! Abort.'
    exit -1
fi

if cargo clippy --all-features; then
    echo 'Clippy check (all-features) done.'
else
    echo 'Clippy check (all-features) failed! Abort.'
    exit -1
fi

if cargo fmt; then
    echo 'Cargo fmt done.'
else
    echo 'Cargo fmt failed! Abort.'
    exit -1
fi

if test -z '$(git status --porcelain)'; then
    echo 'Git status OK.'
else
    echo 'Git working tree is not clean! Abort.'
    exit -1
fi

echo 'Ready to publish.'

for PROJECT in "${PROJECTS[@]}"; do
    echo ""
    echo "Publishing ${PROJECT}..."
    cargo publish --no-verify -p "${PROJECT}"
done
