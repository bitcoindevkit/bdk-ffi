use crate::error::DescriptorError;
use crate::keys::DescriptorPublicKey;
use crate::keys::DescriptorSecretKey;

use bdk_wallet::bitcoin::bip32::Fingerprint;
use bdk_wallet::bitcoin::key::Secp256k1;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::descriptor::{ExtendedDescriptor, IntoWalletDescriptor};
use bdk_wallet::keys::DescriptorPublicKey as BdkDescriptorPublicKey;
use bdk_wallet::keys::{DescriptorSecretKey as BdkDescriptorSecretKey, KeyMap};
use bdk_wallet::template::{
    Bip44, Bip44Public, Bip49, Bip49Public, Bip84, Bip84Public, Bip86, Bip86Public,
    DescriptorTemplate,
};
use bdk_wallet::KeychainKind;

use crate::error::MiniscriptError;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct Descriptor {
    pub extended_descriptor: ExtendedDescriptor,
    pub key_map: KeyMap,
}

impl Descriptor {
    pub(crate) fn new(descriptor: String, network: Network) -> Result<Self, DescriptorError> {
        let secp = Secp256k1::new();
        let (extended_descriptor, key_map) = descriptor.into_wallet_descriptor(&secp, network)?;
        Ok(Self {
            extended_descriptor,
            key_map,
        })
    }

    pub(crate) fn new_bip44(
        secret_key: &DescriptorSecretKey,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = &secret_key.0;

        match derivable_key {
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip44(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorSecretKey::MultiXPrv(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip44_public(
        public_key: &DescriptorPublicKey,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = &public_key.0;

        match derivable_key {
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip44Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorPublicKey::MultiXPub(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip49(
        secret_key: &DescriptorSecretKey,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = &secret_key.0;

        match derivable_key {
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip49(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorSecretKey::MultiXPrv(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip49_public(
        public_key: &DescriptorPublicKey,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = &public_key.0;

        match derivable_key {
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip49Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorPublicKey::MultiXPub(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip84(
        secret_key: &DescriptorSecretKey,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = &secret_key.0;

        match derivable_key {
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip84(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorSecretKey::MultiXPrv(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip84_public(
        public_key: &DescriptorPublicKey,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = &public_key.0;

        match derivable_key {
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip84Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorPublicKey::MultiXPub(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip86(
        secret_key: &DescriptorSecretKey,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = &secret_key.0;

        match derivable_key {
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip86(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorSecretKey::MultiXPrv(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn new_bip86_public(
        public_key: &DescriptorPublicKey,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = &public_key.0;

        match derivable_key {
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let (extended_descriptor, key_map, _) =
                    Bip86Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    extended_descriptor,
                    key_map,
                }
            }
            BdkDescriptorPublicKey::MultiXPub(_) => {
                unreachable!()
            }
        }
    }

    pub(crate) fn to_string_with_secret(&self) -> String {
        let descriptor = &self.extended_descriptor;
        let key_map = &self.key_map;
        descriptor.to_string_with_secret(key_map)
    }

    pub(crate) fn is_multipath(&self) -> bool {
        self.extended_descriptor.is_multipath()
    }

    pub(crate) fn to_single_descriptors(&self) -> Result<Vec<Arc<Descriptor>>, MiniscriptError> {
        self.extended_descriptor
            .clone()
            .into_single_descriptors()
            .map_err(MiniscriptError::from)
            .map(|descriptors| {
                descriptors
                    .into_iter()
                    .map(|desc| {
                        Arc::new(Descriptor {
                            extended_descriptor: desc,
                            key_map: self.key_map.clone(),
                        })
                    })
                    .collect()
            })
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.extended_descriptor)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use assert_matches::assert_matches;
    use bdk_wallet::bitcoin::Network;
    use bdk_wallet::KeychainKind;

    fn get_descriptor_secret_key() -> DescriptorSecretKey {
        let mnemonic = Mnemonic::from_string("chaos fabric time speed sponsor all flat solution wisdom trophy crack object robot pave observe combine where aware bench orient secret primary cable detect".to_string()).unwrap();
        DescriptorSecretKey::new(Network::Testnet, &mnemonic, None)
    }

    #[test]
    fn test_descriptor_templates() {
        let master: DescriptorSecretKey = get_descriptor_secret_key();
        println!("Master: {:?}", master.as_string());
        // tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h
        let handmade_public_44 = master
            .derive(&DerivationPath::new("m/44h/1h/0h".to_string()).unwrap())
            .unwrap()
            .as_public();
        println!("Public 44: {}", handmade_public_44.as_string());
        // Public 44: [d1d04177/44'/1'/0']tpubDCoPjomfTqh1e7o1WgGpQtARWtkueXQAepTeNpWiitS3Sdv8RKJ1yvTrGHcwjDXp2SKyMrTEca4LoN7gEUiGCWboyWe2rz99Kf4jK4m2Zmx/*
        let handmade_public_49 = master
            .derive(&DerivationPath::new("m/49h/1h/0h".to_string()).unwrap())
            .unwrap()
            .as_public();
        println!("Public 49: {}", handmade_public_49.as_string());
        // Public 49: [d1d04177/49'/1'/0']tpubDC65ZRvk1NDddHrVAUAZrUPJ772QXzooNYmPywYF9tMyNLYKf5wpKE7ZJvK9kvfG3FV7rCsHBNXy1LVKW95jrmC7c7z4hq7a27aD2sRrAhR/*
        let handmade_public_84 = master
            .derive(&DerivationPath::new("m/84h/1h/0h".to_string()).unwrap())
            .unwrap()
            .as_public();
        println!("Public 84: {}", handmade_public_84.as_string());
        // Public 84: [d1d04177/84'/1'/0']tpubDDNxbq17egjFk2edjv8oLnzxk52zny9aAYNv9CMqTzA4mQDiQq818sEkNe9Gzmd4QU8558zftqbfoVBDQorG3E4Wq26tB2JeE4KUoahLkx6/*
        let handmade_public_86 = master
            .derive(&DerivationPath::new("m/86h/1h/0h".to_string()).unwrap())
            .unwrap()
            .as_public();
        println!("Public 86: {}", handmade_public_86.as_string());
        // Public 86: [d1d04177/86'/1'/0']tpubDCJzjbcGbdEfXMWaL6QmgVmuSfXkrue7m2YNoacWwyc7a2XjXaKojRqNEbo41CFL3PyYmKdhwg2fkGpLX4SQCbQjCGxAkWHJTw9WEeenrJb/*
        let template_private_44 =
            Descriptor::new_bip44(&master, KeychainKind::External, Network::Testnet);
        let template_private_49 =
            Descriptor::new_bip49(&master, KeychainKind::External, Network::Testnet);
        let template_private_84 =
            Descriptor::new_bip84(&master, KeychainKind::External, Network::Testnet);
        let template_private_86 =
            Descriptor::new_bip86(&master, KeychainKind::External, Network::Testnet);
        // the extended public keys are the same when creating them manually as they are with the templates
        println!("Template 49: {}", template_private_49);
        println!("Template 44: {}", template_private_44);
        println!("Template 84: {}", template_private_84);
        println!("Template 86: {}", template_private_86);
        let template_public_44 = Descriptor::new_bip44_public(
            &handmade_public_44,
            "d1d04177".to_string(),
            KeychainKind::External,
            Network::Testnet,
        );
        let template_public_49 = Descriptor::new_bip49_public(
            &handmade_public_49,
            "d1d04177".to_string(),
            KeychainKind::External,
            Network::Testnet,
        );
        let template_public_84 = Descriptor::new_bip84_public(
            &handmade_public_84,
            "d1d04177".to_string(),
            KeychainKind::External,
            Network::Testnet,
        );
        let template_public_86 = Descriptor::new_bip86_public(
            &handmade_public_86,
            "d1d04177".to_string(),
            KeychainKind::External,
            Network::Testnet,
        );
        println!("Template public 49: {}", template_public_49);
        println!("Template public 44: {}", template_public_44);
        println!("Template public 84: {}", template_public_84);
        println!("Template public 86: {}", template_public_86);
        // when using a public key, both to_string and as_string_private return the same string
        assert_eq!(
            template_public_44.to_string_with_secret(),
            template_public_44.to_string()
        );
        assert_eq!(
            template_public_49.to_string_with_secret(),
            template_public_49.to_string()
        );
        assert_eq!(
            template_public_84.to_string_with_secret(),
            template_public_84.to_string()
        );
        assert_eq!(
            template_public_86.to_string_with_secret(),
            template_public_86.to_string()
        );
        // when using to_string on a private key, we get the same result as when using it on a public key
        assert_eq!(
            template_private_44.to_string(),
            template_public_44.to_string()
        );
        assert_eq!(
            template_private_49.to_string(),
            template_public_49.to_string()
        );
        assert_eq!(
            template_private_84.to_string(),
            template_public_84.to_string()
        );
        assert_eq!(
            template_private_86.to_string(),
            template_public_86.to_string()
        );
    }
    #[test]
    fn test_descriptor_from_string() {
        let descriptor1 = Descriptor::new("wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEftEaNzz7dPGhWuKFU4VULesmhEfZYyBXdE/0/*)".to_string(), Network::Testnet);
        let descriptor2 = Descriptor::new("wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEftEaNzz7dPGhWuKFU4VULesmhEfZYyBXdE/0/*)".to_string(), Network::Bitcoin);
        // Creating a Descriptor using an extended key that doesn't match the network provided will throw a DescriptorError::Key with inner InvalidNetwork error
        assert!(descriptor1.is_ok());
        assert_matches!(descriptor2.unwrap_err(), DescriptorError::Key { .. });
    }
}
