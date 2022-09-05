# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Breaking Changes
  - Rename `get_network()` method on `Wallet` interface to `network()` [#185]
  - Rename `get_transactions()` method on `Wallet` interface to `list_transactions()` [#185]
  - Remove `generate_extended_key`, returned ExtendedKeyInfo [#154]
  - Remove `restore_extended_key`, returned ExtendedKeyInfo [#154]
  - Remove dictionary `ExtendedKeyInfo {mnenonic, xprv, fingerprint}` [#154]
  - Remove interface `Transaction` [#190]
  - Changed `Wallet` interface `list_transaction()` to return array of `TransactionDetails` [#190]
- APIs Added [#154]
  - `generate_mnemonic()`, returns string mnemonic
  - `interface DescriptorSecretKey`
    - `new(Network, string_mnenoinc, password)`, contructs DescriptorSecretKey
    - `derive(DerivationPath)`, derives and returns child DescriptorSecretKey
    - `extend(DerivationPath)`, extends and returns DescriptorSecretKey
    - `as_public()`, returns DescriptorSecretKey as DescriptorPublicKey
    - `as_string()`, returns DescriptorSecretKey as String
  - `interface DescriptorPublicKey`
    - `derive(DerivationPath)` derives and returns child DescriptorPublicKey
    - `extend(DerivationPath)` extends and returns DescriptorPublicKey
    - `as_string()` returns DescriptorPublicKey as String
  - Add to `interface Blockchain` the `get_height()` and `get_block_hash()` methods [#184]
  - Add to `interface TxBuilder`  the `set_recipients(recipient: Vec<AddressAmount>)` method [#186]
  - Add to `dictionary TransactionDetails` the `confirmation_time` field [#190]
- Interfaces Added [#154]
  - `DescriptorSecretKey`
  - `DescriptorPublicKey`
  - `DerivationPath`

[#154]: https://github.com/bitcoindevkit/bdk-ffi/pull/154
[#184]: https://github.com/bitcoindevkit/bdk-ffi/pull/184
[#185]: https://github.com/bitcoindevkit/bdk-ffi/pull/185

## [v0.8.0]
- Update BDK to version 0.20.0 [#169]
- APIs Added
  - `TxBuilder.add_data(data: Vec<u8>)` [#163]
  - `Wallet.list_unspent()` returns `Vec<LocalUtxo>` [#158]
  - Add coin control methods on TxBuilder [#164]

[#163]: https://github.com/bitcoindevkit/bdk-ffi/pull/163
[#158]: https://github.com/bitcoindevkit/bdk-ffi/pull/158
[#164]: https://github.com/bitcoindevkit/bdk-ffi/pull/164
[#169]: https://github.com/bitcoindevkit/bdk-ffi/pull/169
[#190]: https://github.com/bitcoindevkit/bdk-ffi/pull/190

## [v0.7.0]
- Update BDK to version 0.19.0
  - fixes sqlite-db issue causing wrong balance
  - adds experimental taproot descriptor and PSBT support
- APIs Removed 
  - `Wallet.get_new_address()`, returned String, [#137] 
  - `Wallet.get_last_unused_address()`, returned String [#137]
- APIs Added
  - `Wallet.get_address(AddressIndex)`, returns `AddressInfo` [#137]
- APIs Changed
  - `Wallet.sign(PartiallySignedBitcoinTransaction)` now returns a bool, true if finalized [#161]

[#137]: https://github.com/bitcoindevkit/bdk-ffi/pull/137
[#161]: https://github.com/bitcoindevkit/bdk-ffi/pull/161

## [v0.6.0]
- Update BDK to version 0.18.0
- Add BumpFeeTxBuilder to bump the fee on an unconfirmed tx created by the Wallet
- Change TxBuilder.build() to TxBuilder.finish() to align with bdk function name 

## [v0.5.0]
- Fix Wallet.broadcast function, now returns a tx id as a hex string
- Remove creating a new spending Transaction via the PartiallySignedBitcoinTransaction constructor
- Add TxBuilder for creating new spending PartiallySignedBitcoinTransaction
- Add TxBuilder .add_recipient, .fee_rate, and .build functions
- Add TxBuilder .drain_wallet and .drain_to functions
- Update generate cli tool to generate all binding languages and rename to bdk-ffi-bindgen

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

[unreleased]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.8.0...HEAD
[v0.8.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.0.0...v0.2.0
