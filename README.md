# Native language bindings for BDK

This repository contains source code for generating native language bindings for the rust based 
[bdk] library which is the central artifact of the [Bitcoin Dev Kit] project.

Each supported language has it's own repository that includes this project as a [git submodule]. 
The rust code in this project is a wrapper around the [bdk] library to expose it's APIs in a 
uniform way using the [mozilla/uniffi-rs] bindings generator for each supported target language.

## Supported target languages and platforms

The below repositories include instructions for using, building, and publishing the native 
language binding for [bdk] supported by this project.

| Language | Platform     | Repository   |
| -------- | ------------ | ------------ |
| Kotlin   | jvm          | [bdk-kotlin] |
| Kotlin   | android      | [bdk-kotlin] |
| Swift    | iOS, macOS   | [bdk-swift]  |
| Python   | linux, macOS | [bdk-python] |

[bdk]: https://github.com/bitcoindevkit/bdk
[Bitcoin Dev Kit]: https://github.com/bitcoindevkit
[git submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules
[uniffi-rs]: https://github.com/mozilla/uniffi-rs

[bdk-kotlin]: https://github.com/bitcoindevkit/bdk-kotlin
[bdk-swift]: https://github.com/bitcoindevkit/bdk-swift
[bdk-python]: https://github.com/thunderbiscuit/bdk-python

## Contributing

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

This project is made possible thanks to the wonderful work by the [mozilla/uniffi-rs] team.

[mozilla/uniffi-rs]: https://github.com/mozilla/uniffi-rs
