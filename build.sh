#!/usr/bin/env bash
set -euo pipefail

## confirm bdk-ffi rust lib builds
pushd bdk-ffi
echo "Confirm bdk-ffi rust lib builds"
cargo build --release

## build bdk-ffi rust libs for apple targets
echo "Build bdk-ffi libs for apple targets"

TARGET_TRIPLES=("x86_64-apple-darwin" "x86_64-apple-ios" "aarch64-apple-ios")
for TARGET in ${TARGET_TRIPLES[@]}; do
  echo "Build bdk-ffi lib for target $TARGET"
  cargo build --release --target $TARGET
done

echo "Generate bdk-ffi swift bindings"
uniffi-bindgen generate src/bdk.udl --no-format --out-dir ../Sources/BitcoinDevKit --language swift
popd

# rename bdk.swift bindings to BitcoinDevKit.swift
mv Sources/BitcoinDevKit/bdk.swift Sources/BitcoinDevKit/BitcoinDevKit.swift

# copy bdkFFI.h to bdkFFI.xcframework platforms
PLATFORMS=("macos-x86_64" "ios-arm64_x86_64-simulator" "ios-arm64")
for PLATFORM in ${PLATFORMS[@]}; do
  cp Sources/BitcoinDevKit/bdkFFI.h bdkFFI.xcframework/$PLATFORM/bdkFFI.framework/Headers
done

# remove unneed .h and .modulemap files
rm Sources/BitcoinDevKit/bdkFFI.h
rm Sources/BitcoinDevkit/bdkFFI.modulemap

# add bdkFFI libs to bdkFFI.xcframework

# macos-x86_64 platform
cp bdk-ffi/target/x86_64-apple-darwin/release/libbdkffi.a bdkFFI.xcframework/macos-x86_64/bdkFFI.framework/bdkFFI

# ios-arm64 platform
cp bdk-ffi/target/aarch64-apple-ios/release/libbdkffi.a bdkFFI.xcframework/ios-arm64/bdkFFI.framework/bdkFFI

# ios-arm64_x86_64-simulator, currently x86_64 only (need to make fat binary to add M1)
cp bdk-ffi/target/x86_64-apple-ios/release/libbdkffi.a bdkFFI.xcframework/ios-arm64_x86_64-simulator/bdkFFI.framework/bdkFFI

# TODO add license info

# remove any existing bdkFFI.xcframework.zip
rm bdkFFI.xcframework.zip

# zip bdkFFI.xcframework directory into a bundle for distribution
zip -9 -r bdkFFI.xcframework.zip bdkFFI.xcframework

# compute bdkFFI.xcframework.zip checksum
echo checksum:
swift package compute-checksum bdkFFI.xcframework.zip

# TODO update Package.swift with checksum
# TODO upload zip to github release
