// This file could also be called nested/mod.rs
// Which approach you choose is down largely to preference but using a file
// with the same name as the module is recommended by the Rust developers.
pub mod inner;

pub use inner::deeply_nested_fn;

pub fn nested_fn() {
    println!("nested_fn");
}
