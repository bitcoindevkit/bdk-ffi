# bdk-android
This project builds an .aar package for the Android platform that provide Kotlin language bindings for the [`bdk`] library. The Kotlin language bindings are created by the [`bdk-ffi`] project which is included in the root of this repository.

## How to Use
To use the Kotlin language bindings for [`bdk`] in your Android project add the following to your gradle dependencies:
```kotlin
repositories {
    mavenCentral()
}

dependencies { 
    implementation("org.bitcoindevkit:bdk-android:<version>")
}
```

You may then import and use the `org.bitcoindevkit` library in your Kotlin code like so. Note that this example is for the `0.30.0` release. For examples of the 1.0 API in the alpha releases, take a look at the tests [here](https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-android/lib/src/androidTest/kotlin/org/bitcoindevkit).
```kotlin
import org.bitcoindevkit.*

// ...

val externalDescriptor = Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)", Network.TESTNET)
val internalDescriptor = Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)", Network.TESTNET)

val esploraClient: EsploraClient = EsploraClient("https://esplora.testnet.kuutamo.cloud/")
val wallet: Wallet = Wallet(
    descriptor = externalDescriptor, 
    changeDescriptor = internalDescriptor, 
    persistenceBackendPath = "./bdkwallet.db", 
    network = Network.TESTNET
)
val update = esploraClient.fullScan(
    wallet = wallet,
    stopGap = 10uL,
    parallelRequests = 1uL
)

wallet.applyUpdate(update)

val newAddress = wallet.getAddress(AddressIndex.LastUnused)
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
* [bdk-kotlin-example-wallet](https://github.com/bitcoindevkit/bdk-kotlin-example-wallet)
* [Devkit Wallet](https://github.com/thunderbiscuit/devkit-wallet)
* [Padawan Wallet](https://github.com/thunderbiscuit/padawan-wallet)

### How to build
_Note that Kotlin version `1.9.23` or later is required to build the library._

1. Clone this repository.
```shell
git clone https://github.com/bitcoindevkit/bdk-ffi
```
2. Follow the "General" bdk-ffi ["Getting Started (Developer)"] instructions. 
3. Install Rust (note that we are currently building using Rust 1.73.0):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default 1.73.0
```
4. Install required targets
```sh
rustup target add x86_64-linux-android aarch64-linux-android armv7-linux-androideabi
```
5. Install Android SDK and Build-Tools for API level 30+
6. Setup `$ANDROID_SDK_ROOT` and `$ANDROID_NDK_ROOT` path variables (which are required by the
   build tool), for example (note that currently, NDK version 25.2.9519653 or above is required):
```shell
export ANDROID_SDK_ROOT=~/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_SDK_ROOT/ndk/25.2.9519653
```
7. Build kotlin bindings
 ```sh
 # build Android library
 cd bdk-android
 ./gradlew buildAndroidLib
 ```
1. Start android emulator and run tests
```sh
./gradlew connectedAndroidTest
```

## How to publish to your local Maven repo
```shell
cd bdk-android
./gradlew publishToMavenLocal -P localBuild
```

Note that the commands assume you don't need the local libraries to be signed. If you do wish to sign them, simply set your `~/.gradle/gradle.properties` signing key values like so:
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
    // ...
    implementation("net.java.dev.jna:jna:5.12.1")
}
```

### x86 emulators
For some older versions of macOS, Android Studio will recommend users install the x86 version of the emulator by default. This will not work with the bdk-android library, as we do not support 32-bit architectures. Make sure you install an x86_64 emulator to work with bdk-android.

[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
