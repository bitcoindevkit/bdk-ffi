# Foreign language bindings for BDK (bdk-ffi)

This repository contains source code for generating foreign language bindings
for the rust library bdk for the Bitcoin Dev Kit (BDK) project.

## Supported target languages and platforms

| Language | Platform | Status |
| --- | --- | --- |
| Kotlin | JVM | WIP |
| Kotlin | Android | WIP |
| Swift | iOS | WIP |

## Getting Started

This project uses rust. A basic knowledge of the rust ecosystem is helpful.

### General
1. Install `uniffi-bindgen`
    ```sh
    cargo install uniffi_bindgen
    ```
1. See the [UniFFI User Guide](https://mozilla.github.io/uniffi-rs/) for more info

### Kotlin Bindings for JVM (OSX / Linux)

1. Install required targets
    ```sh
      rustup target add x86_64-apple-darwin x86_64-unknown-linux-gnu
    ```
1. Build kotlin (JVM) bindings
    ```sh
      ./build.sh -k
    ```
1. Generated kotlin bindings are available at `/bindings/bdk-kotlin/`
1. A demo app is available at `/bindings/bdk-kotlin/demo/`. It uses stdin for
inputs and can be run from gradle.
    ```sh
    cd bindings/bdk-kotlin
    ./gradlew :demo:run
    ```

### Kotlin bindings for Android

1. Install required targets
    ```sh
    rustup target add x86_64-linux-android aarch64-linux-android
    armv7-linux-androideabi i686-linux-android
    ```
1. Install Android SDK and Build-Tools for API level 30+
1. Setup `$ANDROID_NDK_HOME` and `$ANDROID_SDK_ROOT` path variables (which are
required by the build scripts)
1. Build kotlin (Android) bindings
    ```sh
    ./build.sh -a
    ```
2. A demo android app is available at [notmandatory/bdk-sample-app](https://github.com/notmandatory/bitcoindevkit-android-sample-app/tree/upgrade-to-bdk-ffi/)

### Swift bindings for iOS

1. Install the latest version of xcode, download and install the advanced tools.
1. Ensure Swift is installed
1. Install required targets
    ```sh
    rustup target add aarch64-apple-ios x86_64-apple-ios
    ```
1. Build swift (iOS) bindings
    ```sh
    ./build.sh -s
    ```
1. Example iOS app can be found in `/examples/iOS` which can be run by xcode.

## Notes

### Adding new structs and functions

See the [UniFFI User Guide](https://mozilla.github.io/uniffi-rs/)

#### For pass by value objects

1. create new rust struct with only fields that are supported UniFFI types
1. update mapping `bdk.udl` file with new `dictionary`

#### For pass by reference values 

1. create wrapper rust struct/impl with only fields that are `Sync + Send`
1. update mapping `bdk.udl` file with new `interface`

## Goals

1. Language bindings should feel idiomatic in target languages/platforms
1. Adding new targets should be easy
1. Getting up and running should be easy
1. Contributing should be easy
1. Get it right, then automate

## Thanks

This project is made possible thanks to the wonderful work on [mozilla/uniffi-rs](https://github.com/mozilla/uniffi-rs)
