pub trait Sort {
    fn name() -> String;

    fn sort<T>(v: &mut [T])
    where
        T: Ord;

    fn sort_by<T, F>(v: &mut [T], compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering;
}

mod ffi_types;
mod known_good_stable_sort;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_sort_misc"))]
mod partial;
mod patterns;
mod tests;
mod zipf;
