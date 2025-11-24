#!/bin/bash

set -euo pipefail

HEADERPATH="Sources/BitcoinDevKit/BitcoinDevKitFFI.h"
MODMAPPATH="Sources/BitcoinDevKit/BitcoinDevKitFFI.modulemap"
TARGETDIR="../bdk-ffi/target"
OUTDIR="."
NAME="bdkffi"
STATIC_LIB_NAME="lib${NAME}.a"
NEW_HEADER_DIR="../bdk-ffi/target/include"
PROFILE_DIR="debug"

HOST_ARCH=$(uname -m)
if [ "$HOST_ARCH" = "arm64" ]; then
    MAC_TARGET="aarch64-apple-darwin"
    IOS_SIM_TARGET="aarch64-apple-ios-sim"
else
    MAC_TARGET="x86_64-apple-darwin"
    IOS_SIM_TARGET="x86_64-apple-ios"
fi
IOS_DEVICE_TARGET="aarch64-apple-ios"

cd ../bdk-ffi/ || exit

rustup component add rust-src
rustup target add "$MAC_TARGET" "$IOS_SIM_TARGET" "$IOS_DEVICE_TARGET"

cargo build --package bdk-ffi --target "$MAC_TARGET"
cargo build --package bdk-ffi --target "$IOS_SIM_TARGET"
cargo build --package bdk-ffi --target "$IOS_DEVICE_TARGET"

cargo run --bin uniffi-bindgen generate \
    --library "./target/$IOS_DEVICE_TARGET/$PROFILE_DIR/lib${NAME}.dylib" \
    --language swift \
    --out-dir ../bdk-swift/Sources/BitcoinDevKit \
    --no-format

cd ../bdk-swift/ || exit

mkdir -p "$NEW_HEADER_DIR"
mv "$HEADERPATH" "$NEW_HEADER_DIR"
mv "$MODMAPPATH" "$NEW_HEADER_DIR/module.modulemap"
echo >> "$NEW_HEADER_DIR/module.modulemap"

rm -rf "${OUTDIR}/${NAME}.xcframework"

xcodebuild -create-xcframework \
    -library "${TARGETDIR}/${MAC_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_DEVICE_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_SIM_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}.xcframework"
