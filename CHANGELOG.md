# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

## [0.1.0-0.2.0]
### Added
- Update BDK to version `0.20.0`
- APIs Added
    - TxBuilder.add_data(data)
    - Wallet.list_unspent() returns a list of local UTXOs
    - Add coin control methods on TxBuilder

### Fixed
- Tox tests

## [0.0.5-0.1.0]
### Added
- Community related files (bug report, feature request, and pull request templates)
- Changelog
- MIT and Apache 2.0 licenses
- Update BDK to version `0.19.0`
- Add `BumpFeeTxBuilder` to bump the fee on an unconfirmed tx created by the Wallet
- Add `Blockchain.broadcast` function (does not return anything)
- Add TxBuilder for creating new spending `PartiallySignedBitcoinTransaction` 
- Add TxBuilder `add_recipient`, `fee_rate`, and `finish` functions 
- Add TxBuilder `drain_wallet` and `drain_to` functions 
- Update generate cli tool to generate all binding languages and rename to bdk-ffi-bindgen 
- Add sqlite database support
- Fix memory database configuration enum, remove junk field 
- Remove hard coded sync progress value (was always returning 21.0)
- Fix tests and tox build workflow

## [0.0.1-0.0.5]
### Added
- Readme
- Rust binary to build
- CI workflow for tessting PRs
- Publishing workflow using GitHub Actions
- Basic examples
- Release on PyPI

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/  
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html  
[unreleased]: https://github.com/bitcoindevkit/bdk-python/compare/v0.0.5...HEAD  
[0.0.1-0.0.5]: https://github.com/bitcoindevkit/bdk-python/compare/58f189f987cc644a1d86e965623c8f50904588ad...v0.0.5  
[0.0.5-0.1.0]: https://github.com/bitcoindevkit/bdk-python/compare/v0.0.5...v0.1.0
[0.1.0-0.2.0]: https://github.com/bitcoindevkit/bdk-python/compare/v0.1.0...v0.2.0