//! Note
//! ----
//! You're probably viewing this file because you're adding a test (or you might
//! just be browsing, in that case, hey there!).
//!
//! The iter test suite is split into two big modules, and some miscellaneous
//! smaller modules. The two big modules are `adapters` and `traits`.
//!
//! `adapters` are for methods on `Iterator` that adapt the data inside the
//! iterator, whether it be by emitting another iterator or returning an item
//! from inside the iterator after executing a closure on each item.
//!
//! `traits` are for trait's that extend an `Iterator` (and the `Iterator`
//! trait itself, mostly containing miscellaneous methods). For the most part,
//! if a test in `traits` uses a specific adapter, then it should be moved to
//! that adapter's test file in `adapters`.

extern crate alloc;
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
use alloc::vec::Vec;

#[cfg(any(not(feature = "partial_test"), feature = "test_iter_adapters"))]
mod adapters;
#[cfg(any(not(feature = "partial_test"), feature = "test_iter_range"))]
mod range;
#[cfg(any(not(feature = "partial_test"), feature = "test_iter_sources"))]
mod sources;
#[cfg(any(not(feature = "partial_test"), feature = "test_iter_traits"))]
mod traits;

#[cfg(any(not(feature = "partial_test"), feature = "test_iter_sources"))]
use core::cell::Cell; // iter, adapters, range, traits
#[cfg(any(
    not(feature = "partial_test"),
    any(feature = "test_iter", feature = "test_iter_range", feature = "test_iter_sources")
))]
use core::iter::*;

#[cfg(any(
    not(feature = "partial_test"),
    any(feature = "test_iter_range", feature = "test_iter_sources")
))]
pub fn is_trusted_len<I: TrustedLen>(_: I) {} // iter, adapters

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_multi_iter() {
    let xs = [1, 2, 3, 4];
    let ys = [4, 3, 2, 1];
    assert!(xs.iter().eq(ys.iter().rev()));
    assert!(xs.iter().lt(xs.iter().skip(2)));
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_counter_from_iter() {
    let it = (0..).step_by(5).take(10);
    let xs: Vec<isize> = FromIterator::from_iter(it);
    assert_eq!(xs, [0, 5, 10, 15, 20, 25, 30, 35, 40, 45]);
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_functor_laws() {
    // identity:
    fn identity<T>(x: T) -> T {
        x
    }
    assert_eq!((0..10).map(identity).sum::<usize>(), (0..10).sum());

    // composition:
    fn f(x: usize) -> usize {
        x + 3
    }
    fn g(x: usize) -> usize {
        x * 2
    }
    fn h(x: usize) -> usize {
        g(f(x))
    }
    assert_eq!((0..10).map(f).map(g).sum::<usize>(), (0..10).map(h).sum());
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_monad_laws_left_identity() {
    fn f(x: usize) -> impl Iterator<Item = usize> {
        (0..10).map(move |y| x * y)
    }
    assert_eq!(once(42).flat_map(f.clone()).sum::<usize>(), f(42).sum());
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_monad_laws_right_identity() {
    assert_eq!((0..10).flat_map(|x| once(x)).sum::<usize>(), (0..10).sum());
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
fn test_monad_laws_associativity() {
    fn f(x: usize) -> impl Iterator<Item = usize> {
        0..x
    }
    fn g(x: usize) -> impl Iterator<Item = usize> {
        (0..x).rev()
    }
    assert_eq!(
        (0..10).flat_map(f).flat_map(g).sum::<usize>(),
        (0..10).flat_map(|x| f(x).flat_map(g)).sum::<usize>()
    );
}

#[test]
#[cfg(any(not(feature = "partial_test"), feature = "test_iter"))]
pub fn extend_for_unit() {
    let mut x = 0;
    {
        let iter = (0..5).map(|_| {
            x += 1;
        });
        ().extend(iter);
    }
    assert_eq!(x, 5);
}
