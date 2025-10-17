//! `libcheriot` is a helper library to be able to run simple Rust tests on CHERIoT.
//! It exposes an allocator, a panic handler and various simple utilities.
//!
//! `libcheriot` is a `#![no_std]` library, but uses `core` and also uses `alloc`.

// We don't use `std`...
#![no_std]

// ...but we want `alloc` for some basic interaction with the user.
extern crate alloc;

mod global_allocator;

/// Bare-bones implementation of `prinln!` and friends.
pub mod print;

/// Bare-bones PRNG.
pub mod rand;

/// Panic handlers.
mod panic;
pub use panic::*;
