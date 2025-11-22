#!/bin/bash

if [ -z "$ANDROID_NDK_ROOT" ]; then
    echo "Error: ANDROID_NDK_ROOT is not defined in your environment"
    exit 1
fi

PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/darwin-x86_64/bin:$PATH"
CFLAGS="-D__ANDROID_MIN_SDK_VERSION__=24"
AR="llvm-ar"
LIB_NAME="libbdkffi.so"
COMPILATION_TARGET_ARM64_V8A="aarch64-linux-android"
RESOURCE_DIR_ARM64_V8A="arm64-v8a"

# Move to the Rust library directory
cd ../bdk-ffi/ || exit
rustup target add $COMPILATION_TARGET_ARM64_V8A 

# Build the binaries
# The CC and CARGO_TARGET_<TARGET>_LINUX_ANDROID_LINKER environment variables must be declared on the same line as the cargo build command
CC="aarch64-linux-android24-clang" CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android24-clang" cargo build --target $COMPILATION_TARGET_ARM64_V8A

# Copy the binaries to their respective resource directories
mkdir -p ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
cp ./target/$COMPILATION_TARGET_ARM64_V8A/debug/$LIB_NAME ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/

# Generate Kotlin bindings using uniffi-bindgen
cargo run --bin uniffi-bindgen generate --library ./target/$COMPILATION_TARGET_ARM64_V8A/debug/$LIB_NAME --language kotlin --out-dir ../bdk-android/lib/src/main/kotlin/ --no-format
