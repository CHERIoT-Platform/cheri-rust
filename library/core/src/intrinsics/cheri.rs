//! Some intrinsics specific to CHERI systems.

/// Create an pointer without provenance metadata from the given value.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
#[rustc_intrinsic_const_stable_indirect]
pub const fn cheri_without_provenance<T>(value: usize) -> *mut T;

/// Retrieve the address of the pointer.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_address_get(ptr: *const ()) -> usize;
