#!/usr/bin/env bash
set -eo pipefail

echo "Build and test bdk-ffi library for local platform (darwin or linux)"
pushd bdk-ffi

OS=$(uname)
echo -n "Copy "
case $OS in
  "Darwin")
    echo -n "darwin "
    # x86_64 (intel)
    cargo build --release --target x86_64-apple-darwin
    mkdir -p ../jvm/src/main/resources/darwin-x86-64
    cp target/x86_64-apple-darwin/release/libbdkffi.dylib ../jvm/src/main/resources/darwin-x86-64
    # aarch64 (m1)
    cargo build --release --target aarch64-apple-darwin
    mkdir -p ../jvm/src/main/resources/darwin-aarch64
    cp target/aarch64-apple-darwin/release/libbdkffi.dylib ../jvm/src/main/resources/darwin-aarch64
    ;;
  "Linux")
    echo -n "linux "
    cargo build --release
    mkdir -p ../jvm/src/main/resources/linux-x86-64
    cp target/release/libbdkffi.so ../jvm/src/main/resources/linux-x86-64
    ;;
esac
echo "libs to jvm subproject"

echo "Generate kotlin bindings from bdk.udl to jvm subproject"
uniffi-bindgen generate src/bdk.udl --no-format --out-dir ../jvm/src/main/kotlin --language kotlin

## android

# If ANDROID_NDK_HOME is not set then set it to github actions default
[ -z "$ANDROID_NDK_HOME" ] && export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle

# Update this line accordingly if you are not building *from* darwin-x86_64 or linux-x86_64
export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/`uname | tr '[:upper:]' '[:lower:]'`-x86_64/bin

# Required for 'ring' dependency to cross-compile to Android platform, must be at least 21
export CFLAGS="-D__ANDROID_API__=21"

# IMPORTANT: make sure every target is not a substring of a different one. We check for them with grep later on
BUILD_TARGETS="${BUILD_TARGETS:-aarch64,x86_64,i686}"

mkdir -p ../android/src/main/jniLibs/arm64-v8a ../android/src/main/jniLibs/x86_64 ../android/src/main/jniLibs/x86

if echo $BUILD_TARGETS | grep "aarch64"; then
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android21-clang" CC="aarch64-linux-android21-clang" cargo build --release --target=aarch64-linux-android
    cp target/aarch64-linux-android/release/libbdkffi.so ../android/src/main/jniLibs/arm64-v8a
fi
if echo $BUILD_TARGETS | grep "x86_64"; then
    CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="x86_64-linux-android21-clang" CC="x86_64-linux-android21-clang" cargo build --release --target=x86_64-linux-android
    cp target/x86_64-linux-android/release/libbdkffi.so ../android/src/main/jniLibs/x86_64
fi
if echo $BUILD_TARGETS | grep "i686"; then
    CARGO_TARGET_I686_LINUX_ANDROID_LINKER="i686-linux-android21-clang" CC="i686-linux-android21-clang" cargo build --release --target=i686-linux-android
    cp target/i686-linux-android/release/libbdkffi.so ../android/src/main/jniLibs/x86
fi

popd

# copy bdk-ffi kotlin binding sources from jvm to android
cp -R jvm/src/main/kotlin android/src/main

# bdk-kotlin build jar and aar subprojects
./gradlew build
