# Language Bindings for BDK

<p>
  <a href="https://github.com/bitcoindevkit/bdk-ffi/blob/master/LICENSE"><img alt="MIT or Apache-2.0 Licensed" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg"/></a>
  <a href="https://discord.gg/d7NkDKm"><img alt="Chat on Discord" src="https://img.shields.io/discord/753336465005608961?logo=discord"></a>
</p>

The code in this repository creates a library ready for export to other languages using [uniffi-rs] for the Rust-based [bdk_wallet] library from the [Bitcoin Dev Kit] project.

Each supported language (Kotlin for Android and Swift for iOS) and the platforms it's packaged for has its own directory. The Rust code in this project is in the bdk-ffi directory and is a wrapper around the [bdk_wallet] library to expose its APIs in a uniform way using the [mozilla/uniffi-rs] bindings generator for each supported target language.

## Supported target languages and platforms

The below directories include instructions for building and using the language binding for [bdk_wallet] supported by this project.

| Language | Platform              | Published Package | Building Documentation | API Docs              |
| -------- |-----------------------|-------------------|------------------------|-----------------------|
| Kotlin   | Android               | [bdk-android]     | [Readme bdk-android]   | [Android API Docs]    |
| Swift    | iOS, macOS            | [bdk-swift]       | [Readme bdk-swift]     |                       |

## Other supported languages maintained in separate repositories

The `bdk-ffi` codebase in this repository can be used to produce language bindings for more than Swift and Kotlin. Some of these require the use of uniffi 3rd party plugins and some not. Below are some of the libraries that use the bdk-ffi API, but are maintained separately. Refer to their individual READMEs for information on their state of production-readiness.

| Language | Platform              | Published Package         | Repository   | API Docs              |
| -------- |-----------------------|---------------------------|--------------|-----------------------|
| Kotlin   | JVM                   | [bdk-jvm (Maven Central)] | [bdk-jvm]    |                       |
| Python   | Linux, macOS, Windows | [bdk-python (PyPI)]       | [bdk-python] |                       |

## Building and testing the libraries

If you are familiar with the build tools for the specific languages you wish to build the libraries for, you can use their normal build/test workflows. We also include some [just](https://just.systems/) files to simplify the work across different languages. If you have the `just` tool installed on your system, you can simply call the commands defined in the justfiles, for example:

```sh
cd bdk-android
just build
just test
just publish-local
```

## Contributing

To add new structs and functions, see the [UniFFI User Guide](https://mozilla.github.io/uniffi-rs/) and the [uniffi-examples](https://thunderbiscuit.github.io/uniffi-examples/) repository.

## Goals

1. Language bindings should feel idiomatic in target languages/platforms
2. Adding new targets should be easy 
3. Getting up and running should be easy 
4. Contributing should be easy 
5. Get it right, then automate

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

[bdk-jvm]: https://github.com/bitcoindevkit/bdk-jvm
[bdk-jvm (Maven Central)]: https://central.sonatype.com/artifact/org.bitcoindevkit/bdk-jvm/
[bdk-android]: https://central.sonatype.com/artifact/org.bitcoindevkit/bdk-android/
[bdk-swift]: https://github.com/bitcoindevkit/bdk-swift
[bdk-python]: https://github.com/bitcoindevkit/bdk-python
[bdk-python (PyPI)]: https://pypi.org/project/bdk-python
[mozilla/uniffi-rs]: https://github.com/mozilla/uniffi-rs
[bdk_wallet]: https://github.com/bitcoindevkit/bdk_wallet
[Bitcoin Dev Kit]: https://github.com/bitcoindevkit
[uniffi-rs]: https://github.com/mozilla/uniffi-rs
[Readme bdk-android]: https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-android
[Readme bdk-swift]: https://github.com/bitcoindevkit/bdk-swift
[Android API Docs]: https://bitcoindevkit.org/android
