use crate::bitcoin::{Network, NetworkKind};
use crate::descriptor::Descriptor;
use crate::error::DescriptorKeyError;
use crate::keys::{DerivationPath, DescriptorPublicKey, DescriptorSecretKey, Mnemonic};
use crate::types::WildcardType;
use bdk_wallet::bitcoin::hex::DisplayHex;
use bdk_wallet::bitcoin::PrivateKey as BdkPrivateKey;
use std::sync::Arc;

fn get_inner() -> DescriptorSecretKey {
    let mnemonic = Mnemonic::from_string("chaos fabric time speed sponsor all flat solution wisdom trophy crack object robot pave observe combine where aware bench orient secret primary cable detect".to_string()).unwrap();
    DescriptorSecretKey::new(NetworkKind::Test, &mnemonic, None)
}

fn derive_dsk(
    key: &DescriptorSecretKey,
    path: &str,
) -> Result<Arc<DescriptorSecretKey>, DescriptorKeyError> {
    let path = DerivationPath::new(path.to_string()).unwrap();
    key.derive(&path)
}

fn extend_dsk(
    key: &DescriptorSecretKey,
    path: &str,
) -> Result<Arc<DescriptorSecretKey>, DescriptorKeyError> {
    let path = DerivationPath::new(path.to_string()).unwrap();
    key.extend(&path)
}

fn derive_dpk(
    key: &DescriptorPublicKey,
    path: &str,
) -> Result<Arc<DescriptorPublicKey>, DescriptorKeyError> {
    let path = DerivationPath::new(path.to_string()).unwrap();
    key.derive(&path)
}

fn extend_dpk(
    key: &DescriptorPublicKey,
    path: &str,
) -> Result<Arc<DescriptorPublicKey>, DescriptorKeyError> {
    let path = DerivationPath::new(path.to_string()).unwrap();
    key.extend(&path)
}

#[test]
fn test_generate_descriptor_secret_key() {
    let master_dsk = get_inner();
    assert_eq!(master_dsk.to_string(), "tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h");
    assert_eq!(master_dsk.as_public().to_string(), "tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa");
}

#[test]
fn test_derive_self() {
    let master_dsk = get_inner();
    let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m").unwrap();
    assert_eq!(derived_dsk.to_string(), "[d1d04177]tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h");
    let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
    let derived_dpk: &DescriptorPublicKey = &derive_dpk(master_dpk, "m").unwrap();
    assert_eq!(derived_dpk.to_string(), "[d1d04177]tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa");
}

#[test]
fn test_derive_descriptors_keys() {
    let master_dsk = get_inner();
    let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m/0").unwrap();
    assert_eq!(derived_dsk.to_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ");
    let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
    let derived_dpk: &DescriptorPublicKey = &derive_dpk(master_dpk, "m/0").unwrap();
    assert_eq!(derived_dpk.to_string(), "[d1d04177/0]tpubD9oaCiP1MPmQdndm7DCD3D3QU34pWd6BbKSRedoZF1UJcNhEk3PJwkALNYkhxeTKL29oGNR7psqvT1KZydCGqUDEKXN6dVQJY2R8ooLPy8m");
}

#[test]
fn test_extend_descriptor_keys() {
    let master_dsk = get_inner();
    let extended_dsk: &DescriptorSecretKey = &extend_dsk(&master_dsk, "m/0").unwrap();
    assert_eq!(extended_dsk.to_string(), "tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h/0");
    let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
    let extended_dpk: &DescriptorPublicKey = &extend_dpk(master_dpk, "m/0").unwrap();
    assert_eq!(extended_dpk.to_string(), "tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa/0");
    let wif = "L2wTu6hQrnDMiFNWA5na6jB12ErGQqtXwqpSL7aWquJaZG8Ai3ch";
    let extended_key = DescriptorSecretKey::from_string(wif.to_string()).unwrap();
    let result = extended_key.derive(&DerivationPath::new("m/0".to_string()).unwrap());
    assert!(result.is_err());
}

#[test]
fn test_from_str_inner() {
    let key1 = "L2wTu6hQrnDMiFNWA5na6jB12ErGQqtXwqpSL7aWquJaZG8Ai3ch";
    let key2 = "tprv8ZgxMBicQKsPcwcD4gSnMti126ZiETsuX7qwrtMypr6FBwAP65puFn4v6c3jrN9VwtMRMph6nyT63NrfUL4C3nBzPcduzVSuHD7zbX2JKVc/1/1/1/*";
    let _private_descriptor_key1 = DescriptorSecretKey::from_string(key1.to_string()).unwrap();
    let _private_descriptor_key2 = DescriptorSecretKey::from_string(key2.to_string()).unwrap();
    // Should error out because you can't produce a DescriptorSecretKey from an xpub
    let key0 = "tpubDBrgjcxBxnXyL575sHdkpKohWu5qHKoQ7TJXKNrYznh5fVEGBv89hA8ENW7A8MFVpFUSvgLqc4Nj1WZcpePX6rrxviVtPowvMuGF5rdT2Vi";
    assert!(DescriptorSecretKey::from_string(key0.to_string()).is_err());
}

#[test]
fn test_secret_bytes_from_single_and_multipath_keys() {
    let wif = "L2wTu6hQrnDMiFNWA5na6jB12ErGQqtXwqpSL7aWquJaZG8Ai3ch";
    let single_key = DescriptorSecretKey::from_string(wif.to_string()).unwrap();
    let expected_single_bytes = BdkPrivateKey::from_wif(wif)
        .unwrap()
        .inner
        .secret_bytes()
        .to_vec();
    assert_eq!(single_key.secret_bytes(), expected_single_bytes);

    let base_xprv = "tprv8ZgxMBicQKsPcwcD4gSnMti126ZiETsuX7qwrtMypr6FBwAP65puFn4v6c3jrN9VwtMRMph6nyT63NrfUL4C3nBzPcduzVSuHD7zbX2JKVc";
    let multipath_key = DescriptorSecretKey::from_string(format!("{base_xprv}/<0;1>/*")).unwrap();
    // A multipath key names a family of keys, not a single one, so it has no secret bytes.
    assert!(multipath_key.secret_bytes().is_empty());
}

#[test]
fn test_derive_and_extend_inner() {
    let master_dsk = get_inner();
    // derive DescriptorSecretKey with path "m/0" from master
    let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m/0").unwrap();
    assert_eq!(derived_dsk.to_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ");
    // extend derived_dsk with path "m/0"
    let extended_dsk: &DescriptorSecretKey = &extend_dsk(derived_dsk, "m/0").unwrap();
    assert_eq!(extended_dsk.to_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ/0");
}

#[test]
fn test_derive_hardened_path_using_public() {
    let master_dpk = get_inner().as_public();
    let derived_dpk = &derive_dpk(&master_dpk, "m/84h/1h/0h");
    assert!(derived_dpk.is_err());
}

#[test]
fn test_derivation_path_parsing() {
    let path_str = "m/44h/0h/0h/0/0";
    let expected_path = "44'/0'/0'/0/0";
    let path_str_2 = "m";

    let derivation_path = DerivationPath::new(path_str.to_string()).unwrap();
    assert_eq!(derivation_path.to_string(), expected_path);

    let derivation_path2 = DerivationPath::new(path_str_2.to_string()).unwrap();
    assert!(derivation_path2.is_master());

    let derivation_path3 = DerivationPath::master();
    assert!(derivation_path3.is_master());

    assert!(!derivation_path.is_master());
}

#[test]
fn test_invalid_derivation_path() {
    let invalid_path_str = "m/44x/0h/0h/0/0";
    let result = DerivationPath::new(invalid_path_str.to_string());
    assert!(result.is_err());
}

#[test]
fn test_add_wildcard() {
    let mnemonic = Mnemonic::from_string("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome".to_string()).unwrap();
    let key = DescriptorSecretKey::new(NetworkKind::Test, &mnemonic, None);

    let derivation_path = DerivationPath::new("84/2h/1".to_string()).unwrap();
    let extended_key = key.extend(&derivation_path).unwrap();
    assert_eq!(extended_key.to_string(), "tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/84/2'/1");

    // Add an unhardened wildcard
    let dsk_unhardened = extended_key.add_wildcard(WildcardType::Unhardened).unwrap();
    assert_eq!(dsk_unhardened.to_string(), "tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/84/2'/1/*");

    // Add a hardened wildcard
    let dsk_hardened = extended_key.add_wildcard(WildcardType::Hardened).unwrap();
    assert_eq!(dsk_hardened.to_string(), "tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4/84/2'/1/*h");

    // Calling add_wildcard with the same type returns the same key
    assert_eq!(
        dsk_unhardened
            .add_wildcard(WildcardType::Unhardened)
            .unwrap()
            .to_string(),
        dsk_unhardened.to_string()
    );
    assert_eq!(
        dsk_hardened
            .add_wildcard(WildcardType::Hardened)
            .unwrap()
            .to_string(),
        dsk_hardened.to_string()
    );

    // Attempting to change the wildcard type returns an error
    assert!(matches!(
        dsk_unhardened.add_wildcard(WildcardType::Hardened),
        Err(DescriptorKeyError::CannotChangeWildcardType)
    ));
    assert!(matches!(
        dsk_hardened.add_wildcard(WildcardType::Unhardened),
        Err(DescriptorKeyError::CannotChangeWildcardType)
    ));

    // DescriptorPublicKey: add_wildcard() always adds an unhardened wildcard
    let dpk = extended_key.as_public();
    let dpk_with_wildcard = dpk.add_wildcard().unwrap();
    assert_eq!(dpk_with_wildcard.to_string(), "[5bc5d243/84/2']tpubDAFG7XHSgRo927vaVKhcJAjuYW6AXJPunmS8So9ipV1xUyAUzEoBoiS5xSgPNBmjPMSnSXKjsJnTHWieJzUVxz8TUdWm8BUqgy4wL9yz5hp/1/*");

    // Calling add_wildcard on a DPK that already has an unhardened wildcard returns the same DPK
    assert_eq!(
        dpk_with_wildcard.add_wildcard().unwrap().to_string(),
        dpk_with_wildcard.to_string()
    );

    // Calling add_wildcard on a DPK converted from a DSK with a hardened wildcard returns an error
    let dpk_hardened = dsk_hardened.as_public();
    assert!(matches!(
        dpk_hardened.add_wildcard(),
        Err(DescriptorKeyError::CannotChangeWildcardType)
    ));
}

// A key built with extend() records its derivation path but keeps the extended key it was built
// from, so secret_bytes() has to walk that path before reading out the private key.
#[test]
fn test_secret_bytes_follow_derivation_path() {
    let mnemonic =
        Mnemonic::from_string("all all all all all all all all all all all all".to_string())
            .unwrap();
    let master = DescriptorSecretKey::new(NetworkKind::Test, &mnemonic, None);
    let master_xprv = "tprv8ZgxMBicQKsPdfqH2fGKQkBAMXpqCpC6v6WhYnEZC7TbpcEavC1N27tHbFP16eLm9XdFDW6cqnGChit8gWXyyT1zQ3xFqUWgHTS9XBQw3j5";
    assert_eq!(master.to_string(), master_xprv);
    assert_eq!(
        master.secret_bytes().to_lower_hex_string(),
        "a1ee72b13e74424be7875abd2702d42d0baf2bb7e52baeb8524540fecaf38b26"
    );

    let extended = extend_dsk(&master, "m/1h").unwrap();
    assert_eq!(extended.to_string(), format!("{master_xprv}/1'"));
    assert_eq!(
        extended.secret_bytes().to_lower_hex_string(),
        "78a3ce14aca05a6235a3c989d850b9e9ec5ccec90e4a6dad0877838a499b5618"
    );
    assert_ne!(
        extended.secret_bytes().to_lower_hex_string(),
        master.secret_bytes().to_lower_hex_string()
    );

    // extend() and derive() name the same key, so they agree on its bytes.
    let derived = derive_dsk(&master, "m/1h").unwrap();
    assert_eq!(
        extended.secret_bytes().to_lower_hex_string(),
        derived.secret_bytes().to_lower_hex_string()
    );
}

// The account-plus-index shape extend() is built for: one fixed prefix, one varying index. Every
// index has to yield its own key.
#[test]
fn test_secret_bytes_differ_per_index() {
    let mnemonic =
        Mnemonic::from_string("all all all all all all all all all all all all".to_string())
            .unwrap();
    let master = DescriptorSecretKey::new(NetworkKind::Test, &mnemonic, None);
    let account = extend_dsk(&master, "m/84h/1h/0h/55h/3").unwrap();

    let master_bytes = master.secret_bytes().to_lower_hex_string();
    let mut seen = std::collections::HashSet::new();
    for index in [0u32, 42, 1000] {
        let child = extend_dsk(&account, &format!("m/{index}")).unwrap();
        let child_bytes = child.secret_bytes().to_lower_hex_string();
        assert_ne!(child_bytes, master_bytes);
        assert!(seen.insert(child_bytes.clone()));
        // Matches the key derive() produces for the full path.
        let full = derive_dsk(&master, &format!("m/84h/1h/0h/55h/3/{index}")).unwrap();
        assert_eq!(child_bytes, full.secret_bytes().to_lower_hex_string());
    }

    let index_42 = extend_dsk(&account, "m/42").unwrap();
    assert_eq!(
        index_42.secret_bytes().to_lower_hex_string(),
        "56f63b26cbfcd98e949ea0a2dc6f66a522c418c9e98eacb3e5c3b15b330fa72e"
    );
}

// A wildcard names a family of keys rather than a concrete child, so no secret bytes are returned
// for it.
#[test]
fn test_secret_bytes_with_wildcard() {
    let mnemonic =
        Mnemonic::from_string("all all all all all all all all all all all all".to_string())
            .unwrap();
    let master = DescriptorSecretKey::new(NetworkKind::Test, &mnemonic, None);
    let account = extend_dsk(&master, "m/84h/1h/0h").unwrap();
    assert!(!account.secret_bytes().is_empty());

    let unhardened = account.add_wildcard(WildcardType::Unhardened).unwrap();
    assert!(unhardened.secret_bytes().is_empty());

    let hardened = account.add_wildcard(WildcardType::Hardened).unwrap();
    assert!(hardened.secret_bytes().is_empty());
}

// Expected bytes are the private keys embedded in the xprv/WIF strings published in BIP-32 (test
// vector 1), BIP-84 and BIP-86, so they check secret_bytes() against values that do not come from
// this library.
#[test]
fn test_secret_bytes_match_bip_test_vectors() {
    // BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
    // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-1
    let master = DescriptorSecretKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi".to_string()).unwrap();
    assert_eq!(
        master.secret_bytes().to_lower_hex_string(),
        "e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35"
    );
    for (path, expected) in [
        (
            "m/0h/1/2h",
            "cbce0d719ecf7431d88e6a89fa1483e02e35092af60c042b1df2ff59fa424dca",
        ),
        (
            "m/0h/1/2h/2/1000000000",
            "471b76e389e528d6de6d816857e012c5455051cad6660850e58372a6c3e6e7c8",
        ),
    ] {
        assert_eq!(
            extend_dsk(&master, path)
                .unwrap()
                .secret_bytes()
                .to_lower_hex_string(),
            expected
        );
        assert_eq!(
            derive_dsk(&master, path)
                .unwrap()
                .secret_bytes()
                .to_lower_hex_string(),
            expected
        );
    }

    // BIP-84 and BIP-86 share this mnemonic.
    let mnemonic = Mnemonic::from_string(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .to_string(),
    )
    .unwrap();
    let root = DescriptorSecretKey::new(NetworkKind::Main, &mnemonic, None);
    assert_eq!(root.to_string(), "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu");
    assert_eq!(
        root.secret_bytes().to_lower_hex_string(),
        "1837c1be8e2995ec11cda2b066151be2cfb48adf9e47b151d46adab3a21cdf67"
    );

    // BIP-84 account 0, extended from the account key the way a wallet would walk its addresses.
    // https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki#test-vectors
    let bip84_account = extend_dsk(&root, "m/84h/0h/0h").unwrap();
    for (path, expected) in [
        (
            "m/0/0",
            "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3",
        ),
        (
            "m/0/1",
            "2fd0affa51529f940a358ec0c50de81267d0bf5158ca61887347676946362c5b",
        ),
        (
            "m/1/0",
            "3277578a56b721e4c9f071f1e24aa0f94c4ff72e7967fea03b134f605f07c8fd",
        ),
    ] {
        assert_eq!(
            extend_dsk(&bip84_account, path)
                .unwrap()
                .secret_bytes()
                .to_lower_hex_string(),
            expected
        );
    }

    // BIP-86 account 0, first receiving address.
    // https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki#test-vectors
    assert_eq!(
        extend_dsk(&root, "m/86h/0h/0h/0/0")
            .unwrap()
            .secret_bytes()
            .to_lower_hex_string(),
        "41f41d69260df4cf277826a9b65a3717e4eeddbeedf637f212ca096576479361"
    )
}

#[test]
fn test_derive_applies_extended_path() {
    let master_dsk = get_inner();

    // extend(m/1h).derive(m/2h) must equal derive(m/1h/2h), not derive(m/2h)
    let extended_then_derived = derive_dsk(&extend_dsk(&master_dsk, "1h").unwrap(), "2h").unwrap();
    let derived_full = derive_dsk(&master_dsk, "m/1h/2h").unwrap();
    let derived_only = derive_dsk(&master_dsk, "m/2h").unwrap();
    assert_eq!(extended_then_derived.to_string(), derived_full.to_string());
    assert_ne!(extended_then_derived.to_string(), derived_only.to_string());

    // Multiple chained extends are all applied
    let many_extends = extend_dsk(
        &extend_dsk(&extend_dsk(&master_dsk, "44h").unwrap(), "1h").unwrap(),
        "0h",
    )
    .unwrap();
    assert_eq!(
        derive_dsk(&many_extends, "0").unwrap().to_string(),
        derive_dsk(&master_dsk, "m/44h/1h/0h/0")
            .unwrap()
            .to_string()
    );

    // Same for public keys
    let master_dpk = master_dsk.as_public();
    let extended_then_derived = derive_dpk(&extend_dpk(&master_dpk, "1").unwrap(), "2").unwrap();
    let derived_full = derive_dpk(&master_dpk, "m/1/2").unwrap();
    assert_eq!(extended_then_derived.to_string(), derived_full.to_string());
    assert!(extended_then_derived
        .to_string()
        .starts_with("[d1d04177/1/2]"));
}

#[test]
fn test_derive_applies_parsed_path() {
    let master_dsk = get_inner();

    // A key parsed with a path suffix must have that path applied by derive()
    let parsed = DescriptorSecretKey::from_string(format!("{master_dsk}/84h/1h/0h")).unwrap();
    assert_eq!(
        derive_dsk(&parsed, "0").unwrap().to_string(),
        derive_dsk(&master_dsk, "m/84h/1h/0h/0")
            .unwrap()
            .to_string()
    );

    // Keys with an origin keep that origin and append the full path to it
    let account = derive_dsk(&master_dsk, "m/84h/1h/0h").unwrap();
    let parsed_with_origin = DescriptorSecretKey::from_string(format!("{account}/1")).unwrap();
    assert_eq!(
        derive_dsk(&parsed_with_origin, "5").unwrap().to_string(),
        derive_dsk(&master_dsk, "m/84h/1h/0h/1/5")
            .unwrap()
            .to_string()
    );

    let account_dpk = account.as_public();
    let parsed_dpk = DescriptorPublicKey::from_string(format!("{account_dpk}/1")).unwrap();
    assert_eq!(
        derive_dpk(&parsed_dpk, "5").unwrap().to_string(),
        derive_dsk(&master_dsk, "m/84h/1h/0h/1/5")
            .unwrap()
            .as_public()
            .to_string()
    );
}

// Official test vectors from BIP-32, BIP-84 and BIP-86. Each test reaches the BIP key through
// extend() and/or fromString() with a path suffix, then derive(), so any pending derivation
// path that is dropped or misapplied shows up as a mismatch against the published values.

fn path(path: &str) -> DerivationPath {
    DerivationPath::new(path.to_string()).unwrap()
}

fn address(descriptor: String) -> String {
    Descriptor::new(descriptor, NetworkKind::Main)
        .unwrap()
        .derive_address(0, Network::Bitcoin)
        .unwrap()
        .to_string()
}

// BIP-39 mnemonic used by the BIP-84 and BIP-86 test vectors.
fn abandon_master() -> DescriptorSecretKey {
    let mnemonic = Mnemonic::from_string("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string()).unwrap();
    DescriptorSecretKey::new(NetworkKind::Main, &mnemonic, None)
}

// BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
const BIP32_V1_MASTER_XPRV: &str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";

#[test]
fn test_bip32_vector_1_extend_then_derive() {
    let master = DescriptorSecretKey::from_string(BIP32_V1_MASTER_XPRV.to_string()).unwrap();

    // Chain m/0H/1/2H
    let key = master
        .extend(&path("0h"))
        .unwrap()
        .extend(&path("1"))
        .unwrap()
        .derive(&path("2h"))
        .unwrap();
    assert_eq!(key.to_string(), "[3442193e/0'/1/2']xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM");

    // Chain m/0H/1/2H/2/1000000000
    let key = master
        .extend(&path("0h/1/2h"))
        .unwrap()
        .derive(&path("2/1000000000"))
        .unwrap();
    assert_eq!(key.to_string(), "[3442193e/0'/1/2'/2/1000000000]xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76");
    assert_eq!(key.as_public().to_string(), "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy");
}

#[test]
fn test_bip32_vector_1_public_parsed_path_then_derive() {
    // Chain m/0H/1/2H xpub with its origin, and a pending "/2" parsed from the string.
    let key = DescriptorPublicKey::from_string("[3442193e/0'/1/2']xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5/2".to_string()).unwrap();

    // Chain m/0H/1/2H/2/1000000000
    assert_eq!(
        key.derive(&path("1000000000")).unwrap().to_string(),
        "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy"
    );
}

#[test]
fn test_bip86_vectors() {
    let master = abandon_master();
    assert_eq!(master.to_string(), "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu");

    // First receiving address, m/86'/0'/0'/0/0, via extend() then derive()
    let first_receive = master
        .extend(&path("86h/0h/0h"))
        .unwrap()
        .derive(&path("0/0"))
        .unwrap();
    assert_eq!(first_receive.to_string(), "[73c5da0a/86'/0'/0'/0/0]xprvA449goEeU9okwCzzZaxiy475EQGQzBkc65su82nXEvcwzfSskb2hAt2WymrjyRL6kpbVTGL3cKtp9herYXSjjQ1j4stsXXiRF7kXkCacK3T");
    assert_eq!(
        address(format!("tr({})", first_receive.as_public())),
        "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr"
    );

    // Second receiving address, m/86'/0'/0'/0/1, via fromString() with a path suffix
    let account = DescriptorSecretKey::from_string(format!("{master}/86h/0h/0h")).unwrap();
    let second_receive = account.derive(&path("0/1")).unwrap();
    assert_eq!(second_receive.to_string(), "[73c5da0a/86'/0'/0'/0/1]xprvA449goEeU9okyiF1LmKiDaTgeXvmh87DVyRd35VPbsSop8n8uALpbtrUhUXByPFKK7C2yuqrB1FrhiDkEMC4RGmA5KTwsE1aB5jRu9zHsuQ");
    assert_eq!(
        address(format!("tr({})", second_receive.as_public())),
        "bc1p4qhjn9zdvkux4e44uhx8tc55attvtyu358kutcqkudyccelu0was9fqzwh"
    );

    // First change address, m/86'/0'/0'/1/0, from the published account xpub
    let account_xpub = DescriptorPublicKey::from_string("[73c5da0a/86'/0'/0']xpub6BgBgsespWvERF3LHQu6CnqdvfEvtMcQjYrcRzx53QJjSxarj2afYWcLteoGVky7D3UKDP9QyrLprQ3VCECoY49yfdDEHGCtMMj92pReUsQ".to_string()).unwrap();
    let first_change = account_xpub
        .extend(&path("1"))
        .unwrap()
        .derive(&path("0"))
        .unwrap();
    assert_eq!(first_change.to_string(), "[73c5da0a/86'/0'/0'/1/0]xpub6GL8SnQwRCGDhB59LEz9HMyM6sRYoByXBzXK3iEKWgCz8XrZNHUzd9L3AUBELW5NzA7dEFvMas1F84TuPH3xqdUA5tumaGWFgihJzWytXe3");
    assert_eq!(
        address(format!("tr({first_change})")),
        "bc1p3qkhfews2uk44qtvauqyr2ttdsw7svhkl9nkm9s9c3x4ax5h60wqwruhk7"
    );
}

#[test]
fn test_bip84_vectors() {
    // BIP-84 publishes zprv/zpub keys, which BDK does not produce, so these checks compare the
    // published addresses instead.
    let master = abandon_master();

    // First receiving address, m/84'/0'/0'/0/0, via extend() then derive()
    let first_receive = master
        .extend(&path("84h/0h/0h"))
        .unwrap()
        .derive(&path("0/0"))
        .unwrap();
    assert_eq!(
        address(format!("wpkh({})", first_receive.as_public())),
        "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"
    );

    // Second receiving address, m/84'/0'/0'/0/1, via fromString() with a path suffix
    let account = DescriptorSecretKey::from_string(format!("{master}/84h/0h/0h/0")).unwrap();
    assert_eq!(
        address(format!(
            "wpkh({})",
            account.derive(&path("1")).unwrap().as_public()
        )),
        "bc1qnjg0jd8228aq7egyzacy8cys3knf9xvrerkf9g"
    );

    // First change address, m/84'/0'/0'/1/0, from the account xpub
    let account_xpub = master.derive(&path("m/84h/0h/0h")).unwrap().as_public();
    let first_change = account_xpub
        .extend(&path("1"))
        .unwrap()
        .derive(&path("0"))
        .unwrap();
    assert_eq!(
        address(format!("wpkh({first_change})")),
        "bc1q8c6fshw2dlwun7ekn9qwf37cu2rn755upcp6el"
    );
}
