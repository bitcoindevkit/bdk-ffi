#!/bin/bash
# This script builds local swift-bdk Swift language bindings and corresponding bdkFFI.xcframework.
# The results of this script can be used for locally testing your SPM package adding a local package
# to your application pointing at the bdk-swift directory.

HEADER_BASENAME="BitcoinDevKitFFI"
TARGETDIR="../bdk-ffi/target"
OUTDIR="."
RELDIR="release-smaller"
NAME="bdkffi"
STATIC_LIB_NAME="lib${NAME}.a"
NEW_HEADER_DIR="../bdk-ffi/target/include"
SWIFT_OUT_DIR="../bdk-swift/Sources/BitcoinDevKit"
HEADER_OUT_DIR="${NEW_HEADER_DIR}/${HEADER_BASENAME}"

cd ../bdk-ffi/ || exit

# install component and targets
rustup component add rust-src
rustup target add aarch64-apple-ios      # iOS arm64
rustup target add x86_64-apple-ios       # iOS x86_64
rustup target add aarch64-apple-ios-sim  # simulator mac M1
rustup target add aarch64-apple-darwin   # mac M1
rustup target add x86_64-apple-darwin    # mac x86_64

# build bdk-ffi rust lib for apple targets
cargo build --package bdk-ffi --profile release-smaller --target x86_64-apple-darwin
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-darwin
cargo build --package bdk-ffi --profile release-smaller --target x86_64-apple-ios
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-ios
cargo build --package bdk-ffi --profile release-smaller --target aarch64-apple-ios-sim

# build bdk-ffi Swift bindings and put in bdk-swift Sources
UNIFFI_LIBRARY_PATH="./target/aarch64-apple-ios/${RELDIR}/lib${NAME}.dylib"
cargo run --bin uniffi-bindgen generate --library "${UNIFFI_LIBRARY_PATH}" --language swift --out-dir "${SWIFT_OUT_DIR}" --no-format

# Final xcframework structure (per-arch):
#   Headers/
#     <ModuleName>/
#       <ModuleName>.h
#       module.modulemap
rm -rf "${HEADER_OUT_DIR:?}"
mkdir -p "${HEADER_OUT_DIR}"
cargo run --bin uniffi-bindgen generate --library "${UNIFFI_LIBRARY_PATH}" --language swift --out-dir "${HEADER_OUT_DIR}" --no-format

# Keep the header output directory clean: xcframework headers should only contain .h + module.modulemap
find "${HEADER_OUT_DIR}" -maxdepth 1 -name '*.swift' -delete

# Uniffi emits <basename>.modulemap; rename it to module.modulemap (expected by Apple toolchains)
if [ -f "${HEADER_OUT_DIR}/${HEADER_BASENAME}.modulemap" ]; then
    mv "${HEADER_OUT_DIR}/${HEADER_BASENAME}.modulemap" "${HEADER_OUT_DIR}/module.modulemap"
fi

echo -e "\n" >> "${HEADER_OUT_DIR}/module.modulemap"

# combine bdk-ffi static libs for aarch64 and x86_64 targets via lipo tool
mkdir -p target/lipo-ios-sim/release-smaller
lipo target/aarch64-apple-ios-sim/release-smaller/libbdkffi.a target/x86_64-apple-ios/release-smaller/libbdkffi.a -create -output target/lipo-ios-sim/release-smaller/libbdkffi.a
mkdir -p target/lipo-macos/release-smaller
lipo target/aarch64-apple-darwin/release-smaller/libbdkffi.a target/x86_64-apple-darwin/release-smaller/libbdkffi.a -create -output target/lipo-macos/release-smaller/libbdkffi.a

cd ../bdk-swift/ || exit

# remove old xcframework directory
rm -rf "${OUTDIR}/${NAME}.xcframework"

# create new xcframework directory from bdk-ffi static libs and headers
xcodebuild -create-xcframework \
    -library "${TARGETDIR}/lipo-macos/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/aarch64-apple-ios/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/lipo-ios-sim/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}.xcframework"