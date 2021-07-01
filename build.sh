#!/usr/bin/env bash
set -eo pipefail -o xtrace

# rust
cargo fmt
cargo build
cargo test --features c-headers -- generate_headers

# cc
export LD_LIBRARY_PATH=`pwd`/target/debug
cc cc/bdk_ffi_test.c -o cc/bdk_ffi_test -L target/debug -l bdk_ffi -l pthread -l dl -l m

# bdk-kotlin jar
mkdir -p bdk-kotlin/jvm/src/main/resources/linux-x86-64
cp target/debug/libbdk_ffi.so bdk-kotlin/jvm/src/main/resources/linux-x86-64

(cd bdk-kotlin && gradle :jvm:build && gradle :jvm:publishToMavenLocal)

# rust android

# If ANDROID_NDK_HOME is not set then set it to github actions default
[ -z "$ANDROID_NDK_HOME" ] && export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle

# Update this line accordingly if you are not building *from* darwin-x86_64 or linux-x86_64
export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/`uname | tr '[:upper:]' '[:lower:]'`-x86_64/bin

# Required for 'ring' dependency to cross-compile to Android platform, must be at least 21
export CFLAGS="-D__ANDROID_API__=21"

# IMPORTANT: make sure every target is not a substring of a different one. We check for them with grep later on
BUILD_TARGETS="${BUILD_TARGETS:-aarch64,armv7,x86_64,i686}"

mkdir -p bdk-kotlin/android/src/main/jniLibs/ bdk-kotlin/android/src/main/jniLibs/arm64-v8a bdk-kotlin/android/src/main/jniLibs/x86_64 bdk-kotlin/android/src/main/jniLibs/armeabi-v7a bdk-kotlin/android/src/main/jniLibs/x86

if echo $BUILD_TARGETS | grep "aarch64"; then
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android21-clang" CC="aarch64-linux-android21-clang" cargo build --target=aarch64-linux-android
    cp target/aarch64-linux-android/debug/libbdk_ffi.so bdk-kotlin/android/src/main/jniLibs/arm64-v8a
fi
if echo $BUILD_TARGETS | grep "x86_64"; then
    CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="x86_64-linux-android21-clang" CC="x86_64-linux-android21-clang" cargo build --target=x86_64-linux-android
    cp target/x86_64-linux-android/debug/libbdk_ffi.so bdk-kotlin/android/src/main/jniLibs/x86_64
fi
if echo $BUILD_TARGETS | grep "armv7"; then
    CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="armv7a-linux-androideabi21-clang" CC="armv7a-linux-androideabi21-clang" cargo build --target=armv7-linux-androideabi
    cp target/armv7-linux-androideabi/debug/libbdk_ffi.so bdk-kotlin/android/src/main/jniLibs/armeabi-v7a
fi
if echo $BUILD_TARGETS | grep "i686"; then
    CARGO_TARGET_I686_LINUX_ANDROID_LINKER="i686-linux-android21-clang" CC="i686-linux-android21-clang" cargo build --target=i686-linux-android
    cp target/i686-linux-android/debug/libbdk_ffi.so bdk-kotlin/android/src/main/jniLibs/x86
fi

# bdk-kotlin aar
(cd bdk-kotlin && gradle :android:build && gradle :android:publishToMavenLocal)
