#!/bin/bash

if [ "$1" == "" ]; then
    echo "Missing version. Usage: $0 [VERSION] [--skip-napi]"
    exit -1
fi

VERSION="$1"
shift

# --skip-napi: only publish crates + wasm, do not sync/tag float-pigment-css-napi
SKIP_NAPI=0
for arg in "$@"; do
    case "$arg" in
        --skip-napi) SKIP_NAPI=1 ;;
        *) echo "Unknown option: $arg"; exit -1 ;;
    esac
done

PROJECTS=$(egrep '^[ \t]*"(.+)",$' Cargo.toml | sed -E 's/^[ \t]*"(.+)",$/\1/g')

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

# run wasm-pack to see if there is any bugs
if wasm-pack build float-pigment-css --target nodejs --features nodejs-package; then
    echo 'WebAssembly built for float-pigment-css.'
else
    echo 'Failed to build WebAssembly package for float-pigment-css! Abort.'
    exit -1
fi

# git operations
if [ -z "$(git status --porcelain)" ]; then
    echo 'Git status OK.'

    # update compile cache for float-pigment-css
    if cargo check -p float-pigment-css --features serialize_compile_cache && cargo run --bin float_pigment_css_update_version --features compile_cache; then
        echo 'Compile cache for float-pigment-css updated.'
    else
        echo 'Failed to update compile cache for float-pigment-css! Abort'
        exit -1
    fi

    # update version fields in cargo.toml
    if sed -i '' -E "s/version = \"[^\"]+\"/version = \"${VERSION}\"/" Cargo.toml; then
        echo 'Modified versions in Cargo.toml.'
    else
        echo 'Failed to modify version in Cargo.toml! Abort.'
        exit -1
    fi

    # sync the float-pigment-css-napi npm version to match the workspace.
    # Rust side follows automatically via `version.workspace = true`, but the
    # npm package.json + per-platform npm/* sub-packages are versioned
    # separately and must be kept in lock-step here.
    if [ "$SKIP_NAPI" == "1" ]; then
        echo 'Skipping float-pigment-css-napi version sync (--skip-napi).'
    elif (cd float-pigment-css-napi \
        && npm version "${VERSION}" --no-git-tag-version --allow-same-version \
        && npx --no-install napi version); then
        echo 'Synced float-pigment-css-napi npm version.'
    else
        echo 'Failed to sync napi npm version! Abort.'
        exit -1
    fi

    # run cargo check again to update cargo lock
    if cargo check; then
        echo 'Cargo check done.'
    else
        echo 'Cargo check failed! Abort.'
        exit -1
    fi

    # generate a new commit and tag it
    NAPI_PATHS="float-pigment-css-napi/package.json float-pigment-css-napi/npm"
    [ "$SKIP_NAPI" == "1" ] && NAPI_PATHS=""
    if git add Cargo.toml Cargo.lock float-pigment-css/compile_cache $NAPI_PATHS && git commit -m "chore: update version to publish"; then
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
    # also tag napi-v* to trigger the float-pigment-css-napi CI, which builds
    # all platforms and publishes the npm packages (see .github/workflows/napi.yml)
    if [ "$SKIP_NAPI" == "1" ]; then
        echo 'Skipping napi-v tag (--skip-napi).'
    elif git tag "napi-v${VERSION}"; then
        echo 'Git napi tag done.'
    else
        echo 'Git napi tag failed! Abort.'
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

# run wasm-pack again to update package version
if wasm-pack build float-pigment-css --target nodejs --features nodejs-package; then
    echo 'WebAssembly built for float-pigment-css.'
else
    echo 'Failed to build WebAssembly package for float-pigment-css! Abort.'
    exit -1
fi

# cargo publish
echo "Ready to publish version ${VERSION}."
for PROJECT in $PROJECTS; do
    echo ""
    echo "Publishing ${PROJECT}..."
    cargo publish -p "${PROJECT}"
done

# npm publish
cd float-pigment-css/pkg
echo "Publishing NPM package for float-pigment-css..."
npm publish --registry https://registry.npmjs.org
cd ../..
