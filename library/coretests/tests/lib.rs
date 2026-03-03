#![cfg_attr(feature = "partial_test", no_std)]
#![cfg_attr(feature = "partial_test", no_main)]
#![cfg_attr(feature = "partial_test", feature(custom_test_frameworks))]
#![cfg_attr(feature = "partial_test", test_runner(test::run_tests))]
#![cfg_attr(feature = "partial_test", reexport_test_harness_main = "test_main")]
// tidy-alphabetical-start
#![cfg_attr(target_has_atomic = "128", feature(integer_atomics))]
#![cfg_attr(test, feature(cfg_select))]
#![feature(alloc_layout_extra)]
#![feature(array_ptr_get)]
#![feature(array_try_from_fn)]
#![feature(array_try_map)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(async_iter_from_iter)]
#![feature(async_iterator)]
#![feature(bigint_helper_methods)]
#![feature(bool_to_result)]
#![feature(bstr)]
#![feature(cfg_target_has_reliable_f16_f128)]
#![feature(char_internals)]
#![feature(char_max_len)]
#![feature(clamp_magnitude)]
#![feature(clone_to_uninit)]
#![feature(const_array)]
#![feature(const_cell_traits)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_drop_in_place)]
#![feature(const_eval_select)]
#![feature(const_index)]
#![feature(const_ops)]
#![feature(const_option_ops)]
#![feature(const_ref_cell)]
#![feature(const_result_trait_fn)]
#![feature(const_select_unpredictable)]
#![feature(const_trait_impl)]
#![feature(control_flow_ok)]
#![feature(core_float_math)]
#![feature(core_intrinsics)]
#![feature(core_intrinsics_fallbacks)]
#![feature(core_io_borrowed_buf)]
#![feature(core_private_bignum)]
#![feature(core_private_diy_float)]
#![feature(cstr_display)]
#![feature(debug_closure_helpers)]
#![feature(dec2flt)]
#![feature(drop_guard)]
#![feature(duration_constants)]
#![feature(duration_constructors)]
#![feature(error_generic_member_access)]
#![feature(exact_div)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(extern_types)]
#![feature(f16)]
#![feature(f128)]
#![feature(float_algebraic)]
// #![feature(float_gamma)]
#![feature(float_minimum_maximum)]
#![feature(flt2dec)]
#![feature(fmt_internals)]
#![feature(formatting_options)]
#![feature(freeze)]
#![feature(funnel_shifts)]
#![feature(future_join)]
#![feature(generic_assert_internals)]
#![feature(hasher_prefixfree_extras)]
#![feature(hashmap_internals)]
#![feature(int_lowest_highest_one)]
#![feature(int_roundings)]
#![feature(ip)]
#![feature(is_ascii_octdigit)]
#![feature(isolate_most_least_significant_one)]
#![feature(iter_advance_by)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(iter_intersperse)]
#![feature(iter_is_partitioned)]
#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(iter_order_by)]
#![feature(iter_partition_in_place)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]
#![feature(layout_for_ptr)]
#![feature(maybe_uninit_fill)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(min_specialization)]
#![feature(never_type)]
#![feature(new_range_api)]
#![feature(next_index)]
#![feature(non_exhaustive_omitted_patterns_lint)]
#![feature(numfmt)]
#![feature(one_sided_range)]
#![feature(option_reduce)]
#![feature(pattern)]
#![feature(pointer_is_aligned_to)]
#![feature(portable_simd)]
#![feature(ptr_metadata)]
#![feature(result_option_map_or_default)]
#![feature(slice_from_ptr_range)]
#![feature(slice_index_methods)]
#![feature(slice_internals)]
#![feature(slice_partition_dedup)]
#![feature(slice_split_once)]
#![feature(sliceindex_wrappers)]
#![feature(split_array)]
#![feature(split_as_slice)]
#![feature(std_internals)]
#![feature(step_trait)]
#![feature(str_internals)]
#![feature(strict_provenance_lints)]
#![feature(test)]
#![feature(trusted_len)]
#![feature(trusted_random_access)]
#![feature(try_blocks)]
#![feature(try_find)]
#![feature(try_trait_v2)]
#![feature(type_info)]
#![feature(uint_bit_width)]
#![feature(uint_gather_scatter_bits)]
#![feature(unsize)]
#![feature(unwrap_infallible)]
// tidy-alphabetical-end
#![allow(internal_features)]
#![deny(fuzzy_provenance_casts)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Version of `assert_matches` that ignores fancy runtime printing in const context and uses structural equality.
#[cfg(any(
    not(feature = "partial_test"),
    any(feature = "test_num_int", feature = "test_num_uint")
))]
macro_rules! assert_eq_const_safe {
    ($t:ty: $left:expr, $right:expr) => {
        assert_eq_const_safe!($t: $left, $right, concat!(stringify!($left), " == ", stringify!($right)));
    };
    ($t:ty: $left:expr, $right:expr$(, $($arg:tt)+)?) => {
        {
            fn runtime() {
                assert_eq!($left, $right, $($($arg)*),*);
            }
            const fn compiletime() {
                const PAT: $t = $right;
                assert!(matches!($left, PAT), $($($arg)*),*);
            }
            core::intrinsics::const_eval_select((), compiletime, runtime)
        }
    };
}

/// Creates a test for runtime and a test for constant-time.
#[cfg(any(
    not(feature = "partial_test"),
    any(feature = "test_num_int", feature = "test_num_uint")
))]
macro_rules! test_runtime_and_compiletime {
    ($(
        $(#[$attr:meta])*
        fn $test:ident() $block:block
    )*) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test() $block
            $(#[$attr])*
            const _: () = $block;
        )*
    }
}

#[cfg(any(not(feature = "partial_test"), feature = "test_alloc"))]
mod alloc;
#[cfg(any(not(feature = "partial_test"), feature = "test_any"))]
mod any;
#[cfg(any(not(feature = "partial_test"), feature = "test_array"))]
mod array;
#[cfg(any(not(feature = "partial_test"), feature = "test_ascii"))]
mod ascii;
#[cfg(any(not(feature = "partial_test"), feature = "test_ascii_char"))]
mod ascii_char;
#[cfg(any(not(feature = "partial_test"), feature = "test_asserting"))]
mod asserting;
#[cfg(any(not(feature = "partial_test"), feature = "test_async_iter"))]
mod async_iter;
#[cfg(any(not(feature = "partial_test"), feature = "test_atomic"))]
mod atomic;
#[cfg(any(not(feature = "partial_test"), feature = "test_bool"))]
mod bool;
#[cfg(any(not(feature = "partial_test"), feature = "test_bstr"))]
mod bstr;
#[cfg(any(not(feature = "partial_test"), feature = "test_cell"))]
mod cell;
#[cfg(any(not(feature = "partial_test"), feature = "test_char"))]
mod char;
#[cfg(any(not(feature = "partial_test"), feature = "test_clone"))]
mod clone;
#[cfg(any(not(feature = "partial_test"), feature = "test_cmp"))]
mod cmp;
#[cfg(any(not(feature = "partial_test"), feature = "test_const_ptr"))]
mod const_ptr;
#[cfg(any(not(feature = "partial_test"), feature = "test_convert"))]
mod convert;
#[cfg(any(not(feature = "partial_test"), feature = "test_ffi"))]
mod ffi;
#[cfg(any(not(feature = "partial_test"), feature = "test_fmt"))]
#[cfg(not(target_abi = "cheriot"))] // FIXME: lots of std
mod floats;
#[cfg(any(
    not(feature = "partial_test"),
    any(
        feature = "test_fmt",
        feature = "test_fmt_builders",
        feature = "test_fmt_float",
        feature = "test_fmt_num"
    )
))]
mod fmt;
#[cfg(any(not(feature = "partial_test"), feature = "test_future"))]
#[cfg(not(target_abi = "cheriot"))] // FIXME: lots of std
mod future;
#[cfg(any(not(feature = "partial_test"), feature = "test_hash"))]
mod hash;
#[cfg(any(not(feature = "partial_test"), feature = "test_hint"))]
mod hint;
#[cfg(any(not(feature = "partial_test"), feature = "test_index"))]
mod index;
#[cfg(any(not(feature = "partial_test"), feature = "test_intrinsics"))]
mod intrinsics;
#[cfg(any(not(feature = "partial_test"), feature = "test_io"))]
mod io;
#[cfg(any(
    not(feature = "partial_test"),
    any(
        feature = "test_iter",
        feature = "test_iter_adapters",
        feature = "test_iter_range",
        feature = "test_iter_sources",
        feature = "test_iter_traits",
    )
))]
mod iter;
#[cfg(any(not(feature = "partial_test"), feature = "test_lazy"))]
mod lazy;
#[cfg(any(not(feature = "partial_test"), feature = "test_macros"))]
mod macros;
#[cfg(any(not(feature = "partial_test"), feature = "test_manually_drop"))]
mod manually_drop;
#[cfg(any(not(feature = "partial_test"), feature = "test_mem"))]
mod mem;
#[cfg(any(not(feature = "partial_test"), feature = "test_net"))]
mod net;
#[cfg(any(not(feature = "partial_test"), feature = "test_nonzero"))]
mod nonzero;
#[cfg(any(
    not(feature = "partial_test"),
    any(
        feature = "test_num",
        feature = "test_num_int",
        feature = "test_num_uint",
        feature = "test_num_rest"
    )
))]
mod num;
#[cfg(any(not(feature = "partial_test"), feature = "test_ops"))]
mod ops;
#[cfg(any(not(feature = "partial_test"), feature = "test_option"))]
mod option;
#[cfg(any(not(feature = "partial_test"), feature = "test_panic"))]
mod panic;
#[cfg(any(not(feature = "partial_test"), feature = "test_pattern"))]
mod pattern;
#[cfg(any(not(feature = "partial_test"), feature = "test_pin"))]
mod pin;
#[cfg(any(not(feature = "partial_test"), feature = "test_pin_macro"))]
mod pin_macro;
#[cfg(any(not(feature = "partial_test"), feature = "test_ptr"))]
mod ptr;
#[cfg(any(not(feature = "partial_test"), feature = "test_result"))]
mod result;
#[cfg(any(not(feature = "partial_test"), feature = "test_simd"))]
mod simd;
#[cfg(any(not(feature = "partial_test"), feature = "test_slice"))]
mod slice;
#[cfg(any(not(feature = "partial_test"), feature = "test_str"))]
mod str;
#[cfg(any(not(feature = "partial_test"), feature = "test_str_lossy"))]
mod str_lossy;
#[cfg(any(not(feature = "partial_test"), feature = "test_task"))]
mod task;
#[cfg(any(not(feature = "partial_test"), feature = "test_time"))]
mod time;
#[cfg(any(not(feature = "partial_test"), feature = "test_tuple"))]
mod tuple;
#[cfg(any(not(feature = "partial_test"), feature = "test_unicode"))]
mod unicode;
#[cfg(any(not(feature = "partial_test"), feature = "test_waker"))]
mod waker;
#[cfg(any(not(feature = "partial_test"), feature = "test_wtf8"))]
mod wtf8;

/// Copied from `std::test_helpers::test_rng`, see that function for rationale.
#[track_caller]
#[allow(dead_code)] // Not used in all configurations.
#[cfg(not(feature = "partial_test"))]
pub(crate) fn test_rng() -> rand_xorshift::XorShiftRng {
    use core::hash::{BuildHasher, Hash, Hasher};
    let mut hasher = std::hash::RandomState::new().build_hasher();
    core::panic::Location::caller().hash(&mut hasher);
    let hc64 = hasher.finish();
    let seed_vec = hc64.to_le_bytes().into_iter().chain(0u8..8).collect::<Vec<u8>>();
    let seed: [u8; 16] = seed_vec.as_slice().try_into().unwrap();
    rand::SeedableRng::from_seed(seed)
}

#[cfg(feature = "partial_test")]
#[unsafe(no_mangle)]
pub extern "C" fn __rust_main() -> i32 {
    test_main();
    return 0;
}
