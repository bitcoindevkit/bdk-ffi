mod bitcoin;
mod descriptor;
mod electrum;
mod error;
mod esplora;
mod keys;
mod kyoto;
mod macros;
mod signer;
#[cfg(feature = "experimental-silent-payments")]
mod silent_payments;
mod store;
mod tx_builder;
mod types;
mod wallet;

#[cfg(test)]
mod tests;

use crate::bitcoin::FeeRate;
use crate::bitcoin::OutPoint;

uniffi::setup_scaffolding!("bdk");
