# bdk-jvm

This project builds a .jar package for the JVM platform that provides Kotlin language bindings for the [BDK] libraries. The Kotlin language bindings are created by the `bdk-ffi` project which is included in the root of this repository.

## How to Use

To use the Kotlin language bindings for BDK in your JVM project add the following to your gradle dependencies:

```kotlin
repositories {
    mavenCentral()
}

dependencies {
    implementation("org.bitcoindevkit:bdk-jvm:<version>")
}
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

- [Tatooine Faucet](https://github.com/thunderbiscuit/tatooine)
- [Godzilla Wallet](https://github.com/thunderbiscuit/godzilla-wallet)

## How to build

_Note that Kotlin version `1.9.23` or later is required to build the library._
1. Install JDK 17. For example, with SDKMAN!:
```shell
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"
sdk install java 17.0.2-tem
```
2. Build kotlin bindings
```sh
bash ./scripts/build-<your-local-architecture>.sh
```

## How to publish to your local Maven repo

```shell
cd bdk-jvm
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

[BDK]: https://github.com/bitcoindevkit/
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
