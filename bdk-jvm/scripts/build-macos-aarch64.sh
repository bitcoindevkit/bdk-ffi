#!/bin/bash

# Move to the Rust library directory
cd ../bdk-ffi/

# Build the Rust library
cargo build --profile release-smaller --target aarch64-apple-darwin

# Generate Kotlin bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-darwin/release-smaller/libbdkffi.dylib --language kotlin --out-dir ../bdk-jvm/lib/src/main/kotlin/ --no-format

# Copy the binary to the Android resources directory
cp ./target/aarch64-apple-darwin/release-smaller/libbdkffi.dylib ../bdk-jvm/lib/src/main/resources/darwin-aarch64/libbdkffi.dylib
