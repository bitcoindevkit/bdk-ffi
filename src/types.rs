use ::safer_ffi::prelude::*;
use safer_ffi::char_p::char_p_boxed;

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct FfiResult<T> {
    pub ok: Option<repr_c::Box<T>>,
    pub err: Option<repr_c::Box<char_p_boxed>>,
}

#[derive_ReprC]
#[repr(C)]
pub struct FfiResultVec<T> {
    pub ok: repr_c::Vec<T>,
    pub err: Option<repr_c::Box<char_p_boxed>>,
}

#[ffi_export]
fn free_string_result(string_result: FfiResult<char_p_boxed>) {
    drop(string_result)
}

#[ffi_export]
fn free_void_result(void_result: FfiResult<()>) {
    drop(void_result)
}

// TODO do we need this? remove?
/// Frees a Rust-allocated string
#[ffi_export]
fn free_string(string: Option<char_p_boxed>) {
    drop(string)
}
