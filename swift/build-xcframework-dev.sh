#!/bin/bash

set -euo pipefail

HEADER_BASENAME="BitcoinDevKitFFI"
TARGETDIR="../bdk-ffi/target"
OUTDIR="."
NAME="bdkffi"
STATIC_LIB_NAME="lib${NAME}.a"
NEW_HEADER_DIR="../bdk-ffi/target/include"
PROFILE_DIR="debug"
SWIFT_OUT_DIR="../bdk-swift/Sources/BitcoinDevKit"
HEADER_OUT_DIR="${NEW_HEADER_DIR}/${HEADER_BASENAME}"
MIN_IOS_VERSION="15.0"

HOST_ARCH=$(uname -m)
if [ "$HOST_ARCH" = "arm64" ]; then
    MAC_TARGET="aarch64-apple-darwin"
    IOS_SIM_TARGET="aarch64-apple-ios-sim"
else
    MAC_TARGET="x86_64-apple-darwin"
    IOS_SIM_TARGET="x86_64-apple-ios"
fi
IOS_DEVICE_TARGET="aarch64-apple-ios"

export IPHONEOS_DEPLOYMENT_TARGET="${MIN_IOS_VERSION}"

cd ../bdk-ffi/ || exit

rustup component add rust-src
rustup target add "$MAC_TARGET" "$IOS_SIM_TARGET" "$IOS_DEVICE_TARGET"

cargo build --package bdk-ffi --target "$MAC_TARGET"
cargo build --package bdk-ffi --target "$IOS_SIM_TARGET"
cargo build --package bdk-ffi --target "$IOS_DEVICE_TARGET"

UNIFFI_LIBRARY_PATH="./target/$IOS_DEVICE_TARGET/$PROFILE_DIR/lib${NAME}.dylib"
cargo run --bin uniffi-bindgen generate \
    --library "${UNIFFI_LIBRARY_PATH}" \
    --language swift \
    --out-dir "${SWIFT_OUT_DIR}" \
    --no-format

# Final xcframework structure (per-arch):
#   Headers/
#     <ModuleName>/
#       <ModuleName>.h
#       module.modulemap
rm -rf "${NEW_HEADER_DIR:?}"/*
rm -rf "${HEADER_OUT_DIR:?}"
mkdir -p "${HEADER_OUT_DIR}"
cargo run --bin uniffi-bindgen generate \
    --library "${UNIFFI_LIBRARY_PATH}" \
    --language swift \
    --out-dir "${HEADER_OUT_DIR}" \
    --no-format

# Keep the header output directory clean: xcframework headers should only contain .h + module.modulemap
find "${HEADER_OUT_DIR}" -maxdepth 1 -name '*.swift' -delete

# Uniffi emits <basename>.modulemap; rename it to module.modulemap (expected by Apple toolchains)
if [ -f "${HEADER_OUT_DIR}/${HEADER_BASENAME}.modulemap" ]; then
    mv "${HEADER_OUT_DIR}/${HEADER_BASENAME}.modulemap" "${HEADER_OUT_DIR}/module.modulemap"
fi

echo -e "\n" >> "${HEADER_OUT_DIR}/module.modulemap"

# Keep Swift sources clean: only .swift files should stay in the package Sources dir
rm -f "${SWIFT_OUT_DIR}/${HEADER_BASENAME}.h"
rm -f "${SWIFT_OUT_DIR}/${HEADER_BASENAME}.modulemap"

cd ../bdk-swift/ || exit

rm -rf "${OUTDIR}/${NAME}.xcframework"

xcodebuild -create-xcframework \
    -library "${TARGETDIR}/${MAC_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_DEVICE_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_SIM_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}.xcframework"
