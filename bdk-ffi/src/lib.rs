mod bitcoin;
mod descriptor;
#[cfg(not(target_arch = "wasm32"))]
mod electrum;
mod error;
mod esplora;
mod keys;
#[cfg(not(target_arch = "wasm32"))]
mod kyoto;
mod macros;
mod signer;
mod store;
mod tx_builder;
mod types;
mod wallet;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
use crate::bitcoin::FeeRate;
use crate::bitcoin::OutPoint;

// This is required because the wasm runtime exports are otherwise dropped by the linker
#[cfg(target_arch = "wasm32")]
extern crate uniffi_runtime_wasm as _;

uniffi::setup_scaffolding!("bdk");
