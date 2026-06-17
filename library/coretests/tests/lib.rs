#![cfg_attr(target_abi = "cheriot", no_main)]
#![cfg_attr(target_abi = "cheriot", feature(custom_test_frameworks))]
#![cfg_attr(target_abi = "cheriot", test_runner(test::run_tests))]
#![cfg_attr(target_abi = "cheriot", reexport_test_harness_main = "test_main")]
// tidy-alphabetical-start
#![cfg_attr(not(panic = "abort"), feature(reentrant_lock))]
#![cfg_attr(target_has_atomic = "128", feature(integer_atomics))]
#![feature(array_ptr_get)]
#![feature(array_try_from_fn)]
#![feature(array_try_map)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(async_iter_from_iter)]
#![feature(async_iterator)]
#![feature(borrowed_buf_init)]
#![feature(bstr)]
#![feature(casefold)]
#![feature(cfg_target_has_reliable_f16_f128)]
#![feature(char_internals)]
#![feature(clone_to_uninit)]
#![feature(cmp_minmax)]
#![feature(const_array)]
#![feature(const_bool)]
#![feature(const_cell_traits)]
#![feature(const_clone)]
#![feature(const_closures)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![feature(const_drop_in_place)]
#![feature(const_eval_select)]
#![feature(const_index)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_option_ops)]
#![feature(const_ref_cell)]
#![feature(const_result_trait_fn)]
#![feature(const_select_unpredictable)]
#![feature(const_trait_impl)]
#![feature(const_unsigned_bigint_helpers)]
#![feature(core_intrinsics)]
#![feature(core_intrinsics_fallbacks)]
#![feature(core_io)]
#![feature(core_io_borrowed_buf)]
#![feature(core_private_bignum)]
#![feature(core_private_diy_float)]
#![feature(cstr_display)]
#![feature(debug_closure_helpers)]
#![feature(dec2flt)]
#![feature(drop_guard)]
#![feature(duration_constants)]
#![feature(duration_constructors)]
#![feature(exact_div)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(extern_types)]
#![feature(f16)]
#![feature(f128)]
#![feature(float_exact_integer_constants)]
#![feature(float_gamma)]
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
#![feature(int_from_ascii)]
#![feature(int_roundings)]
#![feature(integer_casts)]
#![feature(io_slice_as_bytes)]
#![feature(ip)]
#![feature(is_ascii_octdigit)]
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
#![feature(macro_metavar_expr_concat)]
#![feature(maybe_uninit_fill)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(min_specialization)]
#![feature(never_type)]
#![feature(next_index)]
#![feature(non_exhaustive_omitted_patterns_lint)]
#![feature(num_internals)]
#![feature(numfmt)]
#![feature(one_sided_range)]
#![feature(panic_internals)]
#![feature(pattern)]
#![feature(pointer_is_aligned_to)]
#![feature(portable_simd)]
#![feature(ptr_metadata)]
#![feature(rustc_attrs)]
#![feature(signed_bigint_helpers)]
#![feature(slice_from_ptr_range)]
#![feature(slice_index_methods)]
#![feature(slice_internals)]
#![feature(slice_partition_dedup)]
#![feature(slice_shift)]
#![feature(slice_split_once)]
#![feature(sliceindex_wrappers)]
#![feature(split_array)]
#![feature(split_as_slice)]
#![feature(std_internals)]
#![feature(step_trait)]
#![feature(str_internals)]
#![feature(strict_provenance_lints)]
#![feature(titlecase)]
#![feature(trusted_len)]
#![feature(trusted_random_access)]
#![feature(try_blocks)]
#![feature(try_find)]
#![feature(try_from_int_error_kind)]
#![feature(try_trait_v2)]
#![feature(type_info)]
#![feature(uint_carryless_mul)]
#![feature(uint_gather_scatter_bits)]
#![feature(unicode_internals)]
#![feature(unsize)]
#![feature(unwrap_infallible)]
// tidy-alphabetical-end
#![allow(internal_features)]
#![deny(fuzzy_provenance_casts)]
#![deny(lossy_provenance_casts)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Version of `assert_matches` that ignores fancy runtime printing in const context and uses structural equality.
#[cfg(any(not(target_abi = "cheriot"), feature = "test_num"))]
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
#[cfg(any(not(target_abi = "cheriot"), feature = "test_num"))]
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

#[cfg(any(not(target_abi = "cheriot"), feature = "test_alloc"))]
mod alloc;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_any"))]
mod any;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_array"))]
mod array;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_ascii"))]
mod ascii;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_ascii_char"))]
mod ascii_char;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_asserting"))]
mod asserting;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_async_iter"))]
mod async_iter;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_atomic"))]
mod atomic;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_bool"))]
mod bool;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_bstr"))]
mod bstr;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_cell"))]
mod cell;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_char"))]
mod char;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_clone"))]
mod clone;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_cmp"))]
mod cmp;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_const_ptr"))]
mod const_ptr;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_convert"))]
mod convert;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_ffi"))]
mod ffi;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_fmt"))]
mod fmt;
#[cfg(all(target_abi = "cheriot", feature = "test_fmt_builders"))]
#[path = "fmt/builders.rs"]
mod fmt_builders;
#[cfg(all(target_abi = "cheriot", feature = "test_fmt_float"))]
#[path = "fmt/float.rs"]
mod fmt_float;
#[cfg(all(target_abi = "cheriot", feature = "test_fmt_num"))]
#[path = "fmt/num.rs"]
mod fmt_num;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_future"))]
mod future;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_hash"))]
mod hash;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_hint"))]
mod hint;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_index"))]
mod index;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_intrinsics"))]
mod intrinsics;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_io"))]
mod io;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_iter"))]
mod iter;
#[cfg(all(target_abi = "cheriot", feature = "test_iter_adapters"))]
#[path = "iter/adapters/mod.rs"]
mod iter_adapters;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_lazy"))]
mod lazy;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_macros"))]
mod macros;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_manually_drop"))]
mod manually_drop;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_mem"))]
mod mem;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_net"))]
mod net;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_nonzero"))]
mod nonzero;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_num"))]
mod num;
#[cfg(all(target_abi = "cheriot", feature = "test_num_rest"))]
#[path = "num/rest.rs"]
mod num_rest;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_ops"))]
mod ops;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_option"))]
mod option;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_panic"))]
mod panic;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_pattern"))]
mod pattern;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_pin"))]
mod pin;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_pin_macro"))]
mod pin_macro;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_ptr"))]
mod ptr;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_result"))]
mod result;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_simd"))]
mod simd;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_slice"))]
mod slice;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_str"))]
mod str;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_str_lossy"))]
mod str_lossy;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_task"))]
mod task;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_time"))]
mod time;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_tuple"))]
mod tuple;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_unicode"))]
mod unicode;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_waker"))]
mod waker;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_wtf8"))]
mod wtf8;

/// Copied from `std::test_helpers::test_rng`, see that function for rationale.
#[track_caller]
#[allow(dead_code)] // Not used in all configurations.
pub(crate) fn test_rng() -> rand_xorshift::XorShiftRng {
    use core::hash::{BuildHasher, Hash, Hasher};
    let mut hasher = std::hash::RandomState::new().build_hasher();
    core::panic::Location::caller().hash(&mut hasher);
    let hc64 = hasher.finish();
    let seed_vec = hc64.to_le_bytes().into_iter().chain(0u8..8).collect::<Vec<u8>>();
    let seed: [u8; 16] = seed_vec.as_slice().try_into().unwrap();
    rand::SeedableRng::from_seed(seed)
}

#[cfg(target_abi = "cheriot")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> i32 {
    test_main();
    return 0;
}
