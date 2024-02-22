mod bitcoin;
mod descriptor;
mod error;
mod esplora;
mod keys;
mod types;
mod wallet;

use crate::bitcoin::Address;
use crate::bitcoin::OutPoint;
use crate::bitcoin::PartiallySignedTransaction;
use crate::bitcoin::Script;
use crate::bitcoin::Transaction;
use crate::bitcoin::TxOut;
use crate::descriptor::Descriptor;
use crate::error::Alpha3Error;
use crate::error::CalculateFeeError;
use crate::error::EsploraError;
use crate::esplora::EsploraClient;
use crate::keys::DerivationPath;
use crate::keys::DescriptorPublicKey;
use crate::keys::DescriptorSecretKey;
use crate::keys::Mnemonic;
use crate::types::AddressIndex;
use crate::types::AddressInfo;
use crate::types::Balance;
use crate::types::FeeRate;
use crate::types::LocalOutput;
use crate::types::ScriptAmount;
use crate::wallet::BumpFeeTxBuilder;
use crate::wallet::SentAndReceivedValues;
use crate::wallet::TxBuilder;
use crate::wallet::Update;
use crate::wallet::Wallet;

use crate::error::WalletCreationError;
use bdk::bitcoin::Network;
use bdk::keys::bip39::WordCount;
use bdk::wallet::tx_builder::ChangeSpendPolicy;
use bdk::KeychainKind;

uniffi::include_scaffolding!("bdk");

// TODO: TxIn, Payload
