#![deny(unsafe_op_in_unsafe_fn)]

pub mod abi;

#[path = "../unsupported/os.rs"]
pub mod os;

#[path = "../unsupported/common.rs"]
mod common;
pub use common::*;
