#![deny(unsafe_code)] /* No `unsafe` needed! */

mod wallet;

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("bdk_ffi.h")?
        .generate()
}
