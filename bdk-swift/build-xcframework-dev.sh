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

# move bdk-ffi static lib header files to temporary directory
# Final xcframework structure (per-arch):
#   Headers/
#     BitcoinDevKitFFI/
#       BitcoinDevKitFFI.h
#       module.modulemap

# Start from a clean header include dir so we don't get duplicates
rm -f "${NEW_HEADER_DIR}/BitcoinDevKitFFI.h" "${NEW_HEADER_DIR}/module.modulemap"
rm -rf "${NEW_HEADER_DIR}/BitcoinDevKitFFI"
mkdir -p "${NEW_HEADER_DIR}/BitcoinDevKitFFI"

# Move generated header and modulemap into BitcoinDevKitFFI subfolder only
mv "${HEADERPATH}" "${NEW_HEADER_DIR}/BitcoinDevKitFFI/BitcoinDevKitFFI.h"
mv "${MODMAPPATH}" "${NEW_HEADER_DIR}/BitcoinDevKitFFI/module.modulemap"

# Ensure the modulemap points at the header using the BitcoinDevKitFFI/ prefix,
# matching the desired structure inside bdkffi.xcframework.
sed -i '' 's#header \"BitcoinDevKitFFI/BitcoinDevKitFFI.h\"#header \"BitcoinDevKitFFI.h\"#' "${NEW_HEADER_DIR}/BitcoinDevKitFFI/module.modulemap" || true

echo -e "\n" >> "${NEW_HEADER_DIR}/BitcoinDevKitFFI/module.modulemap"

rm -rf "${OUTDIR}/${NAME}.xcframework"

xcodebuild -create-xcframework \
    -library "${TARGETDIR}/${MAC_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_DEVICE_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/${IOS_SIM_TARGET}/${PROFILE_DIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}.xcframework"
