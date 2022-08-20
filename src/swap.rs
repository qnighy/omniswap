/// Swaps values of two references.
///
/// This is a **sentinel-based** swapping.
/// Therefore, unlike [`std::mem::swap`],
/// it can be applied to **possibly overlapping references**.
///
/// ## Basic usage
///
/// It has the same syntax as [`std::mem::swap`].
///
/// ```rust
/// let mut x = 'a';
/// let mut y = 'b';
/// omniswap::swap!(&mut x, &mut y);
/// assert_eq!((x, y), ('b', 'a'));
/// ```
///
/// ## Swapping with cells
///
/// It also supports [`std::cell::Cell`] and [`std::cell::RefCell`].
///
/// ```rust
/// # use std::cell::Cell;
/// let x = Cell::new('a');
/// let y = Cell::new('b');
/// omniswap::swap!(&x, &y);
/// assert_eq!((x.get(), y.get()), ('b', 'a'));
/// ```
///
/// The two references need not of the same type;
/// for example, you may swap values between `&mut T` and `&Cell<T>`.
///
/// ```rust
/// # use std::cell::Cell;
/// let mut x = 'a';
/// let y = Cell::new('b');
/// omniswap::swap!(&mut x, &y);
/// assert_eq!((x, y.get()), ('b', 'a'));
/// ```
///
/// ## Sentinel requirements
///
/// In order for `swap!` to work this way, it requires the value type
/// to satisfy [`Default`](std::default::Default) or [`Clone`](std::clone::Clone).
///
/// If multiple traits can apply, the behavior is determined
/// in the following order:
///
/// 1. If [`T: Copy`](std::marker::Copy) it copies the value from the first seference.
/// 2. Otherwise, if [`T: Default`](std::default::Default), it swaps the value with the default value.
/// 3. Otherwise, if [`T: Clone`](std::clone::Clone), it clones out the value from the first reference.
///
/// Additionally, if the first reference is [`&Cell<T>`](std::cell::Cell), `T` must satisfy
/// [`Default`](std::default::Default) or [`Copy`](std::marker::Copy). [`Clone`](std::clone::Clone) alone does not suffice.
///
/// ## Evaluation order
///
/// It evaluates the arguments in the order of appearance, and then
/// **the first argument again** to put the value back.
///
/// ## Alternatives
///
/// If the type does not have a good sentinel, you may need to use the following alternatives
/// that does not need a sentinel:
///
/// - [`std::mem::swap`]
/// - [`<[T]>::swap`][slice::swap()]
/// - [`Cell::swap`](std::cell::Cell::swap)
/// - [`RefCell::swap`](std::cell::RefCell::swap)
#[macro_export]
macro_rules! swap {
    ($x: expr, $y: expr) => {
        $crate::rotate!($x, $y)
    };
    ($x: expr, $y: expr,) => {
        $crate::rotate!($x, $y)
    };
}

/// A variant of [`swap!`] that works for more than two values.
///
/// ## Example
///
/// ```rust
/// let mut x = 'a';
/// let mut y = 'b';
/// let mut z = 'c';
/// omniswap::rotate!(&mut x, &mut y, &mut z);
/// assert_eq!((x, y, z), ('c', 'a', 'b'));
/// ```
#[macro_export]
macro_rules! rotate {
    ($x: expr, $($y: expr),*) => {
        {
            let value = $crate::take!($x);
            $(
                let value = $crate::Replace::replace($y, value);
            )*
            let _ = $crate::Replace::replace($x, value);
        }
    };
    ($x: expr) => {
        $crate::rotate!($x,)
    };
    ($x: expr, $($y: expr),*,) => {
        $crate::rotate!($x, $($y),*)
    };
}
