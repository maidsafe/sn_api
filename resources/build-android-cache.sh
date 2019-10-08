#!/usr/bin/env bash

if [[ -z "$build_type" ]]; then
    echo "build_type must be set to dev or non-dev"
    exit 1
fi

if [[ -z "$target" ]]; then
    echo "target must be set to x86_64-linux-android or armv7-linux-androideabi"
    exit 1
fi

if [[ "$build_type" == "dev" ]]; then
    cargo build --features=mock-network,fake-auth \
        --release --lib --manifest-path=safe-ffi/Cargo.toml --target="$target"
else
    cargo build --release --lib --manifest-path=safe-ffi/Cargo.toml --target="$target"
fi
