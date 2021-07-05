use std::convert::TryFrom;

use ::safer_ffi::prelude::*;
use safer_ffi::char_p::char_p_boxed;

// Non-opaque returned values

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransactionDetails {
    // TODO Optional transaction
    // pub transaction: Option<Transaction>,
    /// Transaction id
    pub txid: char_p_boxed,
    /// Timestamp
    pub timestamp: u64,
    /// Received value (sats)
    pub received: u64,
    /// Sent value (sats)
    pub sent: u64,
    /// Fee value (sats)
    pub fees: u64,
    /// Confirmed in block height, `None` means unconfirmed
    pub height: i32,
}

impl From<&bdk::TransactionDetails> for TransactionDetails {
    fn from(op: &bdk::TransactionDetails) -> Self {
        TransactionDetails {
            txid: char_p_boxed::try_from(op.txid.to_string()).unwrap(),
            timestamp: op.timestamp,
            received: op.received,
            sent: op.sent,
            fees: op.fees,
            height: op.height.map(|h| h as i32).unwrap_or(-1),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OutPoint {
    /// The referenced transaction's txid, as hex string
    pub txid: char_p_boxed,
    /// The index of the referenced output in its transaction's vout
    pub vout: u32,
}

impl From<&bdk::bitcoin::OutPoint> for OutPoint {
    fn from(op: &bdk::bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: char_p_boxed::try_from(op.txid.to_string()).unwrap(),
            vout: op.vout,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TxOut {
    /// The value of the output, in satoshis
    pub value: u64,
    /// The script which must satisfy for the output to be spent, as hex string
    pub script_pubkey: char_p_boxed,
}

impl From<&bdk::bitcoin::TxOut> for TxOut {
    fn from(to: &bdk::bitcoin::TxOut) -> Self {
        TxOut {
            value: to.value,
            script_pubkey: char_p_boxed::try_from(to.script_pubkey.to_string()).unwrap(),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LocalUtxo {
    /// Reference to a transaction output
    pub outpoint: OutPoint,
    /// Transaction output
    pub txout: TxOut,
    /// Type of keychain, as short 0 for "external" or 1 for "internal"
    pub keychain: u16,
}

impl From<&bdk::LocalUtxo> for LocalUtxo {
    fn from(lu: &bdk::LocalUtxo) -> Self {
        LocalUtxo {
            outpoint: OutPoint::from(&lu.outpoint),
            txout: TxOut::from(&lu.txout),
            keychain: lu.keychain as u16,
        }
    }
}
