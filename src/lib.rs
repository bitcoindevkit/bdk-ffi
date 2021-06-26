#![deny(unsafe_code)] /* No `unsafe` needed! */

mod blockchain;
mod database;
mod error;
mod wallet;

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .with_guard("__RUST_BDK_FFI__")
        .to_file("cc/bdk_ffi.h")?
        .generate()
}
