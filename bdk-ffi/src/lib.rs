mod bitcoin;
mod descriptor;
mod electrum;
mod error;
mod esplora;
mod keys;
mod kyoto;
mod macros;
mod store;
mod tx_builder;
mod types;
mod wallet;

use crate::bitcoin::FeeRate;
use crate::bitcoin::OutPoint;
use crate::error::AddressParseError;
use crate::error::Bip32Error;
use crate::error::Bip39Error;
use crate::error::CreateTxError;
use crate::error::DescriptorError;
use crate::error::DescriptorKeyError;
use crate::error::ElectrumError;
use crate::error::EsploraError;
use crate::error::MiniscriptError;
use crate::error::ParseAmountError;
use crate::error::PsbtError;
use crate::error::PsbtFinalizeError;
use crate::error::TransactionError;

use bdk_wallet::bitcoin::Network;

uniffi::include_scaffolding!("bdk");
