#!/bin/bash
# This script builds local swift-bdk Swift language bindings and corresponding bdkFFI.xcframework.
# The results of this script can be used for locally testing your SPM package adding a local package
# to your application pointing at the bdk-swift directory.

# Run the script from the repo root directory, ie: ./bdk-swift/build-local-swift.sh

rustup install 1.74.0
rustup component add rust-src
rustup target add aarch64-apple-ios      # iOS ARM64
rustup target add x86_64-apple-ios       # iOS x86_64
rustup target add aarch64-apple-ios-sim  # simulator mac M1
rustup target add aarch64-apple-darwin   # mac M1
rustup target add x86_64-apple-darwin    # mac x86_64

pushd bdk-ffi
mkdir -p Sources/BitcoinDevKit
cargo run --bin uniffi-bindgen generate src/bdk.udl --language swift --out-dir ../bdk-swift/Sources/BitcoinDevKit --no-format

cargo build --package bdk-ffi --profile release-smaller --target x86_64-apple-darwin
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-darwin
cargo build --package bdk-ffi --profile release-smaller --target x86_64-apple-ios
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-ios
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-ios-sim

mkdir -p target/lipo-ios-sim/release-smaller
lipo target/aarch64-apple-ios-sim/release-smaller/libbdkffi.a target/x86_64-apple-ios/release-smaller/libbdkffi.a -create -output target/lipo-ios-sim/release-smaller/libbdkffi.a
mkdir -p target/lipo-macos/release-smaller
lipo target/aarch64-apple-darwin/release-smaller/libbdkffi.a target/x86_64-apple-darwin/release-smaller/libbdkffi.a -create -output target/lipo-macos/release-smaller/libbdkffi.a

popd
pushd bdk-swift
mv Sources/BitcoinDevKit/bdk.swift Sources/BitcoinDevKit/BitcoinDevKit.swift
cp Sources/BitcoinDevKit/bdkFFI.h bdkFFI.xcframework/ios-arm64/bdkFFI.framework/Headers
cp Sources/BitcoinDevKit/bdkFFI.h bdkFFI.xcframework/ios-arm64_x86_64-simulator/bdkFFI.framework/Headers
cp Sources/BitcoinDevKit/bdkFFI.h bdkFFI.xcframework/macos-arm64_x86_64/bdkFFI.framework/Headers
cp ../bdk-ffi/target/aarch64-apple-ios/release-smaller/libbdkffi.a bdkFFI.xcframework/ios-arm64/bdkFFI.framework/bdkFFI
cp ../bdk-ffi/target/lipo-ios-sim/release-smaller/libbdkffi.a bdkFFI.xcframework/ios-arm64_x86_64-simulator/bdkFFI.framework/bdkFFI
cp ../bdk-ffi/target/lipo-macos/release-smaller/libbdkffi.a bdkFFI.xcframework/macos-arm64_x86_64/bdkFFI.framework/bdkFFI
rm Sources/BitcoinDevKit/bdkFFI.h
rm Sources/BitcoinDevKit/bdkFFI.modulemap
#rm bdkFFI.xcframework.zip || true
#zip -9 -r bdkFFI.xcframework.zip bdkFFI.xcframework