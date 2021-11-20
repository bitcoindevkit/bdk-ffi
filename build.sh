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
   echo "-s     Swift."
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

## bdk swift
build_swift() {
  BUILD_PROFILE=release
  TARGET_DIR=target

  uniffi-bindgen generate src/bdk.udl --no-format --out-dir bindings/bdk-swift/ --language swift
  swiftc -module-name bdk -emit-library -o libbdkffi.dylib -emit-module -emit-module-path ./bindings/bdk-swift/ -parse-as-library -L ./target/release/ -lbdkffi -Xcc -fmodule-map-file=./bindings/bdk-swift/bdkFFI.modulemap ./bindings/bdk-swift/bdk.swift

  STATIC_LIB_NAME=libbdkffi.a

  # Build ios and ios x86_64 ios simulator binaries
  LIBS_ARCHS=("x86_64" "arm64")
  IOS_TRIPLES=("x86_64-apple-ios" "aarch64-apple-ios")
  for i in "${!LIBS_ARCHS[@]}"; do
      cargo build --release --target "${IOS_TRIPLES[${i}]}"
  done

  ## Manually construct xcframework
  LIB_NAME=libbdkffi.a
  SWIFT_DIR="bindings/bdk-swift"
  XCFRAMEWORK_NAME="bdkFFI"
  XCFRAMEWORK_ROOT="$SWIFT_DIR/$XCFRAMEWORK_NAME.xcframework"

  # Cleanup prior build
  rm -rf "$XCFRAMEWORK_ROOT"

  # Common files
  XCFRAMEWORK_COMMON="$XCFRAMEWORK_ROOT/common/$XCFRAMEWORK_NAME.framework"
  mkdir -p "$XCFRAMEWORK_COMMON/Modules"
  cp "$SWIFT_DIR/module.modulemap" "$XCFRAMEWORK_COMMON/Modules/"
  mkdir -p "$XCFRAMEWORK_COMMON/Headers"
  cp "$SWIFT_DIR/bdkFFI-umbrella.h" "$XCFRAMEWORK_COMMON/Headers"
  cp "$SWIFT_DIR/bdkFFI.h" "$XCFRAMEWORK_COMMON/Headers"
  #mkdir -p "$XCFRAMEWORK_COMMON/$XCFRAMEWORK_NAME"
  #cp "$SWIFT_DIR/bdk.swift" "$XCFRAMEWORK_COMMON/$XCFRAMEWORK_NAME"

  # iOS hardware
  mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64"
  cp -R "$XCFRAMEWORK_COMMON" "$XCFRAMEWORK_ROOT/ios-arm64/$XCFRAMEWORK_NAME.framework"
  cp "$TARGET_DIR/aarch64-apple-ios/$BUILD_PROFILE/$LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64/$XCFRAMEWORK_NAME.framework/$XCFRAMEWORK_NAME"

  # iOS simulator, currently x86_64 only (need to make fat binary to add M1)
  mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator"
  cp -R "$XCFRAMEWORK_COMMON" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$XCFRAMEWORK_NAME.framework"
  cp "$TARGET_DIR/x86_64-apple-ios/$BUILD_PROFILE/$LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$XCFRAMEWORK_NAME.framework/$XCFRAMEWORK_NAME"

  # Set up the metadata for the XCFramework as a whole.
  cp "$SWIFT_DIR/Info.plist" "$XCFRAMEWORK_ROOT/Info.plist"
  # TODO add license info

  # Remove common
  rm -rf "$XCFRAMEWORK_ROOT/common"

  # Zip it all up into a bundle for distribution.
  (cd $SWIFT_DIR; zip -9 -r "$XCFRAMEWORK_NAME.xcframework.zip" "$XCFRAMEWORK_NAME.xcframework")

  # Cleanup build
  # rm -rf "$XCFRAMEWORK_ROOT"
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
      -s) build_swift ;;
      -h) help ;;
      *) echo "Option $1 not recognized" ;;
    esac
    shift
  done
fi
