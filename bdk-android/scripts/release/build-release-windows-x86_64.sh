#!/bin/bash

if [ -z "$ANDROID_NDK_ROOT" ]; then
    echo "Error: ANDROID_NDK_ROOT is not defined in your environment"
    exit 1
fi

# Update PATH for Windows
PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/bin:$PATH"
CFLAGS="-D__ANDROID_MIN_SDK_VERSION__=24"
AR="llvm-ar"
LIB_NAME="libbdkffi.so"
COMPILATION_TARGET_ARM64_V8A="aarch64-linux-android"
COMPILATION_TARGET_X86_64="x86_64-linux-android"
COMPILATION_TARGET_ARMEABI_V7A="armv7-linux-androideabi"
RESOURCE_DIR_ARM64_V8A="arm64-v8a"
RESOURCE_DIR_X86_64="x86_64"
RESOURCE_DIR_ARMEABI_V7A="armeabi-v7a"

# Move to the Rust library directory
cd ../bdk-ffi/ || exit
rustup target add $COMPILATION_TARGET_ARM64_V8A $COMPILATION_TARGET_ARMEABI_V7A $COMPILATION_TARGET_X86_64 

# Build the binaries
# The CC and CARGO_TARGET_<TARGET>_LINUX_ANDROID_LINKER environment variables must be declared on the same line as the cargo build command
CC="aarch64-linux-android24-clang.cmd" CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android24-clang.cmd" cargo build --profile release-smaller --target $COMPILATION_TARGET_ARM64_V8A
CC="x86_64-linux-android24-clang.cmd" CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="x86_64-linux-android24-clang.cmd" cargo build --profile release-smaller --target $COMPILATION_TARGET_X86_64
CC="armv7a-linux-androideabi24-clang.cmd" CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="armv7a-linux-androideabi24-clang.cmd" cargo build --profile release-smaller --target $COMPILATION_TARGET_ARMEABI_V7A

# Copy the binaries to their respective resource directories
mkdir -p ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
mkdir -p ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARMEABI_V7A/
mkdir -p ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_X86_64/
cp ./target/$COMPILATION_TARGET_ARM64_V8A/release-smaller/$LIB_NAME ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARM64_V8A/
cp ./target/$COMPILATION_TARGET_ARMEABI_V7A/release-smaller/$LIB_NAME ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_ARMEABI_V7A/
cp ./target/$COMPILATION_TARGET_X86_64/release-smaller/$LIB_NAME ../bdk-android/lib/src/main/jniLibs/$RESOURCE_DIR_X86_64/

# Generate Kotlin bindings using uniffi-bindgen. (Any of the other $COMPILATION_TARGET_* could have been used)
cargo run --bin uniffi-bindgen generate --library ./target/$COMPILATION_TARGET_ARM64_V8A/release-smaller/$LIB_NAME --language kotlin --out-dir ../bdk-android/lib/src/main/kotlin/ --no-format
