# bdk-jvm
This project builds a .jar package for the JVM platform that provide Kotlin language bindings for the [`bdk`] library. The Kotlin language bindings are created by the `bdk-ffi` project which is included in the root of this repository.

## How to Use
To use the Kotlin language bindings for [`bdk`] in your JVM project add the following to your gradle dependencies:
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

val externalDescriptor = Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)", Network.TESTNET)
val internalDescriptor = Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)", Network.TESTNET)

val databaseConfig = DatabaseConfig.Memory

val blockchainConfig = BlockchainConfig.Electrum(
    ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u, true)
)
val wallet = Wallet(externalDescriptor, internalDescriptor, Network.TESTNET, databaseConfig, blockchainConfig)
val newAddress = wallet.getAddress(AddressIndex.LastUnused)
```

### Snapshot releases
To use a snapshot release, specify the snapshot repository url in the `repositories` block and use the snapshot version in the `dependencies` block:
```kotlin
repositories {
    maven("https://s01.oss.sonatype.org/content/repositories/snapshots/")
}

dependencies { 
    implementation("org.bitcoindevkit:bdk-jvm:<version-SNAPSHOT>")
}
```

## Example Projects
* [Tatooine Faucet](https://github.com/thunderbiscuit/tatooine)

## How to build
_Note that Kotlin version `1.9.23` or later is required to build the library._
1. Install JDK 11. It must be version 11 (not 17), otherwise it won't build. For example, with SDKMAN!:
```shell
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"
sdk install java 11.0.19-tem
```
2. Install Rust (note that we are currently building using Rust stable):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```
3. Clone this repository.
```shell
git clone https://github.com/bitcoindevkit/bdk-ffi
```
4. If building on macOS install required intel and m1 jvm targets
```sh
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```
5. Build kotlin bindings
```sh
./gradlew buildJvmLib
```

## How to publish to your local Maven repo
```shell
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

## Known issues
## JNA dependency
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

[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
