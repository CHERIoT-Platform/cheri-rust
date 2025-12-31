//! Some intrinsics specific to CHERI systems.

// Note that since the removal of the `PointerLike` marker type,
// these types can't be generic over the kind of pointer, unless a CHERI-specific `CapabilityLike`
// trait is introduced.
//
// For now, we use *const ().

/// Retrieve the address of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_address_get(ptr: *const ()) -> usize;

/// Increment the offset of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_address_increment(ptr: *const (), offset: usize) -> *const ();

/// Set the address of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_address_set(ptr: *const (), addr: usize) -> *const ();

/// Retrieve the base of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_base_get(ptr: *const ()) -> usize;

/// Set the bounds of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_bounds_set(ptr: *const (), bounds: usize) -> *const ();

/// Set the bounds of the capability without any rounding.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_bounds_set_exact(ptr: *const (), bounds: usize) -> *const ();

/// Compare two capabilities for exact equality.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_is_equal_exact(ptr1: *const (), ptr2: *const ()) -> bool;

/// Retrieve the length of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_length_get(ptr: *const ()) -> usize;

/// Augment the permissions of the capability (computing the logical and).
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_permissions_and(ptr: *const (), perms: usize) -> *const ();

/// Get the raw permissions of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_permissions_get(ptr: *const ()) -> usize;

/// Get the representable alignment mask for the given length.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_representable_alignment_mask(len: usize) -> usize;

/// Get the rounded representable length for the given length.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_round_representable_length(len: usize) -> usize;

/// Hardware-seal the capability with the given key.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_seal(ptr: *const (), key: *const ()) -> *const ();

/// Test if `ptr1` is a subset of `ptr2`.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_subset_test(ptr1: *const (), ptr2: *const ()) -> bool;

/// Clear the tag of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_tag_clear(ptr: *const ());

/// Get the tag of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_tag_get(ptr: *const ()) -> bool;

/// Retrieve the top of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_top_get(ptr: *const ()) -> usize;

/// Get the type of the capability.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub fn cheri_type_get(ptr: *const ()) -> u32;

/// Hardware-unseal the capability with the given key.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn cheri_unseal(ptr: *const (), key: *const ()) -> *const ();

/// Create a pointer without provenance metadata from the given value.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
#[rustc_intrinsic_const_stable_indirect]
pub const fn cheri_without_provenance<T>(value: usize) -> *mut T;
