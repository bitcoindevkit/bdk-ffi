#!/usr/bin/env bash
set -euo pipefail

BUILD_PROFILE=release
BDKFFI_DIR=bdk-ffi
TARGET_DIR=$BDKFFI_DIR/target
STATIC_LIB_NAME=libbdkffi.a
SWIFT_DIR="$BDKFFI_DIR/bindings/bdk-swift"
XCFRAMEWORK_NAME="bdkFFI"
XCFRAMEWORK_ROOT="$XCFRAMEWORK_NAME.xcframework"
XCFRAMEWORK_COMMON="$XCFRAMEWORK_ROOT/common/$XCFRAMEWORK_NAME.framework"

## build bdk-ffi rust libs
echo "Build bdk-ffi rust library"
pushd $BDKFFI_DIR
cargo build --release

echo "Generate bdk-ffi swift bindings"
uniffi-bindgen generate src/bdk.udl --no-format --out-dir bindings/bdk-swift/ --language swift
swiftc -module-name bdk -emit-library -o libbdkffi.dylib -emit-module -emit-module-path bindings/bdk-swift/ -parse-as-library -L target/release/ -lbdkffi -Xcc -fmodule-map-file=bindings/bdk-swift/$XCFRAMEWORK_NAME.modulemap bindings/bdk-swift/bdk.swift -suppress-warnings

## build bdk-ffi rust libs into xcframework
echo "Build bdk-ffi libs into swift xcframework"

TARGET_TRIPLES=("x86_64-apple-darwin" "x86_64-apple-ios" "aarch64-apple-ios")
for TARGET in ${TARGET_TRIPLES[@]}; do
  echo "Build bdk-ffi lib for target $TARGET"
  cargo build --release --target $TARGET
  echo $?
done

popd

## Manually construct xcframework

# Cleanup prior build
rm -rf "$XCFRAMEWORK_ROOT"
rm -f $XCFRAMEWORK_ROOT.zip

# Common files
mkdir -p "$XCFRAMEWORK_COMMON/Modules"
cp "$SWIFT_DIR/module.modulemap" "$XCFRAMEWORK_COMMON/Modules/"
mkdir -p "$XCFRAMEWORK_COMMON/Headers"
cp "$SWIFT_DIR/$XCFRAMEWORK_NAME-umbrella.h" "$XCFRAMEWORK_COMMON/Headers"
cp "$SWIFT_DIR/$XCFRAMEWORK_NAME.h" "$XCFRAMEWORK_COMMON/Headers"

# macOS x86_64 hardware
mkdir -p "$XCFRAMEWORK_ROOT/macos-x86_64"
cp -R "$XCFRAMEWORK_COMMON" "$XCFRAMEWORK_ROOT/macos-x86_64/$XCFRAMEWORK_NAME.framework"
cp "$TARGET_DIR/x86_64-apple-darwin/$BUILD_PROFILE/$STATIC_LIB_NAME" "$XCFRAMEWORK_ROOT/macos-x86_64/$XCFRAMEWORK_NAME.framework/$XCFRAMEWORK_NAME"

# iOS hardware
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64"
cp -R "$XCFRAMEWORK_COMMON" "$XCFRAMEWORK_ROOT/ios-arm64/$XCFRAMEWORK_NAME.framework"
cp "$TARGET_DIR/aarch64-apple-ios/$BUILD_PROFILE/$STATIC_LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64/$XCFRAMEWORK_NAME.framework/$XCFRAMEWORK_NAME"

# iOS simulator, currently x86_64 only (need to make fat binary to add M1)
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator"
cp -R "$XCFRAMEWORK_COMMON" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$XCFRAMEWORK_NAME.framework"
cp "$TARGET_DIR/x86_64-apple-ios/$BUILD_PROFILE/$STATIC_LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$XCFRAMEWORK_NAME.framework/$XCFRAMEWORK_NAME"

# Set up the metadata for the XCFramework as a whole.
cp "$SWIFT_DIR/Info.plist" "$XCFRAMEWORK_ROOT/Info.plist"
# TODO add license info

# Remove common
rm -rf "$XCFRAMEWORK_ROOT/common"

# Zip it all up into a bundle for distribution.
zip -9 -r "$XCFRAMEWORK_ROOT.zip" "$XCFRAMEWORK_ROOT"

swift package compute-checksum bdkFFI.xcframework.zip

# Cleanup build ?
# rm -rf "$XCFRAMEWORK_ROOT"
