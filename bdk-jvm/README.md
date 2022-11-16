# bdk-android
This project builds a .jar package for the `jvm` platform that provide [Kotlin] language bindings for the [`bdk`] library. The Kotlin language bindings are created by the [`bdk-ffi`] project which is included in the root of this repository.

## How to Use
To use the Kotlin language bindings for [`bdk`] in your `jvm` project add the following to your gradle dependencies:
```kotlin
repositories {
    mavenCentral()
}

dependencies {
  implementation("org.bitcoindevkit:bdk-jvm:<version>")
}
```

You may then import and use the `org.bitcoindevkit` library in your Kotlin code. For example:
```kotlin
import org.bitcoindevkit.*

// ...

val externalDescriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
val internalDescriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

val databaseConfig = DatabaseConfig.Memory

val blockchainConfig =
  BlockchainConfig.Electrum(
    ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u)
  )
val wallet = Wallet(externalDescriptor, internalDescriptor, Network.TESTNET, databaseConfig, blockchainConfig)
val newAddress = wallet.getNewAddress()
```

### Example Projects
* [Tatooine Faucet](https://github.com/thunderbiscuit/tatooine)

### How to build
_Note that Kotlin version `1.6.10` or later is required to build the library._

1. Clone this repository.
```shell
git clone https://github.com/bitcoindevkit/bdk-ffi
```
2. Follow the "General" bdk-ffi ["Getting Started (Developer)"] instructions.
3. If building on macOS install required intel and m1 jvm targets
```sh
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```
4. Build kotlin bindings
 ```sh
 # build JVM library
 ./gradlew buildJvmLib
 ```

## How to publish
### Publish to your local maven repo
```shell
# bdk-jvm
cd bdk-jvm
./gradlew publishToMavenLocal --exclude-task signMavenPublication
```

Note that the commands assume you don't need the local libraries to be signed. If you do wish to sign them, simply set your `~/.gradle/gradle.properties` signing key values like so:
```properties
signing.gnupg.keyName=<YOUR_GNUPG_ID>
signing.gnupg.passphrase=<YOUR_GNUPG_PASSPHRASE>
```

and use the `publishToMavenLocal` task without excluding the signing task:
```shell
./gradlew publishToMavenLocal
```
