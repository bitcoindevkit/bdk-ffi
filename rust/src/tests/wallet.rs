use crate::bitcoin::{Network, NetworkKind};
use crate::descriptor::Descriptor;
use crate::store::Persister;
use crate::wallet::Wallet;

use bdk_wallet::KeychainKind;

use std::sync::Arc;

const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";
const TWO_PATH_DESCRIPTOR: &str = "wpkh([9a6a2580/84'/1'/0']tpubDDnGNapGEY6AZAdQbfRJgMg9fvz8pUBrLwvyvUqEgcUfgzM6zc2eVK4vY9x9L5FJWdX8WumXuLEDV5zDZnTfbn87vLe9XceCFwTu9so9Kks/<0;1>/*)";
const EXPECTED_FIRST_ADDRESS: &str = "tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4";

fn external_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn internal_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn two_path_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(TWO_PATH_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn build_wallet() -> Wallet {
    Wallet::new(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap()
}

#[test]
fn test_create_wallet() {
    let wallet = build_wallet();

    assert_eq!(wallet.network(), Network::Signet);
    assert_eq!(wallet.balance().total.to_sat(), 0u64);
    assert!(wallet.list_unspent().is_empty());
    assert_eq!(wallet.derivation_index(KeychainKind::External), None);
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 0);
}

#[test]
fn test_reveal_next_address() {
    let wallet = build_wallet();

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
}

#[test]
fn test_create_single_wallet() {
    let wallet = Wallet::create_single(
        external_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 1);
}

#[test]
fn test_create_two_path_wallet() {
    let wallet = Wallet::create_from_two_path_descriptor(
        two_path_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);
    assert_eq!(wallet.derivation_index(KeychainKind::Internal), None);

    let external_address = wallet.reveal_next_address(KeychainKind::External);
    let internal_address = wallet.reveal_next_address(KeychainKind::Internal);

    assert_eq!(external_address.index, 0);
    assert_eq!(external_address.keychain, KeychainKind::External);
    assert_eq!(internal_address.index, 0);
    assert_eq!(internal_address.keychain, KeychainKind::Internal);
    assert_ne!(
        external_address.address.to_string(),
        internal_address.address.to_string()
    );
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.derivation_index(KeychainKind::Internal), Some(0));
}
