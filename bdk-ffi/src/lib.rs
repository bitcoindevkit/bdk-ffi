pub mod bitcoin;
pub mod descriptor;
pub mod electrum;
pub mod error;
pub mod esplora;
pub mod keys;
pub mod kyoto;
pub mod macros;
pub mod store;
pub mod tx_builder;
pub mod types;
pub mod wallet;

#[cfg(test)]
mod tests;

use crate::bitcoin::FeeRate;
use crate::bitcoin::OutPoint;

uniffi::setup_scaffolding!("bdk");
