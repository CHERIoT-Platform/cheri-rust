//! CHERI-specific intrinsics.

use crate::marker::PointeeSized;

// intrinsic names that dependent on the size of addresses.
#[cfg(target_address_width = "32")]
mod intrinsics {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.offset.increment.32"]
        pub fn __builtin_cheri_address_increment(ptr: *const (), increment: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.address.set.32"]
        pub fn __builtin_cheri_address_set(ptr: *const (), address: usize) -> *const ();

       #[link_name = "llvm.cheri.cap.base.get.32"]
        pub fn __builtin_cheri_base_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.bounds.set.32"]
        pub fn __builtin_cheri_bounds_set(ptr: *const (), bounds: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.bounds.set.exact.32"]
        pub fn __builtin_cheri_bounds_set_exact(ptr: *const (), bounds: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.length.get.32"]
        pub fn __builtin_cheri_length_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.perms.and.32"]
        pub fn __builtin_cheri_perms_and(ptr: *const (), perms: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.perms.get.32"]
        pub fn __builtin_cheri_perms_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.representable.alignment.mask.32"]
        pub fn __builtin_cheri_representable_alignment_mask(align: usize) -> usize;

        #[link_name = "llvm.cheri.round.representable.length.32"]
        pub fn __builtin_cheri_round_representable_length(len: usize) -> usize;

        #[link_name = "llvm.cheri.cap.top.get.32"]
        pub fn __builtin_cheri_top_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.type.get.32"]
        pub fn __builtin_cheri_type_get(ptr: *const ()) -> usize;
    }
}

#[cfg(target_address_width = "64")]
mod intrinsics {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.offset.increment.64"]
        pub fn __builtin_cheri_address_increment(ptr: *const (), increment: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.address.set.64"]
        pub fn __builtin_cheri_address_set(ptr: *const (), address: usize) -> *const ();

       #[link_name = "llvm.cheri.cap.base.get.64"]
        pub fn __builtin_cheri_base_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.bounds.set.64"]
        pub fn __builtin_cheri_bounds_set(ptr: *const (), bounds: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.bounds.set.exact.64"]
        pub fn __builtin_cheri_bounds_set_exact(ptr: *const (), bounds: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.length.get.64"]
        pub fn __builtin_cheri_length_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.perms.and.64"]
        pub fn __builtin_cheri_perms_and(ptr: *const (), perms: usize) -> *const ();

        #[link_name = "llvm.cheri.cap.perms.get.64"]
        pub fn __builtin_cheri_perms_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.representable.alignment.mask.64"]
        pub fn __builtin_cheri_representable_alignment_mask(align: usize) -> usize;

        #[link_name = "llvm.cheri.round.representable.length.64"]
        pub fn __builtin_cheri_round_representable_length(len: usize) -> usize;

        #[link_name = "llvm.cheri.cap.top.get.64"]
        pub fn __builtin_cheri_top_get(ptr: *const ()) -> usize;

        #[link_name = "llvm.cheri.cap.type.get.64"]
        pub fn __builtin_cheri_type_get(ptr: *const ()) -> usize;
    }
}

use intrinsics::*;

/// Increment the offset of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub unsafe fn address_increment<T: PointeeSized>(ptr: *const T, increment: usize) -> *const T {
    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let thin_ptr =
        unsafe { __builtin_cheri_address_increment(core::mem::transmute(thin_ptr), increment) };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Set the address of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub unsafe fn address_set<T: PointeeSized>(ptr: *const T, new_address: usize) -> *const T {
    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let thin_ptr: *const () =
        unsafe { __builtin_cheri_address_set(core::mem::transmute(thin_ptr), new_address) };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Get the address of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub fn address_get<T: PointeeSized>(ptr: *const T) -> usize {

    #[rustc_intrinsic]
    #[rustc_nounwind]
    fn cheri_address_get<T>(ptr: *const T) -> usize;

    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    cheri_address_get(thin_ptr)
}

/// Retrieve the base of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub fn base_get<T: PointeeSized>(ptr: *const T) -> usize {
    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_base_get(core::mem::transmute(thin_ptr)) }
}

/// Set the bounds of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub unsafe fn bounds_set<T: PointeeSized>(ptr: *const T, bounds: usize) -> *const T {
    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let thin_ptr = unsafe { __builtin_cheri_bounds_set(core::mem::transmute(thin_ptr), bounds) };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Set the bounds of the capability without any rounding.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline(always)]
pub unsafe fn bounds_set_exact<T: PointeeSized>(ptr: *const T, bounds: usize) -> *const T {
    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let thin_ptr =
        unsafe { __builtin_cheri_bounds_set_exact(core::mem::transmute(thin_ptr), bounds) };

    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Compare two capabilities for exact equality.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn is_equal_exact<A: PointeeSized, B: PointeeSized>(left: *const A, right: *const B) -> bool {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.equal.exact"]
        pub fn __builtin_cheri_is_equal_exact(left: *const (), right: *const ()) -> bool;
    }
    let (thin_left_ptr, _metadata) = left.to_raw_parts();
    let (thin_right_ptr, _metadata) = right.to_raw_parts();

    unsafe {
        __builtin_cheri_is_equal_exact(
            core::mem::transmute(thin_left_ptr),
            core::mem::transmute(thin_right_ptr),
        )
    }
}

/// Retrieve the length of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn length_get<T: PointeeSized>(ptr: *const T) -> usize {
    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_length_get(core::mem::transmute(thin_ptr)) }
}

/// Restrict the permissions of the capability (computing the logical and).
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub unsafe fn permissions_and<T: PointeeSized>(ptr: *const T, perms: usize) -> *const T {
    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let thin_ptr = unsafe { __builtin_cheri_perms_and(core::mem::transmute(thin_ptr), perms) };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Get the raw permissions of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn permissions_get<T: PointeeSized>(ptr: *const T) -> usize {
    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_perms_get(core::mem::transmute(thin_ptr)) }
}

/// Get the representable alignment mask for the given length.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn representable_alignment_mask(len: usize) -> usize {
    unsafe { __builtin_cheri_representable_alignment_mask(len) }
}

/// Get the rounded representable length for the given length.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn round_representable_length(len: usize) -> usize {
    unsafe { __builtin_cheri_round_representable_length(len) }
}

/// Hardware-seal the capability with the given key.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub unsafe fn seal<T: PointeeSized, K: PointeeSized>(ptr: *const T, key: *const K) -> *const T {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.seal"]
        pub fn __builtin_cheri_seal(ptr: *const (), key: *const ()) -> *const ();
    }

    let (thin_ptr, metadata) = ptr.to_raw_parts();
    let (thin_key_ptr, _metadata) = key.to_raw_parts();

    let thin_ptr = unsafe {
        __builtin_cheri_seal(
            core::mem::transmute(thin_ptr),
            core::mem::transmute(thin_key_ptr),
        )
    };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Hardware-unseal the capability with the given key.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub unsafe fn unseal<T: PointeeSized, K: PointeeSized>(
    sealed_ptr: *const T,
    key: *const K,
) -> *const T {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.unseal"]
        pub fn __builtin_cheri_unseal(sealed_ptr: *const (), key: *const ()) -> *const ();
    }

    let (thin_ptr, metadata) = sealed_ptr.to_raw_parts();
    let (thin_key_ptr, _metadata) = key.to_raw_parts();

    let thin_ptr = unsafe {
        __builtin_cheri_unseal(
            core::mem::transmute(thin_ptr),
            core::mem::transmute(thin_key_ptr),
        )
    };

    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Test if `left` is a subset of `right`.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn subset_test<A: PointeeSized, B: PointeeSized>(left: *const A, right: *const B) -> bool {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.subset.test"]
        pub fn __builtin_cheri_subset_test(left: *const (), right: *const ()) -> bool;
    }

    let (thin_left_ptr, _metadata) = left.to_raw_parts();
    let (thin_right_ptr, _metadata) = right.to_raw_parts();

    unsafe {
        __builtin_cheri_subset_test(
            core::mem::transmute(thin_left_ptr),
            core::mem::transmute(thin_right_ptr),
        )
    }
}

/// Clear the tag of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub unsafe fn tag_clear<T: PointeeSized>(ptr: *const T) -> *const T {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.tag.clear"]
        pub fn __builtin_cheri_tag_clear(ptr: *const ()) -> *const ();
    }

    let (thin_ptr, metadata) = ptr.to_raw_parts();

    let thin_ptr = unsafe { __builtin_cheri_tag_clear(core::mem::transmute(thin_ptr)) };
    core::ptr::from_raw_parts(thin_ptr, metadata)
}

/// Get the tag of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn tag_get<T: PointeeSized>(ptr: *const T) -> bool {
    unsafe extern "C" {
        #[link_name = "llvm.cheri.cap.tag.get"]
        pub fn __builtin_cheri_tag_get(ptr: *const ()) -> bool;
    }

    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_tag_get(core::mem::transmute(thin_ptr)) }
}

/// Retrieve the top of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn top_get<T: PointeeSized>(ptr: *const T) -> usize {
    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_top_get(core::mem::transmute(thin_ptr)) }
}

/// Get the type of the capability.
#[unstable(feature = "stdarch_cheri", issue = "1")]
#[inline]
#[rustc_nounwind]
pub fn type_get<T: PointeeSized>(ptr: *const T) -> usize {
    let (thin_ptr, _metadata) = ptr.to_raw_parts();
    unsafe { __builtin_cheri_type_get(core::mem::transmute(thin_ptr)) }
}

/// Create a pointer without provenance metadata from the given value.
#[inline]
#[rustc_intrinsic]
#[rustc_nounwind]
#[rustc_intrinsic_const_stable_indirect]
#[unstable(feature = "stdarch_cheri", issue = "1")]
pub const fn cheri_without_provenance<T>(value: usize) -> *mut T;
