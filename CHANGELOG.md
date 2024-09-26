# Changelog
Changelog information can also be found in each release's git tag (which can be viewed with `git tag -ln100 "v*"`), as well as on the [GitHub releases](https://github.com/bitcoindevkit/bdk-ffi/releases) page. See [DEVELOPMENT_CYCLE.md](DEVELOPMENT_CYCLE.md) for more details.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0-beta.2]
This release updates the bdk-ffi libraries to the latest bdk_wallet 1.0.0-beta.2 and related libraries (Esplora, Electrum, etc.), as well as uses the latest uniffi-rs library version 0.28.0. The releases now depend on [bitcoin-ffi] for the types that are exposed from the rust-bitcoin org. It also bumps the minimum supported Android API level to 24 (Android Nougat).

#### Added
  - SQLite persistence through bdk_sqlite [https://github.com/bitcoindevkit/bdk-ffi/pull/544]
  - The `Address`, `DescriptorSecretKey`, `DescriptorPublicKey`, `Mnemonic`, and `Descriptor` types now have the `toString()` method implemented on them by default [https://github.com/bitcoindevkit/bdk-ffi/pull/551]
  - `Address.from_script()` [https://github.com/bitcoindevkit/bdk-ffi/pull/554]
  - New `FromScriptError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New type `ChangeSet` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - Wallet constructors do not take a persistence path anymore [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - `Wallet.get_balance()` method renamed to `balance()` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - Add `add_global_xpubs()` method on `TxBuilder` [https://github.com/bitcoindevkit/bdk-ffi/pull/574]
  - Add `wallet.derivation_index` method on Wallet type [https://github.com/bitcoindevkit/bdk-ffi/pull/579]
  - Add `wallet.persist` method on Wallet type [https://github.com/bitcoindevkit/bdk-ffi/pull/582]
  - Add `Connection` type [https://github.com/bitcoindevkit/bdk-ffi/pull/582]

#### Changed
  - `AddressError` is replaced by `AddressParseError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New variants in `CalculateFeeError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New variants in `CreateTxError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New variants in `ParseAmountError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New variants in `SignerError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - New variants in `WalletCreationError` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - `Wallet.calculate_fee()` returns an `Amount` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]
  - Renamed `Transaction.txid()` to `Transaction.compute_txid()` [https://github.com/bitcoindevkit/bdk-ffi/pull/561]

#### Removed
  - flat file persistence [https://github.com/bitcoindevkit/bdk-ffi/pull/544]

[https://github.com/bitcoindevkit/bdk-ffi/pull/544]: https://github.com/bitcoindevkit/bdk-ffi/pull/544
[https://github.com/bitcoindevkit/bdk-ffi/pull/551]: https://github.com/bitcoindevkit/bdk-ffi/pull/551
[https://github.com/bitcoindevkit/bdk-ffi/pull/554]: https://github.com/bitcoindevkit/bdk-ffi/pull/554
[https://github.com/bitcoindevkit/bdk-ffi/pull/561]: https://github.com/bitcoindevkit/bdk-ffi/pull/561
[https://github.com/bitcoindevkit/bdk-ffi/pull/574]: https://github.com/bitcoindevkit/bdk-ffi/pull/574
[https://github.com/bitcoindevkit/bdk-ffi/pull/579]: https://github.com/bitcoindevkit/bdk-ffi/pull/579
[https://github.com/bitcoindevkit/bdk-ffi/pull/582]: https://github.com/bitcoindevkit/bdk-ffi/pull/582
[bitcoin-ffi]: https://github.com/bitcoindevkit/bitcoin-ffi

## [v1.0.0-alpha.11]
This release brings the latest alpha 11 release of the Rust bdk_wallet library, as well as the new Electrum client, the new memory wallet, and a whole lot of new types and APIs across the library. Also of note are the much simpler-to-use full_scan and sync workflows for syncing wallets.

Added:
  - `Amount` type [#533]
  - `TxIn` type [#536]
  - `Transaction.input()` method [#536]
  - `Transaction.output()` method [#536]
  - `Transaction.lock_time()` method [#536]
  - `Electrum` client [#535]
  - Memory wallet [#528]

[#528]: https://github.com/bitcoindevkit/bdk-ffi/pull/528
[#533]: https://github.com/bitcoindevkit/bdk-ffi/pull/533
[#535]: https://github.com/bitcoindevkit/bdk-ffi/pull/535
[#536]: https://github.com/bitcoindevkit/bdk-ffi/pull/536

## [v1.0.0-alpha.7]
This release brings back into the 1.0 API a number of APIs from the 0.31 release, and adds the new flat file persistence feature, as well as more fine-grain errors.

## [v1.0.0-alpha.2a]
This release is the first alpha release of the 1.0 API for the bindings libraries. Here is what is now available:
- Create and recover wallets using descriptors, including the four descriptor templates
- Sync a wallet using a blocking Esplora client
- Query the wallet for balance and addresses
- Create and sign transactions using the transaction builder
- Broadcast transactions

## [v0.31.0]
This release updates the bindings libraries to bdk version 0.29.0, updating rust-bitcoin to version 0.30.2.

- APIs Changed:
  - `BumpFeeTxBuilder.allow_shrinking()` now takes a `Script` as its argument [#443]
  - The `Address` constructor now takes a `Network` argument [#443]
  - The `Payload::PubkeyHash` and `Payload::ScriptHash` now have string arguments instead of byte arrays [#443]
- APIs Added:
  - The `Address` type now has the `is_valid_for_network()` method [#443]

[#443]: https://github.com/bitcoindevkit/bdk-ffi/pull/443

## [v0.30.0]
This release has a new API and a few internal optimizations and refactorings.

- APIs Added
  - Add BIP-86 descriptor templates [#388]

[#388]: https://github.com/bitcoindevkit/bdk-ffi/pull/388

## [v0.29.0]
This release has a number of new APIs, and adds support for Windows in bdk-jvm.

- Add support for Windows in bdk-jvm [#336]
- Add support for older version of Linux distros in bdk-jvm [#345]
- APIs added
  - Expose `is_mine()` method on the `Wallet` type [#355]
  - Expose `to_bytes()` method on the `Script` type [#369]

[#336]: https://github.com/bitcoindevkit/bdk-ffi/pull/336
[#345]: https://github.com/bitcoindevkit/bdk-ffi/pull/345
[#355]: https://github.com/bitcoindevkit/bdk-ffi/pull/355
[#369]: https://github.com/bitcoindevkit/bdk-ffi/pull/369

## [v0.28.0]
- Update BDK to version 0.28.0 [#341]
- Drop support of pypy releases of Python libraries [#351]
- Drop support for Python 3.6 and 3.7 [#351]
- Drop support for very old Linux versions that do not support the manylinux_2_17_x86_64 platform tag [#351]
- APIs changed:
  - Expose Address payload and network properties. [#325]
  - Add SignOptions to Wallet.sign() params. [#326]
  - address field on `AddressInfo` type is now of type `Address` [#333]
  - new PartiallySignedTransaction.json_serialize() function to get JSON serialized value of all PSBT fields. [#334]
  - Add from_script constructor to `Address` type [#337]

[#325]: https://github.com/bitcoindevkit/bdk-ffi/pull/325
[#326]: https://github.com/bitcoindevkit/bdk-ffi/pull/326
[#333]: https://github.com/bitcoindevkit/bdk-ffi/pull/333
[#334]: https://github.com/bitcoindevkit/bdk-ffi/pull/334
[#337]: https://github.com/bitcoindevkit/bdk-ffi/pull/337
[#341]: https://github.com/bitcoindevkit/bdk-ffi/pull/341
[#351]: https://github.com/bitcoindevkit/bdk-ffi/pull/351

## [v0.27.1]
- Update BDK to version 0.27.1 [#312]
- APIs changed
  - `PartiallySignedTransaction.extract_tx()` returns a `Transaction` instead of the transaction bytes. [#296]
  - `Blockchain.broadcast()` takes a `Transaction` instead of a `PartiallySignedTransaction`. [#296]
- APIs added
  - New `Transaction` structure that can be created from or serialized to consensus encoded bytes. [#296]
  - Add Wallet.get_internal_address() API [#304]
  - Add `AddressIndex::Peek(index)` and `AddressIndex::Reset(index)` APIs [#305]

[#296]: https://github.com/bitcoindevkit/bdk-ffi/pull/296
[#304]: https://github.com/bitcoindevkit/bdk-ffi/pull/304
[#305]: https://github.com/bitcoindevkit/bdk-ffi/pull/305
[#312]: https://github.com/bitcoindevkit/bdk-ffi/pull/312

## [v0.26.0]
- Update BDK to version 0.26.0 [#288]
- APIs changed
  - The descriptor and change_descriptor arguments on the wallet constructor now take a `Descriptor` instead of a `String`. [#260]
  - TxBuilder.drain_to() argument is now `Script` instead of address `String`. [#279]
- APIs added
  - Added RpcConfig, BlockchainConfig::Rpc, and Auth [#125]
  - Added Descriptor type in [#260] with the following methods:
    - Default constructor requires a descriptor in String format and a Network
    - new_bip44 constructor returns a Descriptor with structure pkh(key/44'/{0,1}'/0'/{0,1}/*)
    - new_bip44_public constructor returns a Descriptor with structure pkh(key/{0,1}/*)
    - new_bip49 constructor returns a Descriptor with structure sh(wpkh(key/49'/{0,1}'/0'/{0,1}/*))
    - new_bip49_public constructor returns a Descriptor with structure sh(wpkh(key/{0,1}/*))
    - new_bip84 constructor returns a Descriptor with structure wpkh(key/84'/{0,1}'/0'/{0,1}/*)
    - new_bip84_public constructor returns a Descriptor with structure wpkh(key/{0,1}/*)
    - as_string returns the public version of the output descriptor
    - as_string_private returns the private version of the output descriptor if available, otherwise return the public version

[#125]: https://github.com/bitcoindevkit/bdk-ffi/pull/125
[#260]: https://github.com/bitcoindevkit/bdk-ffi/pull/260
[#279]: https://github.com/bitcoindevkit/bdk-ffi/pull/279
[#288]: https://github.com/bitcoindevkit/bdk-ffi/pull/288

## [v0.25.0]
- Update BDK to version 0.25.0 [#272]
- APIs Added:
  - from_string() constructors now available on DescriptorSecretKey and DescriptorPublicKey [#247]

[#247]: https://github.com/bitcoindevkit/bdk-ffi/pull/247
[#272]: https://github.com/bitcoindevkit/bdk-ffi/pull/272

## [v0.11.0]
- Update BDK to version 0.24.0 [#221]
- APIs changed
  - The constructor on the DescriptorSecretKey type now takes a Mnemonic instead of a String.
- APIs added
  - Added Mnemonic struct [#219] with following methods:
    - new(word_count: WordCount) generates and returns Mnemonic with random entropy
    - from_string(mnemonic: String) converts string Mnemonic to Mnemonic type with error
    - from_entropy(entropy: Vec<u8>) generates and returns Mnemonic with given entropy
    - as_string() view Mnemonic as string
- APIs removed
  - generate_mnemonic(word_count: WordCount)

[#219]: https://github.com/bitcoindevkit/bdk-ffi/pull/219
[#221]: https://github.com/bitcoindevkit/bdk-ffi/pull/221

## [v0.10.0]
- Update BDK to version 0.23.0 [#204]
- Update uniffi-rs to latest version 0.21.0 [#216]
- Breaking Changes
  - Changed `TxBuilder.finish()` to return new `TxBuilderResult` [#209]
  - `TxBuilder.add_recipient()` now takes a `Script` instead of an `Address` [#192]
  - `AddressAmount` is now `ScriptAmount` [#192]
- APIs Added
  - Added `TxBuilderResult` with PSBT and TransactionDetails [#209]
  - `Address` and `Script` structs have been added [#192]
  - Add `PartiallySignedBitcoinTransaction.extract_tx()` function [#192]
  - Add `secret_bytes()` method on the `DescriptorSecretKey` [#199]
  - Add `PartiallySignedBitcoinTransaction.combine()` method [#200]

[#192]: https://github.com/bitcoindevkit/bdk-ffi/pull/192
[#199]: https://github.com/bitcoindevkit/bdk-ffi/pull/199
[#200]: https://github.com/bitcoindevkit/bdk-ffi/pull/200
[#204]: https://github.com/bitcoindevkit/bdk-ffi/pull/204
[#209]: https://github.com/bitcoindevkit/bdk-ffi/pull/209
[#216]: https://github.com/bitcoindevkit/bdk-ffi/pull/216

## [v0.9.0]
- Breaking Changes
  - Rename `get_network()` method on `Wallet` interface to `network()` [#185]
  - Rename `get_transactions()` method on `Wallet` interface to `list_transactions()` [#185]
  - Remove `generate_extended_key`, returned ExtendedKeyInfo [#154]
  - Remove `restore_extended_key`, returned ExtendedKeyInfo [#154]
  - Remove dictionary `ExtendedKeyInfo {mnenonic, xprv, fingerprint}` [#154]
  - Remove interface `Transaction` [#190]
  - Changed `Wallet` interface `list_transaction()` to return array of `TransactionDetails` [#190]
  - Update `bdk` dependency version to 0.22 [#193]
- APIs Added [#154]
  - `generate_mnemonic()`, returns string mnemonic
  - `interface DescriptorSecretKey`
    - `new(Network, string_mnenoinc, password)`, constructs DescriptorSecretKey
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
[#193]: https://github.com/bitcoindevkit/bdk-ffi/pull/193

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

[v1.0.0-beta.2]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.11...v1.0.0-beta.2
[v1.0.0-alpha.11]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.7...v1.0.0-alpha.11
[v1.0.0-alpha.7]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.2a...v1.0.0-alpha.7
[v1.0.0-alpha.2a]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.31.0...v1.0.0-alpha.2a
[v0.31.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.30.0...v0.31.0
[v0.30.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.29.0...v0.30.0
[v0.29.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.28.0...v0.29.0
[v0.28.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.27.1...v0.28.0
[v0.27.1]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.26.0...v0.27.1
[v0.26.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.25.0...v0.26.0
[v0.25.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.11.0...v0.25.0
[v0.11.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.0.0...v0.2.0
