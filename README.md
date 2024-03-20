# Native language bindings for BDK

<p>
    <a href="https://github.com/bitcoindevkit/bdk-ffi/blob/master/LICENSE"><img alt="MIT or Apache-2.0 Licensed" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg"/></a>
    <a href="https://github.com/bitcoindevkit/bdk-ffi/actions?query=workflow%3ACI"><img alt="CI Status" src="https://github.com/bitcoindevkit/bdk-ffi/workflows/CI/badge.svg"></a>
    <a href="https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html"><img alt="Rustc Version 1.61.0+" src="https://img.shields.io/badge/rustc-1.61.0%2B-lightgrey.svg"/></a>
    <a href="https://discord.gg/d7NkDKm"><img alt="Chat on Discord" src="https://img.shields.io/discord/753336465005608961?logo=discord"></a>
</p>

## 🚨 Warning 🚨
The `master` branch of this repository is being migrated to the [bdk 1.0 API](https://github.com/bitcoindevkit/bdk) and is incomplete. For production-ready libraries, use the [`0.31.X`](https://github.com/bitcoindevkit/bdk-ffi/tree/release/0.30) releases.

## Readme
The workspace in this repository creates the `libbdkffi` multi-language library for the Rust-based 
[bdk] library from the [Bitcoin Dev Kit] project.

Each supported language and the platform(s) it's packaged for has its own directory. The Rust code in this project is in the bdk-ffi directory and is a wrapper around the [bdk] library to expose its APIs in a uniform way using the [mozilla/uniffi-rs] bindings generator for each supported target language.

## Supported target languages and platforms
The below directories (a separate repository in the case of bdk-swift) include instructions for using, building, and publishing the native language binding for [bdk] supported by this project.

| Language | Platform              | Published Package             | Building Documentation | API Docs              |
| -------- |-----------------------|-------------------------------|------------------------|-----------------------|
| Kotlin   | JVM                   | [bdk-jvm (Maven Central)]     | [Readme bdk-jvm]       | [Kotlin JVM API Docs] |
| Kotlin   | Android               | [bdk-android (Maven Central)] | [Readme bdk-android]   | [Android API Docs]    |
| Swift    | iOS, macOS            | [bdk-swift (GitHub)]          | [Readme bdk-swift]     |                       |
| Python   | linux, macOS, Windows | [bdk-python (PyPI)]           | [Readme bdk-python]    |                       |

## Minimum Supported Rust Version (MSRV)

This library should compile with any combination of features with Rust 1.74.0.

## Contributing

### Adding new structs and functions
See the [UniFFI User Guide](https://mozilla.github.io/uniffi-rs/)

#### For pass by value objects
1. Create new rust struct with only fields that are supported UniFFI types
2. Update mapping `bdk.udl` file with new `dictionary`

#### For pass by reference values
1. Create wrapper rust struct/impl with only fields that are `Sync + Send`
2. Update mapping `bdk.udl` file with new `interface`

## Goals
1. Language bindings should feel idiomatic in target languages/platforms
2. Adding new targets should be easy 
3. Getting up and running should be easy 
4. Contributing should be easy 
5. Get it right, then automate

## Using the libraries
### bdk-android
```kotlin
// build.gradle.kts
repositories {
    mavenCentral()
}
dependencies { 
    implementation("org.bitcoindevkit:bdk-android:<version>")
}
```

### bdk-jvm
```kotlin
// build.gradle.kts
repositories {
    mavenCentral()
}
dependencies { 
    implementation("org.bitcoindevkit:bdk-jvm:<version>")
}
```

_Note:_ We also publish snapshot versions of bdk-jvm and bdk-android. See the specific readmes for instructions on how to use those.

### bdk-python
```shell
pip3 install bdkpython
```

### bdk-swift
Add bdk-swift to your dependencies in XCode.

## Developing language bindings using uniffi-rs
If you are interested in better understanding the base structure we use here in order to build your own Rust-to-Kotlin/Swift/Python language bindings, check out the [uniffi-bindings-template](https://github.com/thunderbiscuit/uniffi-bindings-template) repository. We maintain it as an example and starting point for other projects that wish to leverage the tech stack used in producing the BDK language bindings.

## Verifying Signatures
Both libraries and all their corresponding artifacts are signed with a PGP key you can find in the
root of this repository. To verify the signatures follow the below steps:

1. Import the PGP key in your keyring.
```shell
# Navigate to the root of the repository and import the ./PGP-BDK-BINDINGS.asc public key
gpg --import ./PGP-BDK-BINDINGS.asc
    
# Alternatively, you can import the key directly from a public key server
gpg --keyserver keyserver.ubuntu.com --receive-key 2768C43E8803C6A3
    
# Verify that the correct key was imported
gpg --list-keys
# You should see the below output
pub   ed25519 2022-08-31 [SC]
    88AD93AC4589FD090FF3B8D12768C43E8803C6A3
uid           [ unknown] bitcoindevkit-bindings <bindings@bitcoindevkit.org>
sub   cv25519 2022-08-31 [E]
```

2. Download the binary artifacts and corresponding signature files.
- from [bdk-jvm]
    - `bdk-jvm-<version>.jar`
    - `bdk-jvm-<version>.jar.asc`
- from [bdk-android]
    - `bdk-android-<version>.aar`
    - `bdk-android-<version>.aar.asc`

3. Verify the signatures.
```shell
gpg --verify bdk-jvm-<version>.jar.asc 
gpg --verify bdk-android-<version>.aar.asc

# you should see a "Good signature" result
gpg: Good signature from "bitcoindevkit-bindings <bindings@bitcoindevkit.org>" [unknown]
```

### PGP Metadata
Full key ID: `88AD 93AC 4589 FD09 0FF3 B8D1 2768 C43E 8803 C6A3`  
Fingerprint: `2768C43E8803C6A3`  
Name: `bitcoindevkit-bindings`  
Email: `bindings@bitcoindevkit.org`

## Thanks
This project is made possible thanks to the wonderful work by the [mozilla/uniffi-rs] team.

[Kotlin]: https://kotlinlang.org/
[Android Studio]: https://developer.android.com/studio/
[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
["Getting Started (Developer)"]: https://github.com/bitcoindevkit/bdk-ffi#getting-started-developer
[bdk-jvm]: https://search.maven.org/artifact/org.bitcoindevkit/bdk-jvm/0.11.0/jar
[bdk-android]: https://search.maven.org/artifact/org.bitcoindevkit/bdk-android/0.11.0/aar
[bdk-jvm (Maven Central)]: https://central.sonatype.dev/artifact/org.bitcoindevkit/bdk-jvm/0.11.0
[bdk-android (Maven Central)]: https://central.sonatype.dev/artifact/org.bitcoindevkit/bdk-android/0.11.0
[bdk-swift (GitHub)]: https://github.com/bitcoindevkit/bdk-swift
[bdk-python (PyPI)]: https://pypi.org/project/bdkpython/
[mozilla/uniffi-rs]: https://github.com/mozilla/uniffi-rs
[bdk]: https://github.com/bitcoindevkit/bdk
[Bitcoin Dev Kit]: https://github.com/bitcoindevkit
[uniffi-rs]: https://github.com/mozilla/uniffi-rs
[Readme bdk-jvm]: https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-jvm
[Readme bdk-android]: https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-android
[Readme bdk-swift]: https://github.com/bitcoindevkit/bdk-swift  
[Readme bdk-python]: https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-python
[Kotlin JVM API Docs]: https://bitcoindevkit.org/jvm/
[Android API Docs]: https://bitcoindevkit.org/android/
