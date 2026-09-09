#[path = "cast_table.rs"]
mod cast_table;
use cast_table::ORDERED_VALS;

macro_rules! make_checked_cast_test {
    ($Src:ident as [$($Dst:ident),*]) => {$(
        #[test]
        #[allow(non_snake_case)]
        fn ${concat(test_checked_cast_, $Src, _to_, $Dst)}() {
            for val in ORDERED_VALS.iter() {
                if let Some(src) = val.parse::<$Src>().ok() {
                    let dst: Option<$Dst> = val.parse().ok();
                    assert_eq!(src.checked_cast::<$Dst>(), dst);
                }
            }
        }
    )*}
}

macro_rules! make_bounded_cast_test {
    (|$src:ident| $raw:expr, $Src:ident as [$($Dst:ident),*]) => {$(
        #[test]
        #[allow(non_snake_case)]
        fn ${concat(test_bounded_cast_, $Src, _to_, $Dst)}() {
            let ord_idx = |s| ORDERED_VALS.iter().position(|v| *v == s).unwrap();
            let dst_min_idx = ord_idx(<$Dst>::MIN.to_string());
            let dst_max_idx = ord_idx(<$Dst>::MAX.to_string());
            for (val_idx, val) in ORDERED_VALS.iter().enumerate() {
                if let Some($src) = val.parse::<$Src>().ok() {
                    let dst: Option<$Dst> = val.parse().ok();

                    assert_eq!($src.wrapping_cast::<$Dst>(), $raw as $Dst);

                    if val_idx > dst_max_idx {
                        assert_eq!($src.saturating_cast::<$Dst>(), <$Dst>::MAX);
                    } else if val_idx < dst_min_idx {
                        assert_eq!($src.saturating_cast::<$Dst>(), <$Dst>::MIN);
                    } else {
                        assert_eq!($src.saturating_cast::<$Dst>(), dst.unwrap());
                    }
                }
            }
        }
    )*}
}

macro_rules! make_tests_for_src {
    (|$src:ident| $raw:expr, [$($Src:ident),*]) => {$(
        #[cfg(all(target_abi = "cheriot", feature = "test_num_cast_checked_uint"))]
        make_checked_cast_test!(             $Src as [u8, u16, u32, u64, u128, usize]);
        #[cfg(all(target_abi = "cheriot", feature = "test_num_cast_checked_int"))]
        make_checked_cast_test!(             $Src as [i8, i16, i32, i64, i128, isize]);
        #[cfg(all(target_abi = "cheriot", feature = "test_num_cast_bounded_uint"))]
        make_bounded_cast_test!(|$src| $raw, $Src as [u8, u16, u32, u64, u128, usize]);
        #[cfg(all(target_abi = "cheriot", feature = "test_num_cast_bounded_int"))]
        make_bounded_cast_test!(|$src| $raw, $Src as [i8, i16, i32, i64, i128, isize]);

        #[cfg(not(target_abi = "cheriot"))]
        make_checked_cast_test!(             $Src as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
        #[cfg(not(target_abi = "cheriot"))]
        make_bounded_cast_test!(|$src| $raw, $Src as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);

        // NonZero types are not (yet) implemented.
        // make_checked_cast_test!($Src as [
        //     NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        //     NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
        // ]);
    )*}
}

make_tests_for_src!(|x| x, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);

// NonZero types are not (yet) implemented.
// make_tests_for_src!(
//     |x| x.get(),
//     [
//         NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
//         NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
//     ]
// );
