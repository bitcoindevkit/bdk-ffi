#!/usr/bin/env bash
set -euo pipefail

OS=$(uname -s)
echo "Running on $OS"

dart --version
dart pub get

# Install Rust targets if on macOS
if [[ "$OS" == "Darwin" ]]; then
    LIBNAME=libbdkffi.dylib
elif [[ "$OS" == "Linux" ]]; then
    LIBNAME=libbdkffi.so
else
    echo "Unsupported os: $OS"
    exit 1
fi

cd ../bdk-ffi/
echo "Generating bdk-ffi dart..."
cargo build --profile dev
cargo run --profile dev --bin uniffi-bindgen -- --library target/debug/$LIBNAME --language dart --out-dir ../bdk-dart/lib/

if [[ "$OS" == "Darwin" ]]; then
    echo "Generating native binaries..."
    rustup target add aarch64-apple-darwin x86_64-apple-darwin
    # This is a test script the actual release should not include the test utils feature
    cargo build --profile dev --target aarch64-apple-darwin &
    cargo build --profile dev --target x86_64-apple-darwin &
    wait

    echo "Building macos fat library"
    lipo -create -output ../bdk-dart/$LIBNAME \
        target/aarch64-apple-darwin/debug/$LIBNAME \
        target/x86_64-apple-darwin/debug/$LIBNAME
else
    echo "Generating native binaries..."
    rustup target add x86_64-unknown-linux-gnu
    # This is a test script the actual release should not include the test utils feature
    cargo build --profile dev --target x86_64-unknown-linux-gnu

    echo "Copying bdk-ffi binary"
    cp target/x86_64-unknown-linux-gnu/debug/$LIBNAME ../bdk-dart/$LIBNAME
fi

echo "All done!"
