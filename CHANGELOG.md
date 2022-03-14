# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.4.0]

- Add dual license MIT and Apache 2.0
- Add sqlite database support
- Fix memory database configuration enum, remove junk field

## [v0.3.1]

- Remove hard coded sync progress value (was always returning 21.0)

## [v0.3.0]

- Move bdk-kotlin bindings and ios example to separate repos
- Add bin to generate Python bindings
- Add `PartiallySignedBitcoinTransaction::deserialize` function as named constructor to decode from a string per [BIP 0174]
- Add `PartiallySignedBitcoinTransaction::serialize` function to encode to a string per [BIP 0174]
- Remove `PartiallySignedBitcoinTransaction.details` struct field

[BIP 0174]:https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki#encoding

## [v0.2.0]

[unreleased]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.0.0...v0.2.0
