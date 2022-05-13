#!/usr/bin/env bash
set -euo pipefail

## confirm bdk-ffi rust lib builds
pushd bdk-ffi
echo "Confirm bdk-ffi rust lib builds"
cargo build --release

echo "Generate bdk-ffi swift bindings"
cargo run --package bdk-ffi-bindgen -- --language swift --out-dir ../Sources/BitcoinDevKit

## build bdk-ffi rust libs for apple targets and add to xcframework
echo "Build bdk-ffi libs for apple targets and add to xcframework"

TARGET_TRIPLES=("x86_64-apple-darwin" "aarch64-apple-darwin" "x86_64-apple-ios" "aarch64-apple-ios")
#XCFRAMEWORK_LIBS=""
for TARGET in ${TARGET_TRIPLES[@]}; do
  echo "Build bdk-ffi lib for target $TARGET"
  cargo build --release --target $TARGET
  #XCFRAMEWORK_LIBS="$XCFRAMEWORK_LIBS -library target/$TARGET/release/libbdkffi.a"
done
# special build for M1 ios simulator
cargo +nightly build --release -Z build-std --target aarch64-apple-ios-sim

echo "Create lipo static libs for ios-sim to support M1"
mkdir -p target/lipo-ios-sim/release
lipo target/aarch64-apple-ios-sim/release/libbdkffi.a target/x86_64-apple-ios/release/libbdkffi.a -create -output target/lipo-ios-sim/release/libbdkffi.a

echo "Create lipo static libs for macos to support M1"
mkdir -p target/lipo-macos/release
lipo target/aarch64-apple-darwin/release/libbdkffi.a target/x86_64-apple-darwin/release/libbdkffi.a -create -output target/lipo-macos/release/libbdkffi.a

#echo "Create xcframework with xcodebuild"
#xcodebuild -create-xcframework \
#    -library target/lipo-ios-sim/release/libbdkffi.a \
#    -library target/lipo-macos/release/libbdkffi.a \
#    -library target/aarch64-apple-ios/release/libbdkffi.a \
#    -output ../bdkFFI.xcframework

popd

# rename bdk.swift bindings to BitcoinDevKit.swift
mv Sources/BitcoinDevKit/bdk.swift Sources/BitcoinDevKit/BitcoinDevKit.swift

XCFRAMEWORK_LIBS=("ios-arm64" "ios-arm64_x86_64-simulator" "macos-arm64_x86_64")
for LIB in ${XCFRAMEWORK_LIBS[@]}; do
    # copy possibly updated header file
    cp Sources/BitcoinDevKit/bdkFFI.h bdkFFI.xcframework/$LIB/bdkFFI.framework/Headers
done

echo "Copy libbdkffi.a files to bdkFFI.xcframework/bdkFFI"
cp bdk-ffi/target/aarch64-apple-ios/release/libbdkffi.a bdkFFI.xcframework/ios-arm64/bdkFFI.framework/bdkFFI
cp bdk-ffi/target/lipo-ios-sim/release/libbdkffi.a bdkFFI.xcframework/ios-arm64_x86_64-simulator/bdkFFI.framework/bdkFFI
cp bdk-ffi/target/lipo-macos/release/libbdkffi.a bdkFFI.xcframework/macos-arm64_x86_64/bdkFFI.framework/bdkFFI

# remove unneed .h and .modulemap files
rm Sources/BitcoinDevKit/bdkFFI.h
rm Sources/BitcoinDevkit/bdkFFI.modulemap

# TODO add license info

if test -f "bdkFFI.xcframework.zip"; then
    echo "Remove old bdkFFI.xcframework.zip"
    rm bdkFFI.xcframework.zip
fi


# zip bdkFFI.xcframework directory into a bundle for distribution
zip -9 -r bdkFFI.xcframework.zip bdkFFI.xcframework

# compute bdkFFI.xcframework.zip checksum
echo checksum:
swift package compute-checksum bdkFFI.xcframework.zip

# TODO update Package.swift with checksum
# TODO upload zip to github release
