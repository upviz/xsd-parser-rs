// NOTE: `yaserde_derive` can trigger `non_local_definitions` warnings on recent Rust,
// which become hard errors under `-D warnings` when compiling this crate's unit tests.
#![cfg_attr(test, allow(non_local_definitions))]

pub mod types;
pub mod utils;

// Required for macro-generated code to find this crate under its name.
extern crate self as xsd_types;
