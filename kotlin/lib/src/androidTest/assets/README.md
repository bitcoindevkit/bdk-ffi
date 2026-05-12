# Readme

## Wallet 1

The wallet database `awesome_wallet_1.sqlite3` is used for testing. This wallet:

- Is a Regtest wallet
- Was built using 2 descriptors
- Has a transaction on address index 0
- Has a transaction on address index 21
- Has a transaction that spends from these 2 to address index 25
- Has 3 unspent UTXOs
- Has a total balance of 24,691,201 satoshis
- Was build using the MNEMONIC_AWESOME mnemonic

## Wallet 2

The wallet database `single_descriptor_wallet.sqlite3` is used for testing. This wallet:

- Is a Regtest wallet
- Was built using 1 descriptor
- Has revealed address 0
- Was build using the MNEMONIC_AWESOME mnemonic

## Wallet 3

The wallet database `wallet_pre_v1.sqlite3` is used for testing. This wallet:

- Is a Regtest wallet
- Was built using 2 descriptors
- Has revealed the first 8 addresses on the external keychain (last revealed index is 7)
- Has revealed the first address on the internal keychain (last revealed index is 0)
- The descriptors are BIP86 descriptors with the MNEMONIC_AWESOME mnemonic
