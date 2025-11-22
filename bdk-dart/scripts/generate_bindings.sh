#!/usr/bin/env bash
set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Navigate to bdk-dart directory (parent of scripts/)
BDK_DART_DIR="$SCRIPT_DIR/.."
# Navigate to bdk-ffi directory (contains Cargo.toml)
BDK_FFI_DIR="$BDK_DART_DIR/../bdk-ffi"

OS=$(uname -s)
echo "Running on $OS"

cd "$BDK_DART_DIR"
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

cd "$BDK_FFI_DIR"
echo "Generating bdk dart..."
cargo build --profile dev --features dart
cargo run --profile dev --features dart --bin uniffi-bindgen -- generate --library --language dart --out-dir "$BDK_DART_DIR/lib/" target/debug/$LIBNAME

if [[ "$OS" == "Darwin" ]]; then
    echo "Generating native binaries..."
    rustup target add aarch64-apple-darwin x86_64-apple-darwin
    cargo build --profile dev --features dart --target aarch64-apple-darwin &
    cargo build --profile dev --features dart --target x86_64-apple-darwin &
    wait

    echo "Building macos fat library"
    lipo -create -output "$BDK_DART_DIR/$LIBNAME" \
        target/aarch64-apple-darwin/debug/$LIBNAME \
        target/x86_64-apple-darwin/debug/$LIBNAME
else
    echo "Generating native binaries..."
    rustup target add x86_64-unknown-linux-gnu
    cargo build --profile dev --features dart --target x86_64-unknown-linux-gnu

    echo "Copying libbdkffi binary"
    cp target/x86_64-unknown-linux-gnu/debug/$LIBNAME "$BDK_DART_DIR/$LIBNAME"
fi

echo "All done!"