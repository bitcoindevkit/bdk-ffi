# Changelog

Changelog information can also be found in each release's git tag (which can be viewed with `git tag -ln100 "v*"`), as well as on the [GitHub releases](https://github.com/bitcoindevkit/bdk-ffi/releases) page. See [DEVELOPMENT_CYCLE.md](DEVELOPMENT_CYCLE.md) for more details.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/bitcoindevkit/bdk-ffi/compare/v2.3.0...HEAD)

### Added
- Expose miniscript `has_wildcard` and `sanity_check` methods on `Descriptor` type #945
- Expose `new_wsh_sortedmulti` and `new_pk` methods on `Descriptor` type #949

## [v2.3.0]

This is version `2.3.0` of the BDK language bindings! This release uses the following Rust dependencies:

- bdk_wallet `2.3.0`
- bdk_electrum `0.23.2`
- bdk_esplora `0.22.1`
- bdk_kyoto `0.15.3`
- bitcoin `0.32.7`
- uniffi `0.30.0`

### Added

- Expose `Wallet::apply_update_events` which returns a `WalletEvent` type #908
- Expose `Psbt::output` which returns a list of psbt `Output` #903
- `Other` variant added to CBF `RecoveryPoint` to enable wallet birthdays #920
- Implement `Display` trait for `FeeRate` #859
- Expose `FeeRate::fee_vb` and `FeeRate::fee_wu` methods #859
- Wallet API: expose `TxBuilder::only_witness_utxo` and `TxBuilder::add_foreign_utxo` #928
- Esplora API: expose `get_block_by_hash` which returns a `Block` #936
- Esplora API: expose `get_tip_hash` #930
- Esplora API: expose `get_header_by_hash` #930
- Esplora API: expose `get_address_txs` #930
- Esplora API: expose `get_tx_no_opt` #930
- Esplora API: expose `get_txid_at_block_index` #930
- Esplora API: expose `get_merkle_proof` and `get_output_status` #942
- Electrum API: expose `fetch_tx` #931
- Electrum API: expose `block_header` #931
- Electrum API: expose `block_headers_pop` #931
- Electrum API: expose `relay_fee` and `transaction_get_raw` #938
- DerivationPath: expose `child`, `extend`, and `to_u32_vec` #935

## [v2.2.0]

This is version `2.2.0` of the BDK language bindings! This release uses the following Rust dependencies:

- bdk_wallet `2.2.0`
- bdk_electrum `0.23.2`
- bdk_esplora `0.22.1`
- bdk_kyoto `0.15.2`
- rust-bitcoin `0.32.7`
- uniffi `0.29.4`

### Added

- Implement `Display` for `Script` [#813]
- Expose `Wallet::create_single` constructor on Wallet type [#825]
- Set `lookahead` on `Wallet::load` [#829]
- Expose `Wallet::apply_evicted_txs` method [#832]
- Add `Wallet::create_from_two_path_descriptor` constructor [#847]
- `next_log` and `LogLevel` are removed from `kyoto` [#849]
- Updates Android native libraries to use 16KB page sizes [#865]
- Expose `TxBuilder::exclude_unconfirmed` method [#870]
- Expose `TxBuilder::exclude_below_confirmations` method [#870]
- Expose `Wallet::insert_txout` [#884]
- Expose `Wallet::nmark_used` [#882]
- Add `Wallet::load_single` [#897]
- Expose `DerivationPath::master` , `DerivationPath::is_master` , `DerivationPath::len` , `DerivationPath::is_empty` [#893]

### Changed

- Throw errors rather than panics on faulty Esplora sync/full_scan operations [#863]

[#813]: https://github.com/bitcoindevkit/bdk-ffi/pull/813
[#825]: https://github.com/bitcoindevkit/bdk-ffi/pull/825
[#829]: https://github.com/bitcoindevkit/bdk-ffi/pull/829
[#832]: https://github.com/bitcoindevkit/bdk-ffi/pull/832
[#847]: https://github.com/bitcoindevkit/bdk-ffi/pull/847
[#849]: https://github.com/bitcoindevkit/bdk-ffi/pull/849
[#863]: https://github.com/bitcoindevkit/bdk-ffi/pull/863
[#865]: https://github.com/bitcoindevkit/bdk-ffi/pull/865
[#870]: https://github.com/bitcoindevkit/bdk-ffi/pull/870
[#884]: https://github.com/bitcoindevkit/bdk-ffi/pull/884
[#882]: https://github.com/bitcoindevkit/bdk-ffi/pull/882
[#893]: https://github.com/bitcoindevkit/bdk-ffi/pull/893
[#897]: https://github.com/bitcoindevkit/bdk-ffi/pull/897

## [v2.0.0]

This release brings bdk-ffi to it's `2.0.0` version!

The release uses the following Rust dependencies:

- bdk_wallet `2.0.0`
- bdk_electrum `0.23.0`
- bdk_esplora `0.22.0`
- bdk_kyoto `0.13.1`
- uniffi `0.29.1`
- rust-bitcoin `0.32.6`

### Added

- PSBT file operations: read and write PSBT files [#800]
- New `Psbt::from_unsigned_tx` constructor [#802]
- New `Psbt::spend_utxo` method [#798]
- Arbitrary persistence for wallet [#771]
- Wallet changeset primitives [#756]
- Display implementation for `Transaction` [#799]
- `Descriptor::max_weight_to_satisfy` method [#794]
- Expose `Wallet::public_descriptor` [#786]
- Expose `Wallet::tx_details` [#778]
- Expose `Wallet::latest_checkpoint` [#761]
- `TxGraphChangeSet::first_seen` and `last_evicted` fields [#782]
- `from_string` constructors for hash types [#784]
- `Transaction::wtxid` method [#773]
- Kyoto: average feerate and connect functionality [#797]

### Changed

- Use `Amount` type in `TxOut` instead of u64 [#781]
- Update to latest bdk_kyoto with API changes [#772]
- Add `lookahead` as optional argument to wallet methods [#770]

### Fixed

- Export public types from `Script` [#763]
- Change object to record for struct with fields [#738]

[#738]: https://github.com/bitcoindevkit/bdk-ffi/pull/738
[#756]: https://github.com/bitcoindevkit/bdk-ffi/pull/756
[#761]: https://github.com/bitcoindevkit/bdk-ffi/pull/761
[#763]: https://github.com/bitcoindevkit/bdk-ffi/pull/763
[#770]: https://github.com/bitcoindevkit/bdk-ffi/pull/770
[#771]: https://github.com/bitcoindevkit/bdk-ffi/pull/771
[#772]: https://github.com/bitcoindevkit/bdk-ffi/pull/772
[#773]: https://github.com/bitcoindevkit/bdk-ffi/pull/773
[#778]: https://github.com/bitcoindevkit/bdk-ffi/pull/778
[#781]: https://github.com/bitcoindevkit/bdk-ffi/pull/781
[#782]: https://github.com/bitcoindevkit/bdk-ffi/pull/782
[#784]: https://github.com/bitcoindevkit/bdk-ffi/pull/784
[#786]: https://github.com/bitcoindevkit/bdk-ffi/pull/786
[#794]: https://github.com/bitcoindevkit/bdk-ffi/pull/794
[#797]: https://github.com/bitcoindevkit/bdk-ffi/pull/797
[#798]: https://github.com/bitcoindevkit/bdk-ffi/pull/798
[#799]: https://github.com/bitcoindevkit/bdk-ffi/pull/799
[#800]: https://github.com/bitcoindevkit/bdk-ffi/pull/800
[#802]: https://github.com/bitcoindevkit/bdk-ffi/pull/802

## [v1.2.0]

This release brings in a new experimental compact block filters client!

The release uses the following Rust dependencies:

- bdk_wallet `1.2.0`
- bdk_electrum `0.21.0`
- bdk_esplora `0.20.1`
- bdk_kyoto `0.8.0`
- uniffi `0.29.1`
- rust-bitcoin `0.32.5`

### Added

- New CBF client (Kyoto) [#591], [#716]
- Add optional `proxy` parameter to Esplora client constructor [#711]
- Add optional `socks5` parameter to Electrum client constructor [#711]
- Add `ElectrumClient::ping` method [#689]
- New API docs for a lot of types
- Add `Wallet::apply_unconfirmed_txs` method [#704]
- Add `UnconfirmedTx` type [#704]

### Changed

- The `Amount::from_sat` constructor renamed its `sat` argument to `satoshi` [#708]

[#591]: https://github.com/bitcoindevkit/bdk-ffi/pull/591
[#689]: https://github.com/bitcoindevkit/bdk-ffi/pull/689
[#704]: https://github.com/bitcoindevkit/bdk-ffi/pull/704
[#708]: https://github.com/bitcoindevkit/bdk-ffi/pull/708
[#711]: https://github.com/bitcoindevkit/bdk-ffi/pull/711
[#716]: https://github.com/bitcoindevkit/bdk-ffi/pull/716

## [v1.1.0]

This is our first stable release!

This release uses the following Rust dependencies:

- bdk_wallet `1.1.0`
- bdk_electrum `0.21.0`
- bdk_esplora `0.20.1`
- uniffi `0.29.0`
- rust-bitcoin `0.32.5`

### Added

- Expose `ElectrumClient::block_headers_subscribe` method [#664]
- Expose `EsploraClient::get_block_hash` method [#665]
- Expose `EsploraClient::get_tx_status` method [#666]
- Expose `EsploraClient::get_tx_info` method [#666]
- Support for Testnet 4 [#674]
- Add `AddressData` and `WitnessProgram` types from rust bitcoin [#671]
- Expose `Address::to_address_data` method [#671]

### Changed

- More complete `LocalOutput` type [#667]

[#664]: https://github.com/bitcoindevkit/bdk-ffi/pull/664
[#665]: https://github.com/bitcoindevkit/bdk-ffi/pull/665
[#666]: https://github.com/bitcoindevkit/bdk-ffi/pull/666
[#667]: https://github.com/bitcoindevkit/bdk-ffi/pull/667
[#671]: https://github.com/bitcoindevkit/bdk-ffi/pull/671
[#674]: https://github.com/bitcoindevkit/bdk-ffi/pull/674

## [v1.0.0-beta.7]

This release updates the `bdk-ffi` libraries to the final `bdk_wallet` `1.0.0` and related libraries (Esplora, Electrum, etc).

### Added

- `ElectrumClient::server_features` [#641]
- `ServerFeaturesRes` struct [#641]
- `ElectrumClient::estimate_fee` [#641]
- `EsploraClient::get_fee_estimates` [#648]
- New optional argument sign_options on `Wallet::sign` and `Wallet::finalize_psbt` [#650]

### Changed

- The full_scan and sync methods on the Electrum and Esplora clients now take a renamed `request` argument [#642]
- ElectrumClient::broacast was renamed ElectrumClient::transaction_broadcast to mirror the Rust API [#642]

[#641]: https://github.com/bitcoindevkit/bdk-ffi/pull/641
[#642]: https://github.com/bitcoindevkit/bdk-ffi/pull/642
[#648]: https://github.com/bitcoindevkit/bdk-ffi/pull/648
[#650]: https://github.com/bitcoindevkit/bdk-ffi/pull/650

## [v1.0.0-beta.6]

This release updates the `bdk-ffi` libraries to the latest `bdk_wallet` `1.0.0-beta.6` and related libraries (Esplora, Electrum, etc). 

### Added

- `DescriptorPublicKey::is_multipath` [#625]
- `DescriptorPublicKey::master_fingerprint` [#625]
- `Descriptor::is_multipath` [#625]
- `Descriptor:: to_single_descriptors` [#625]
- `EsploraClient::get_height` [#623]
- `Psbt::finalize` [#630]
- `TxBuilder::add_data` [#611]
- `TxBuilder::current_height` [#611]
- `TxBuilder::nlocktime` [#611]
- `TxBuilder::allow_dust` [#611]
- `TxBuilder::version` [#611]
- `TxBuilder::policy_path` [#629]
- `Wallet::cancel_tx` [#601]
- `Wallet::get_utxo` [#601]
- `Wallet::derivation_of_spk` [#601]
- `Wallet::descriptor_checksum` [#603]
- `Wallet:: finalize_psbt` [#604]
- `Wallet:: policies` [#629]

#### Other

- Added documentation via docstrings

[#601]: https://github.com/bitcoindevkit/bdk-ffi/pull/601
[#603]: https://github.com/bitcoindevkit/bdk-ffi/pull/603
[#604]: https://github.com/bitcoindevkit/bdk-ffi/pull/604
[#611]: https://github.com/bitcoindevkit/bdk-ffi/pull/611
[#623]: https://github.com/bitcoindevkit/bdk-ffi/pull/623
[#625]: https://github.com/bitcoindevkit/bdk-ffi/pull/625
[#629]: https://github.com/bitcoindevkit/bdk-ffi/pull/629
[#645]: https://github.com/bitcoindevkit/bdk-ffi/pull/645

## [v1.0.0-beta.5]

This release updates the bdk-ffi libraries to the latest bdk_wallet 1.0.0-beta.5 and related libraries (Esplora, Electrum, etc.). 

### Added

`EsploraClient`
    - `get_tx` [#598]
`Wallet`
    - `peek_address` [#599]
    - `next_derivation_index` [#599]
    - `next_unused_address` [#599]
    - `mark_used` [#599]
    - `reveal_addresses_to` [#599]
    - `list_unused_addresses` [#599]
    - `descriptor_checksum` [#603]
    - `finalize_psbt` [#604]
    - `cancel_tx` [#601]
    - `get_utxo` [#601]
    - `derivation_of_spk` [#601]
`TxBuilder`
    - `set_exact_sequence` [#600]

### Changed
`Wallet`
    - corrected argument name in `reveal_next_address` [#599]
    
### Removed
`TxBuilder`
    - `enable_rbf` [#600]

[#598]: https://github.com/bitcoindevkit/bdk-ffi/pull/598
[#599]: https://github.com/bitcoindevkit/bdk-ffi/pull/599
[#600]: https://github.com/bitcoindevkit/bdk-ffi/pull/600
[#601]: https://github.com/bitcoindevkit/bdk-ffi/pull/601
[#603]: https://github.com/bitcoindevkit/bdk-ffi/pull/603
[#604]: https://github.com/bitcoindevkit/bdk-ffi/pull/604

## [v1.0.0-beta.2]

This release updates the bdk-ffi libraries to the latest bdk_wallet 1.0.0-beta.2 and related libraries (Esplora, Electrum, etc.), as well as uses the latest uniffi-rs library version 0.28.0. The releases now depend on [bitcoin-ffi] for the types that are exposed from the rust-bitcoin org. It also bumps the minimum supported Android API level to 24 (Android Nougat).

### Added

- SQLite persistence through bdk_sqlite [#544]
- The `Address`, `DescriptorSecretKey`, `DescriptorPublicKey`, `Mnemonic`, and `Descriptor` types now have the `toString()` method implemented on them by default [#551]
- `Address.from_script()` [#554]
- New `FromScriptError` [#561]
- New type `ChangeSet` [#561]
- Wallet constructors do not take a persistence path anymore [#561]
- `Wallet.get_balance()` method renamed to `balance()` [#561]
- Add `add_global_xpubs()` method on `TxBuilder` [#574]
- Add `wallet.derivation_index` method on Wallet type [#579]
- Add `wallet.persist` method on Wallet type [#582]
- Add `Connection` type [#582]

### Changed

- `AddressError` is replaced by `AddressParseError` [#561]
- New variants in `CalculateFeeError` [#561]
- New variants in `CreateTxError` [#561]
- New variants in `ParseAmountError` [#561]
- New variants in `SignerError` [#561]
- New variants in `WalletCreationError` [#561]
- `Wallet.calculate_fee()` returns an `Amount` [#561]
- Renamed `Transaction.txid()` to `Transaction.compute_txid()` [#561]

### Removed

- flat file persistence [#544]

[#544]: https://github.com/bitcoindevkit/bdk-ffi/pull/544
[#551]: https://github.com/bitcoindevkit/bdk-ffi/pull/551
[#554]: https://github.com/bitcoindevkit/bdk-ffi/pull/554
[#561]: https://github.com/bitcoindevkit/bdk-ffi/pull/561
[#574]: https://github.com/bitcoindevkit/bdk-ffi/pull/574
[#579]: https://github.com/bitcoindevkit/bdk-ffi/pull/579
[#582]: https://github.com/bitcoindevkit/bdk-ffi/pull/582
[bitcoin-ffi]: https://github.com/bitcoindevkit/bitcoin-ffi

## [v1.0.0-alpha.11]

This release brings the latest alpha 11 release of the Rust bdk_wallet library, as well as the new Electrum client, the new memory wallet, and a whole lot of new types and APIs across the library. Also of note are the much simpler-to-use full_scan and sync workflows for syncing wallets.

### Added

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

## [v0.32.1]

This is a patch release, updating the bindings libraries to bdk version 0.30.2, fixing an issue with syncing very large wallets.

See https://github.com/bitcoindevkit/bdk/releases/tag/v0.30.2 for details.

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

### Added
- Add BIP-86 descriptor templates [#388]

[#388]: https://github.com/bitcoindevkit/bdk-ffi/pull/388

## [v0.29.0]

This release has a number of new APIs, and adds support for Windows in bdk-jvm.

### Added

- Add support for Windows in bdk-jvm [#336]
- Add support for older version of Linux distros in bdk-jvm [#345]
- Expose `is_mine()` method on the `Wallet` type [#355]
- Expose `to_bytes()` method on the `Script` type [#369]

[#336]: https://github.com/bitcoindevkit/bdk-ffi/pull/336
[#345]: https://github.com/bitcoindevkit/bdk-ffi/pull/345
[#355]: https://github.com/bitcoindevkit/bdk-ffi/pull/355
[#369]: https://github.com/bitcoindevkit/bdk-ffi/pull/369

## [v0.28.0]

### Added
  - Expose Address payload and network properties. [#325]
  - Add SignOptions to Wallet.sign() params. [#326]
  - address field on `AddressInfo` type is now of type `Address` [#333]
  - new PartiallySignedTransaction.json_serialize() function to get JSON serialized value of all PSBT fields. [#334]
  - Add from_script constructor to `Address` type [#337]

### Other

- Update BDK to version 0.28.0 [#341]
- Drop support of pypy releases of Python libraries [#351]
- Drop support for Python 3.6 and 3.7 [#351]
- Drop support for very old Linux versions that do not support the manylinux_2_17_x86_64 platform tag [#351]

[#325]: https://github.com/bitcoindevkit/bdk-ffi/pull/325
[#326]: https://github.com/bitcoindevkit/bdk-ffi/pull/326
[#333]: https://github.com/bitcoindevkit/bdk-ffi/pull/333
[#334]: https://github.com/bitcoindevkit/bdk-ffi/pull/334
[#337]: https://github.com/bitcoindevkit/bdk-ffi/pull/337
[#341]: https://github.com/bitcoindevkit/bdk-ffi/pull/341
[#351]: https://github.com/bitcoindevkit/bdk-ffi/pull/351

## [v0.27.1]

- Update BDK to version 0.27.1 [#312]

### Added

- New `Transaction` structure that can be created from or serialized to consensus encoded bytes. [#296]
- Add Wallet.get_internal_address() API [#304]
- Add `AddressIndex::Peek(index)` and `AddressIndex::Reset(index)` APIs [#305]

### Changed

- `PartiallySignedTransaction.extract_tx()` returns a `Transaction` instead of the transaction bytes. [#296]
- `Blockchain.broadcast()` takes a `Transaction` instead of a `PartiallySignedTransaction`. [#296]

[#296]: https://github.com/bitcoindevkit/bdk-ffi/pull/296
[#304]: https://github.com/bitcoindevkit/bdk-ffi/pull/304
[#305]: https://github.com/bitcoindevkit/bdk-ffi/pull/305
[#312]: https://github.com/bitcoindevkit/bdk-ffi/pull/312

## [v0.26.0]

- Update BDK to version 0.26.0 [#288]

### Added
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

### Changed
  - The descriptor and change_descriptor arguments on the wallet constructor now take a `Descriptor` instead of a `String`. [#260]
  - TxBuilder.drain_to() argument is now `Script` instead of address `String`. [#279]


[#125]: https://github.com/bitcoindevkit/bdk-ffi/pull/125
[#260]: https://github.com/bitcoindevkit/bdk-ffi/pull/260
[#279]: https://github.com/bitcoindevkit/bdk-ffi/pull/279
[#288]: https://github.com/bitcoindevkit/bdk-ffi/pull/288

## [v0.25.0]

- Update BDK to version 0.25.0 [#272]

### Added

  - from_string() constructors now available on DescriptorSecretKey and DescriptorPublicKey [#247]

[#247]: https://github.com/bitcoindevkit/bdk-ffi/pull/247
[#272]: https://github.com/bitcoindevkit/bdk-ffi/pull/272

## [v0.11.0]

- Update BDK to version 0.24.0 [#221]

### Added

- Added Mnemonic struct [#219] with following methods:
  - new(word_count: WordCount) generates and returns Mnemonic with random entropy
  - from_string(mnemonic: String) converts string Mnemonic to Mnemonic type with error
  - from_entropy(entropy: Vec<u8>) generates and returns Mnemonic with given entropy
  - as_string() view Mnemonic as string

### Changed

- The constructor on the DescriptorSecretKey type now takes a Mnemonic instead of a String.

### Removed

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

[v2.3.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v2.2.0...v2.3.0
[v2.2.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v2.0.0...v2.2.0
[v2.0.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.2.0...v2.0.0
[v1.2.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.1.0...v1.2.0
[v1.1.0]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-beta.7...v1.1.0
[v1.0.0-beta.7]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-beta.6...v1.0.0-beta.7
[v1.0.0-beta.6]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-beta.5...v1.0.0-beta.6
[v1.0.0-beta.5]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-beta.2...v1.0.0-beta.5
[v1.0.0-beta.2]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.11...v1.0.0-beta.2
[v1.0.0-alpha.11]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.7...v1.0.0-alpha.11
[v1.0.0-alpha.7]: https://github.com/bitcoindevkit/bdk-ffi/compare/v1.0.0-alpha.2a...v1.0.0-alpha.7
[v1.0.0-alpha.2a]: https://github.com/bitcoindevkit/bdk-ffi/compare/v0.31.0...v1.0.0-alpha.2a
[v0.32.1]: (https://github.com/bitcoindevkit/bdk-ffi/compare/v0.32.0...v0.32.1)
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
