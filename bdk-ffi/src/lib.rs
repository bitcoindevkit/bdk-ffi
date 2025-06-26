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

use bdk_wallet::bitcoin::Network;

uniffi::include_scaffolding!("bdk");
