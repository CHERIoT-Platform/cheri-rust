#![cfg_attr(target_abi = "cheriot", no_main)]
#![cfg_attr(target_abi = "cheriot", feature(custom_test_frameworks))]
#![cfg_attr(target_abi = "cheriot", test_runner(test::run_tests))]
#![cfg_attr(target_abi = "cheriot", reexport_test_harness_main = "test_main")]
#![cfg_attr(target_abi = "cheriot", allow(unused_features))]
#![feature(allocator_api)]
#![feature(binary_heap_pop_if)]
#![feature(const_heap)]
#![feature(deque_extend_front)]
#![feature(iter_array_chunks)]
#![feature(casefold)]
#![feature(cow_is_borrowed)]
#![feature(core_intrinsics)]
#![feature(downcast_unchecked)]
#![feature(exact_size_is_empty)]
#![feature(hashmap_internals)]
#![feature(linked_list_cursors)]
#![feature(map_try_insert)]
#![feature(pattern)]
#![feature(trusted_len)]
#![feature(try_reserve_kind)]
#![feature(try_with_capacity)]
#![feature(unboxed_closures)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(binary_heap_drain_sorted)]
#![feature(slice_ptr_get)]
#![feature(slice_range)]
#![feature(slice_partial_sort_unstable)]
#![feature(inplace_iteration)]
#![feature(iter_advance_by)]
#![feature(iter_next_chunk)]
#![feature(slice_partition_dedup)]
#![feature(string_from_utf8_lossy_owned)]
#![feature(string_remove_matches)]
#![feature(const_btree_len)]
#![feature(const_trait_impl)]
#![feature(test)]
#![feature(thin_box)]
#![feature(drain_keep_rest)]
#![feature(local_waker)]
#![feature(str_as_str)]
#![feature(strict_provenance_lints)]
#![feature(string_replace_in_place)]
#![feature(vec_deque_truncate_front)]
#![feature(unique_rc_arc)]
#![feature(macro_metavar_expr_concat)]
#![feature(vec_peek_mut)]
#![feature(vec_try_remove)]
#![feature(ptr_cast_slice)]
#![allow(internal_features)]
#![deny(implicit_provenance_casts)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use std::hash::{DefaultHasher, Hash, Hasher};

#[cfg(any(not(target_abi = "cheriot"), feature = "test_alloc_test"))]
mod alloc_test;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_arc"))]
mod arc;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_autotraits"))]
mod autotraits;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_borrow"))]
mod borrow;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_boxed"))]
mod boxed;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_btree_set_hash"))]
mod btree_set_hash;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_c_str"))]
mod c_str;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_c_str2"))]
mod c_str2;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_collections"))]
mod collections;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_const_fns"))]
mod const_fns;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_cow_str"))]
mod cow_str;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_fmt"))]
mod fmt;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_heap"))]
mod heap;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_linked_list"))]
mod linked_list;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_misc_tests"))]
mod misc_tests;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_num"))]
mod num;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_rc"))]
mod rc;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_slice"))]
mod slice;
#[cfg(any(
    not(target_abi = "cheriot"),
    any(
        feature = "test_sort_misc",
        feature = "test_sort_i32",
        feature = "test_sort_u64",
        feature = "test_sort_u128",
        feature = "test_sort_cell_i32",
        feature = "test_sort_string",
        feature = "test_sort_f128",
        feature = "test_sort_1k",
        feature = "test_sort_dyn",
        feature = "test_sort_panic_retain",
        feature = "test_sort_panic_observable",
        feature = "test_sort_observable",
        feature = "test_sort_stability",
        feature = "test_sort_det",
        feature = "test_sort_cmp",
        feature = "test_sort_ord",
    )
))]
mod sort;
#[cfg(any(
    not(target_abi = "cheriot"),
    any(feature = "test_str_1", feature = "test_str_2", feature = "test_str_3")
))]
mod str;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_string"))]
mod string;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_sync"))]
mod sync;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_task"))]
mod task;
// Helpers/macros
mod testing;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_thin_box"))]
mod thin_box;
#[cfg(any(not(target_abi = "cheriot"), any(feature = "test_vec_1", feature = "test_vec_2"),))]
mod vec;
#[cfg(any(
    not(target_abi = "cheriot"),
    any(feature = "test_vec_deque_1", feature = "test_vec_deque_2")
))]
mod vec_deque;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Copied from `std::test_helpers::test_rng`, since these tests rely on the
/// seed not being the same for every RNG invocation too.
fn test_rng() -> rand_xorshift::XorShiftRng {
    use std::hash::{BuildHasher, Hash, Hasher};
    let mut hasher = std::hash::RandomState::new().build_hasher();
    std::panic::Location::caller().hash(&mut hasher);
    let hc64 = hasher.finish();
    let seed_vec = hc64.to_le_bytes().into_iter().chain(0u8..8).collect::<Vec<u8>>();
    let seed: [u8; 16] = seed_vec.as_slice().try_into().unwrap();
    rand::SeedableRng::from_seed(seed)
}

#[test]
#[cfg(any(not(target_abi = "cheriot"), feature = "test_extras"))]
fn test_boxed_hasher() {
    let ordinary_hash = hash(&5u32);

    let mut hasher_1 = Box::new(DefaultHasher::new());
    5u32.hash(&mut hasher_1);
    assert_eq!(ordinary_hash, hasher_1.finish());

    let mut hasher_2 = Box::new(DefaultHasher::new()) as Box<dyn Hasher>;
    5u32.hash(&mut hasher_2);
    assert_eq!(ordinary_hash, hasher_2.finish());
}

#[cfg(target_abi = "cheriot")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> i32 {
    test_main();
    return 0;
}
