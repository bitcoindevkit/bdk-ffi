use bdk::bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bdk::bitcoin::blockdata::script::ScriptBuf as BdkScriptBuf;
use bdk::bitcoin::blockdata::transaction::TxOut as BdkTxOut;
use bdk::bitcoin::consensus::Decodable;
use bdk::bitcoin::network::constants::Network as BdkNetwork;
use bdk::bitcoin::psbt::PartiallySignedTransaction as BdkPartiallySignedTransaction;
use bdk::bitcoin::Address as BdkAddress;
use bdk::bitcoin::OutPoint as BdkOutPoint;
use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::bitcoin::Txid;
use bdk::Error as BdkError;

use std::io::Cursor;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub(crate) BdkScriptBuf);

impl Script {
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BdkScriptBuf = raw_output_script.into();
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<BdkScriptBuf> for Script {
    fn from(script: BdkScriptBuf) -> Self {
        Script(script)
    }
}

#[derive(PartialEq, Debug)]
pub enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<Network> for BdkNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BdkNetwork::Bitcoin,
            Network::Testnet => BdkNetwork::Testnet,
            Network::Signet => BdkNetwork::Signet,
            Network::Regtest => BdkNetwork::Regtest,
        }
    }
}

impl From<BdkNetwork> for Network {
    fn from(network: BdkNetwork) -> Self {
        match network {
            BdkNetwork::Bitcoin => Network::Bitcoin,
            BdkNetwork::Testnet => Network::Testnet,
            BdkNetwork::Signet => Network::Signet,
            BdkNetwork::Regtest => Network::Regtest,
            _ => panic!("Network {} not supported", network),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    inner: BdkAddress<NetworkChecked>,
}

impl Address {
    pub fn new(address: String, network: Network) -> Result<Self, BdkError> {
        let parsed_address = address
            .parse::<bdk::bitcoin::Address<NetworkUnchecked>>()
            .map_err(|e| BdkError::Generic(e.to_string()))?;

        let network_checked_address = parsed_address
            .require_network(network.into())
            .map_err(|e| BdkError::Generic(e.to_string()))?;

        Ok(Address {
            inner: network_checked_address,
        })
    }

    /// alternative constructor
    // fn from_script(script: Arc<Script>, network: Network) -> Result<Self, BdkError> {
    //     BdkAddress::from_script(&script.inner, network)
    //         .map(|a| Address { inner: a })
    //         .map_err(|e| BdkError::Generic(e.to_string()))
    // }
    //
    // fn payload(&self) -> Payload {
    //     match &self.inner.payload.clone() {
    //         BdkPayload::PubkeyHash(pubkey_hash) => Payload::PubkeyHash {
    //             pubkey_hash: pubkey_hash.to_vec(),
    //         },
    //         BdkPayload::ScriptHash(script_hash) => Payload::ScriptHash {
    //             script_hash: script_hash.to_vec(),
    //         },
    //         BdkPayload::WitnessProgram { version, program } => Payload::WitnessProgram {
    //             version: *version,
    //             program: program.clone(),
    //         },
    //     }
    // }

    pub fn network(&self) -> Network {
        self.inner.network.into()
    }

    pub fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script(self.inner.script_pubkey()))
    }

    pub fn to_qr_uri(&self) -> String {
        self.inner.to_qr_uri()
    }

    pub fn as_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn is_valid_for_network(&self, network: Network) -> bool {
        let address_str = self.inner.to_string();
        if let Ok(unchecked_address) = address_str.parse::<BdkAddress<NetworkUnchecked>>() {
            unchecked_address.is_valid_for_network(network.into())
        } else {
            false
        }
    }
}

impl From<Address> for BdkAddress {
    fn from(address: Address) -> Self {
        address.inner
    }
}

impl From<BdkAddress> for Address {
    fn from(address: BdkAddress) -> Self {
        Address { inner: address }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    inner: BdkTransaction,
}

impl Transaction {
    pub fn new(transaction_bytes: Vec<u8>) -> Result<Self, BdkError> {
        let mut decoder = Cursor::new(transaction_bytes);
        let tx: BdkTransaction = BdkTransaction::consensus_decode(&mut decoder)
            .map_err(|e| BdkError::Generic(e.to_string()))?;
        Ok(Transaction { inner: tx })
    }

    pub fn txid(&self) -> String {
        self.inner.txid().to_string()
    }

    // fn weight(&self) -> u64 {
    //     self.inner.weight() as u64
    // }

    pub fn size(&self) -> u64 {
        self.inner.size() as u64
    }

    pub fn vsize(&self) -> u64 {
        self.inner.vsize() as u64
    }

    // fn serialize(&self) -> Vec<u8> {
    //     self.inner.serialize()
    // }

    pub fn is_coin_base(&self) -> bool {
        self.inner.is_coin_base()
    }

    pub fn is_explicitly_rbf(&self) -> bool {
        self.inner.is_explicitly_rbf()
    }

    pub fn is_lock_time_enabled(&self) -> bool {
        self.inner.is_lock_time_enabled()
    }

    pub fn version(&self) -> i32 {
        self.inner.version
    }

    // fn lock_time(&self) -> u32 {
    //     self.inner.lock_time.0
    // }

    // fn input(&self) -> Vec<TxIn> {
    //     self.inner.input.iter().map(|x| x.into()).collect()
    // }
    //
    // fn output(&self) -> Vec<TxOut> {
    //     self.inner.output.iter().map(|x| x.into()).collect()
    // }
}

impl From<BdkTransaction> for Transaction {
    fn from(tx: BdkTransaction) -> Self {
        Transaction { inner: tx }
    }
}

impl From<&BdkTransaction> for Transaction {
    fn from(tx: &BdkTransaction) -> Self {
        Transaction { inner: tx.clone() }
    }
}

impl From<&Transaction> for BdkTransaction {
    fn from(tx: &Transaction) -> Self {
        tx.inner.clone()
    }
}

pub struct PartiallySignedTransaction {
    pub(crate) inner: Mutex<BdkPartiallySignedTransaction>,
}

impl PartiallySignedTransaction {
    pub(crate) fn new(psbt_base64: String) -> Result<Self, BdkError> {
        let psbt: BdkPartiallySignedTransaction =
            BdkPartiallySignedTransaction::from_str(&psbt_base64)
                .map_err(|e| BdkError::Generic(e.to_string()))?;

        Ok(PartiallySignedTransaction {
            inner: Mutex::new(psbt),
        })
    }

    pub(crate) fn serialize(&self) -> String {
        let psbt = self.inner.lock().unwrap().clone();
        psbt.to_string()
    }

    // pub(crate) fn txid(&self) -> String {
    //     let tx = self.inner.lock().unwrap().clone().extract_tx();
    //     let txid = tx.txid();
    //     txid.to_hex()
    // }

    pub(crate) fn extract_tx(&self) -> Arc<Transaction> {
        let tx = self.inner.lock().unwrap().clone().extract_tx();
        Arc::new(tx.into())
    }

    // /// Combines this PartiallySignedTransaction with other PSBT as described by BIP 174.
    // ///
    // /// In accordance with BIP 174 this function is commutative i.e., `A.combine(B) == B.combine(A)`
    // pub(crate) fn combine(
    //     &self,
    //     other: Arc<PartiallySignedTransaction>,
    // ) -> Result<Arc<PartiallySignedTransaction>, BdkError> {
    //     let other_psbt = other.inner.lock().unwrap().clone();
    //     let mut original_psbt = self.inner.lock().unwrap().clone();
    //
    //     original_psbt.combine(other_psbt)?;
    //     Ok(Arc::new(PartiallySignedTransaction {
    //         inner: Mutex::new(original_psbt),
    //     }))
    // }

    // /// The total transaction fee amount, sum of input amounts minus sum of output amounts, in Sats.
    // /// If the PSBT is missing a TxOut for an input returns None.
    // pub(crate) fn fee_amount(&self) -> Option<u64> {
    //     self.inner.lock().unwrap().fee_amount()
    // }

    // /// The transaction's fee rate. This value will only be accurate if calculated AFTER the
    // /// `PartiallySignedTransaction` is finalized and all witness/signature data is added to the
    // /// transaction.
    // /// If the PSBT is missing a TxOut for an input returns None.
    // pub(crate) fn fee_rate(&self) -> Option<Arc<FeeRate>> {
    //     self.inner.lock().unwrap().fee_rate().map(Arc::new)
    // }

    // /// Serialize the PSBT data structure as a String of JSON.
    // pub(crate) fn json_serialize(&self) -> String {
    //     let psbt = self.inner.lock().unwrap();
    //     serde_json::to_string(psbt.deref()).unwrap()
    // }
}

impl From<BdkPartiallySignedTransaction> for PartiallySignedTransaction {
    fn from(psbt: BdkPartiallySignedTransaction) -> Self {
        PartiallySignedTransaction {
            inner: Mutex::new(psbt),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

impl From<&OutPoint> for BdkOutPoint {
    fn from(outpoint: &OutPoint) -> Self {
        BdkOutPoint {
            txid: Txid::from_str(&outpoint.txid).unwrap(),
            vout: outpoint.vout,
        }
    }
}

impl From<&BdkOutPoint> for OutPoint {
    fn from(outpoint: &BdkOutPoint) -> Self {
        OutPoint {
            txid: outpoint.txid.to_string(),
            vout: outpoint.vout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Arc<Script>,
}

impl From<&BdkTxOut> for TxOut {
    fn from(tx_out: &BdkTxOut) -> Self {
        TxOut {
            value: tx_out.value,
            script_pubkey: Arc::new(Script(tx_out.script_pubkey.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitcoin::Address;
    use crate::bitcoin::Network;

    #[test]
    fn test_is_valid_for_network() {
        // ====Docs tests====
        // https://docs.rs/bitcoin/0.29.2/src/bitcoin/util/address.rs.html#798-802

        let docs_address_testnet_str = "2N83imGV3gPwBzKJQvWJ7cRUY2SpUyU6A5e";
        let docs_address_testnet =
            Address::new(docs_address_testnet_str.to_string(), Network::Testnet).unwrap();
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );
        assert_ne!(
            docs_address_testnet.network(),
            Network::Bitcoin,
            "Address should not be parsed as Bitcoin"
        );

        let docs_address_mainnet_str = "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf";
        let docs_address_mainnet =
            Address::new(docs_address_mainnet_str.to_string(), Network::Bitcoin).unwrap();
        assert!(
            docs_address_mainnet.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert_ne!(
            docs_address_mainnet.network(),
            Network::Testnet,
            "Address should not be valid for Testnet"
        );
        assert_ne!(
            docs_address_mainnet.network(),
            Network::Signet,
            "Address should not be valid for Signet"
        );
        assert_ne!(
            docs_address_mainnet.network(),
            Network::Regtest,
            "Address should not be valid for Regtest"
        );

        // ====Bech32====

        //     | Network         | Prefix  | Address Type |
        //     |-----------------|---------|--------------|
        //     | Bitcoin Mainnet | `bc1`   | Bech32       |
        //     | Bitcoin Testnet | `tb1`   | Bech32       |
        //     | Bitcoin Signet  | `tb1`   | Bech32       |
        //     | Bitcoin Regtest | `bcrt1` | Bech32       |

        // Bech32 - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Signet
        // - Regtest
        let bitcoin_mainnet_bech32_address_str = "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj";
        let bitcoin_mainnet_bech32_address = Address::new(
            bitcoin_mainnet_bech32_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should not be valid for Signet"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // Bech32 - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        // - Regtest
        let bitcoin_testnet_bech32_address_str =
            "tb1p4nel7wkc34raczk8c4jwk5cf9d47u2284rxn98rsjrs4w3p2sheqvjmfdh";
        let bitcoin_testnet_bech32_address = Address::new(
            bitcoin_testnet_bech32_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            !bitcoin_testnet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not not be valid for Regtest"
        );

        // Bech32 - Signet
        // Valid for:
        // - Signet
        // - Testnet
        // Not valid for:
        // - Bitcoin
        // - Regtest
        let bitcoin_signet_bech32_address_str =
            "tb1pwzv7fv35yl7ypwj8w7al2t8apd6yf4568cs772qjwper74xqc99sk8x7tk";
        let bitcoin_signet_bech32_address = Address::new(
            bitcoin_signet_bech32_address_str.to_string(),
            Network::Signet,
        )
        .unwrap();
        assert!(
            !bitcoin_signet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_signet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_signet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            !bitcoin_signet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not not be valid for Regtest"
        );

        // Bech32 - Regtest
        // Valid for:
        // - Regtest
        // Not valid for:
        // - Bitcoin
        // - Testnet
        // - Signet
        let bitcoin_regtest_bech32_address_str = "bcrt1q39c0vrwpgfjkhasu5mfke9wnym45nydfwaeems";
        let bitcoin_regtest_bech32_address = Address::new(
            bitcoin_regtest_bech32_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Signet),
            "Address should not be valid for Signet"
        );
        assert!(
            bitcoin_regtest_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // ====P2PKH====

        //     | Network                            | Prefix for P2PKH | Prefix for P2SH |
        //     |------------------------------------|------------------|-----------------|
        //     | Bitcoin Mainnet                    | `1`              | `3`             |
        //     | Bitcoin Testnet, Regtest, Signet   | `m` or `n`       | `2`             |

        // P2PKH - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Regtest
        let bitcoin_mainnet_p2pkh_address_str = "1FfmbHfnpaZjKFvyi1okTjJJusN455paPH";
        let bitcoin_mainnet_p2pkh_address = Address::new(
            bitcoin_mainnet_p2pkh_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // P2PKH - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_testnet_p2pkh_address_str = "mucFNhKMYoBQYUAEsrFVscQ1YaFQPekBpg";
        let bitcoin_testnet_p2pkh_address = Address::new(
            bitcoin_testnet_p2pkh_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // P2PKH - Regtest
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_regtest_p2pkh_address_str = "msiGFK1PjCk8E6FXeoGkQPTscmcpyBdkgS";
        let bitcoin_regtest_p2pkh_address = Address::new(
            bitcoin_regtest_p2pkh_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // ====P2SH====

        //     | Network                            | Prefix for P2PKH | Prefix for P2SH |
        //     |------------------------------------|------------------|-----------------|
        //     | Bitcoin Mainnet                    | `1`              | `3`             |
        //     | Bitcoin Testnet, Regtest, Signet   | `m` or `n`       | `2`             |

        // P2SH - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Regtest
        let bitcoin_mainnet_p2sh_address_str = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let bitcoin_mainnet_p2sh_address = Address::new(
            bitcoin_mainnet_p2sh_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // P2SH - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_testnet_p2sh_address_str = "2NFUBBRcTJbYc1D4HSCbJhKZp6YCV4PQFpQ";
        let bitcoin_testnet_p2sh_address = Address::new(
            bitcoin_testnet_p2sh_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // P2SH - Regtest
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_regtest_p2sh_address_str = "2NEb8N5B9jhPUCBchz16BB7bkJk8VCZQjf3";
        let bitcoin_regtest_p2sh_address = Address::new(
            bitcoin_regtest_p2sh_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );
    }
}
