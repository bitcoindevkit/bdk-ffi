use ::safer_ffi::prelude::*;

/// A `struct` usable from both Rust and C
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

/* Export a Rust function to the C world. */
/// Returns the middle point of `[a, b]`.
#[ffi_export]
fn mid_point(a: Option<repr_c::Box<Point>>, b: Option<repr_c::Box<Point>>) -> repr_c::Box<Point> {
    let a = a.unwrap();
    let b = b.unwrap();
    repr_c::Box::new(Point {
        x: (a.x + b.x) / 2.,
        y: (a.y + b.y) / 2.,
    })
}

/// Pretty-prints a point using Rust's formatting logic.
#[ffi_export]
fn print_point(point: Option<repr_c::Box<Point>>) {
    println!("{:?}", point);
}

#[ffi_export]
fn new_point(x: f64, y: f64) -> repr_c::Box<Point> {
    repr_c::Box::new(Point { x, y })
}

#[ffi_export]
fn free_point(point: Option<repr_c::Box<Point>>) {
    drop(point)
}

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("bdk_ffi.h")?
        .generate()
}
