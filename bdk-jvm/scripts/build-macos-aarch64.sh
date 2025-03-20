#!/bin/bash

COMPILATION_TARGET="aarch64-apple-darwin"
TARGET_DIR="target/$COMPILATION_TARGET/release-smaller"
RESOURCE_DIR="resources/darwin-aarch64"
LIB_NAME="libbdkffi.dylib"

# Move to the Rust library directory
cd ../bdk-ffi/ || exit

# Build the Rust library
rustup default 1.84.1
rustup target add $COMPILATION_TARGET
cargo build --profile release-smaller --target $COMPILATION_TARGET

# Generate Kotlin bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./$TARGET_DIR/$LIB_NAME --language kotlin --out-dir ../bdk-jvm/lib/src/main/kotlin/ --no-format

# Copy the binary to the resources directory
mkdir -p ../bdk-jvm/lib/src/main/$RESOURCE_DIR/
cp ./$TARGET_DIR/$LIB_NAME ../bdk-jvm/lib/src/main/$RESOURCE_DIR/
