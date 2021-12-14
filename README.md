# bdk-kotlin

This project builds .jar and .aar packages for the `jvm` and `android` platforms that provide 
[Kotlin] language bindings for the [`bdk`] library. The Kotlin language bindings are created by the 
[`bdk-ffi`] project which is included as a git submodule of this repository.

## How to Use

To use the Kotlin language bindings for [`bdk`] in your `jvm` or `android` project add the 
following to your gradle dependencies:
```groovy
repositories {
    mavenCentral()
}

dependencies {
  
  // for jvm
  implementation 'org.bitcoindevkit:bdk-jvm:0.2.0'
  // OR for android
  implementation 'org.bitcoindevkit:bdk-android:0.2.0'
  
}

```

You may then import and use the `org.bitcoindevkit` library in your Kotlin code. For example:

```kotlin
import org.bitcoindevkit.*

// ...

val descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
val db = DatabaseConfig.Memory("")

val client =
  BlockchainConfig.Electrum(
    ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u)
  )
val wallet = OnlineWallet(descriptor, null, Network.TESTNET, db, client)
val newAddress = wallet.getNewAddress()
```

### How to build

1. Install required targets
    ```sh
    rustup target add x86_64-linux-android aarch64-linux-android armv7-linux-androideabi i686-linux-android
    ```
1. Install Android SDK and Build-Tools for API level 30+
1. Setup `$ANDROID_SDK_ROOT` and `$ANDROID_NDK_HOME` path variables (which are required by the 
   build scripts), for example:
    ```shell
    export ANDROID_SDK_ROOT=~/Android/Sdk
    export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/21.3.6528147    
    ```
1. Build kotlin bindings
    ```sh
    ./build.sh
    ```

### How to publish to your local maven repo

1. Set your `~/.gradle/gradle.properties` signing key values
   ```properties
   signing.gnupg.keyName=<YOUR_GNUPG_ID>
   signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
   ```
1. Publish   
   ```shell
   ./gradlew :jvm:publishReleasePublicationToMavenLocal
   ./gradlew :android:publishReleasePublicationToMavenLocal
   ```

### How to publish to maven central (project maintainers only)

1. Set your `~/.gradle/gradle.properties` signing key values and SONATYPE login
   ```properties
   signing.gnupg.keyName=<YOUR_GNUPG_ID>
   signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
   
   ossrhUserName=<YOUR_SONATYPE_USERNAME>
   ossrhPassword=<YOUR_SONATYPE_PASSWORD>
   ```
1. Publish
   ```shell
   ./gradlew :jvm:publishToSonatype closeAndReleaseSonatypeStagingRepository
   ./gradlew :android:publishToSonatype closeAndReleaseSonatypeStagingRepository
   ```

<!-- TODO A demo android app is available at [notmandatory/bdk-sample-app](https://github.com/notmandatory/bitcoindevkit-android-sample-app/tree/upgrade-to-bdk-ffi/) -->

[Kotlin]: https://kotlinlang.org/
[Android Studio]: https://developer.android.com/studio/
[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
["Getting Started (Developer)"]: https://github.com/bitcoindevkit/bdk-ffi#getting-started-developer
