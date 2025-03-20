# bdk-android

This project builds an .aar package for the Android platform that provide Kotlin language bindings for the [BDK] libraries. The Kotlin language bindings are created by the [`bdk-ffi`] project which is included in the root of this repository.

## How to Use

To use the Kotlin language bindings for BDK in your Android project add the following to your gradle dependencies:

```kotlin
repositories {
    mavenCentral()
}

dependencies { 
    implementation("org.bitcoindevkit:bdk-android:<version>")
}
```

### Snapshot releases

To use a snapshot release, specify the snapshot repository url in the `repositories` block and use the snapshot version in the `dependencies` block:
```kotlin
repositories {
    maven("https://s01.oss.sonatype.org/content/repositories/snapshots/")
}

dependencies { 
    implementation("org.bitcoindevkit:bdk-android:<version-SNAPSHOT>")
}
```

### Example Projects

* [Devkit Wallet](https://github.com/bitcoindevkit/devkit-wallet)
* [Padawan Wallet](https://github.com/thunderbiscuit/padawan-wallet)

### How to build

_Note that Kotlin version `2.1.10` or later is required to build the library._

1. Clone this repository.
```shell
git clone https://github.com/bitcoindevkit/bdk-ffi
```
2. Install Android SDK and Build-Tools for API level 30+
3. Setup `ANDROID_SDK_ROOT` and `ANDROID_NDK_ROOT` path variables which are required by the build tool. Note that currently, NDK version 25.2.9519653 or above is required. For example:
```shell
# macOS
export ANDROID_SDK_ROOT=~/Library/Android/sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.2.9519653

# Linux
export ANDROID_SDK_ROOT=/usr/local/lib/android/sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.2.9519653
```
4. Build Kotlin bindings
```sh
# build Android library
cd bdk-android
bash ./scripts/build-<your-local-architecture>.sh
```
5. Start android emulator and run tests
```sh
./gradlew connectedAndroidTest
```

## How to publish to your local Maven repo

```shell
cd bdk-android
./gradlew publishToMavenLocal -P localBuild
```

Note that the command above assumes you don't need the local libraries to be signed. If you do wish to sign them, simply set your `~/.gradle/gradle.properties` signing key values like so:
```properties
signing.gnupg.keyName=<YOUR_GNUPG_ID>
signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
```

and use the `publishToMavenLocal` task without the `localBuild` flag:
```shell
./gradlew publishToMavenLocal
```

## Known issues

### JNA dependency

Depending on the JVM version you use, you might not have the JNA dependency on your classpath. The exception thrown will be

```shell
class file for com.sun.jna.Pointer not found
```

The solution is to add JNA as a dependency like so:

```kotlin
dependencies {
    implementation("net.java.dev.jna:jna:5.12.1")
}
```

### x86 emulators

For some older versions of macOS, Android Studio will recommend users install the x86 version of the emulator by default. This will not work with the bdk-android library, as we do not support 32-bit architectures. Make sure you install an x86_64 emulator to work with bdk-android.

[BDK]: https://github.com/bitcoindevkit/
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
