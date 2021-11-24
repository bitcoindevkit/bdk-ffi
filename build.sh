#!/usr/bin/env bash
set -eo pipefail

# functions

## help
help()
{
   # Display Help
   echo "Build bdk-ffi and related libraries."
   echo
   echo "Syntax: build [-a|h|k|s]"
   echo "options:"
   echo "-a     Android."
   echo "-h     Print this Help."
   echo "-k     Kotlin."
   echo
}

## rust
build_rust() {
  echo "Build Rust library"
  cargo fmt
  cargo build --release
  cargo test
}

## copy to bdk-bdk-kotlin
copy_lib_kotlin() {
  echo -n "Copy "
  case $OS in
    "Darwin")
      echo -n "darwin "
      mkdir -p bindings/bdk-kotlin/jvm/src/main/resources/darwin-x86-64
      cp target/release/libbdkffi.dylib bindings/bdk-kotlin/jvm/src/main/resources/darwin-x86-64
      ;;
    "Linux")
      echo -n "linux "
      mkdir -p bindings/bdk-kotlin/jvm/src/main/resources/linux-x86-64
      cp target/release/libbdkffi.so bindings/bdk-kotlin/jvm/src/main/resources/linux-x86-64
      ;;
  esac
  echo "libs to kotlin sub-project"
}

## bdk-bdk-kotlin jar
build_kotlin() {
  copy_lib_kotlin
  uniffi-bindgen generate src/bdk.udl --no-format --out-dir bindings/bdk-kotlin/jvm/src/main/kotlin --language kotlin
}

## rust android
build_android() {
  build_kotlin

  # If ANDROID_NDK_HOME is not set then set it to github actions default
  [ -z "$ANDROID_NDK_HOME" ] && export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle

  # Update this line accordingly if you are not building *from* darwin-x86_64 or linux-x86_64
  export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/`uname | tr '[:upper:]' '[:lower:]'`-x86_64/bin

  # Required for 'ring' dependency to cross-compile to Android platform, must be at least 21
  export CFLAGS="-D__ANDROID_API__=21"

  # IMPORTANT: make sure every target is not a substring of a different one. We check for them with grep later on
  BUILD_TARGETS="${BUILD_TARGETS:-aarch64,x86_64,i686}"

  mkdir -p bindings/bdk-kotlin/android/src/main/jniLibs/ bindings/bdk-kotlin/android/src/main/jniLibs/arm64-v8a bindings/bdk-kotlin/android/src/main/jniLibs/x86_64 bindings/bdk-kotlin/android/src/main/jniLibs/x86

  if echo $BUILD_TARGETS | grep "aarch64"; then
      CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android21-clang" CC="aarch64-linux-android21-clang" cargo build --release --target=aarch64-linux-android
      cp target/aarch64-linux-android/release/libbdkffi.so bindings/bdk-kotlin/android/src/main/jniLibs/arm64-v8a
  fi
  if echo $BUILD_TARGETS | grep "x86_64"; then
      CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="x86_64-linux-android21-clang" CC="x86_64-linux-android21-clang" cargo build --release --target=x86_64-linux-android
      cp target/x86_64-linux-android/release/libbdkffi.so bindings/bdk-kotlin/android/src/main/jniLibs/x86_64
  fi
  if echo $BUILD_TARGETS | grep "i686"; then
      CARGO_TARGET_I686_LINUX_ANDROID_LINKER="i686-linux-android21-clang" CC="i686-linux-android21-clang" cargo build --release --target=i686-linux-android
      cp target/i686-linux-android/release/libbdkffi.so bindings/bdk-kotlin/android/src/main/jniLibs/x86
  fi

  # copy sources
  cp -R bindings/bdk-kotlin/jvm/src/main/kotlin bindings/bdk-kotlin/android/src/main

  # bdk-kotlin aar
  (cd bindings/bdk-kotlin && ./gradlew :android:build)
}

OS=$(uname)

if [ "$1" == "-h" ]
then
  help
else
  build_rust

  while [ -n "$1" ]; do # while loop starts
    case "$1" in
      -a) build_android ;;
      -k) build_kotlin ;;
      -h) help ;;
      *) echo "Option $1 not recognized" ;;
    esac
    shift
  done
fi
