use core::cell::{Cell, RefCell};
use core::mem;
use core::ops::{Deref, DerefMut};

/// Takes out the value from a reference.
///
/// ## Usage
///
/// ```rust
/// let mut x = 42;
/// assert_eq!(omniswap::take!(&mut x), 42);
/// ```
///
/// It also supports references to cells:
/// [`&Cell<T>`](std::cell::Cell) and [`&RefCell<T>`](std::cell::RefCell).
///
/// ## Requirements
///
/// It requires the value type to satisfy [`Default`](std::default::Default) or [`Clone`](std::clone::Clone).
///
/// If multiple traits can apply, the behavior is determined
/// in the following order:
///
/// 1. If [`T: Copy`](std::marker::Copy) it copies the value from the reference.
/// 2. Otherwise, if [`T: Default`](std::default::Default), it swaps the value with the default value.
/// 3. Otherwise, if [`T: Clone`](std::clone::Clone), it clones out the value from the reference.
///
/// Additionally, if the reference is [`&Cell<T>`](std::cell::Cell), `T` must satisfy
/// [`Default`](std::default::Default) or [`Copy`](std::marker::Copy). [`Clone`](std::clone::Clone) alone does not suffice.
#[macro_export]
macro_rules! take {
    ($p: expr) => {
        $crate::TakeHelper::new($p).take()
    };
    ($p: expr,) => {
        $crate::take!($p)
    };
}

/// Internal type used in [`take!`].
///
/// Please use [`take!`] instead.
pub struct TakeHelper<T>(TakeHelper2<T>);

/// Internal type used in [`take!`].
///
/// Please use [`take!`] instead.
pub struct TakeHelper2<T>(TakeHelper3<T>);

/// Internal type used in [`take!`].
///
/// Please use [`take!`] instead.
pub struct TakeHelper3<T>(T);

impl<T> TakeHelper<T> {
    pub fn new(inner: T) -> Self {
        TakeHelper(TakeHelper2(TakeHelper3(inner)))
    }
}

impl<'a, T> TakeHelper<&'a mut T>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        *self.0 .0 .0
    }
}

impl<'a, T> TakeHelper<&'a Cell<T>>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        self.0 .0 .0.get()
    }
}

impl<'a, T> TakeHelper<&'a RefCell<T>>
where
    T: Copy,
{
    pub fn take(&mut self) -> T {
        let r = self.0 .0 .0.borrow();
        *r
    }
}

impl<T> Deref for TakeHelper<T> {
    type Target = TakeHelper2<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TakeHelper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TakeHelper2<&'a mut T>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        mem::take(self.0 .0)
    }
}

impl<'a, T> TakeHelper2<&'a Cell<T>>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        self.0 .0.take()
    }
}

impl<'a, T> TakeHelper2<&'a RefCell<T>>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        let mut r = self.0 .0.borrow_mut();
        mem::take(&mut *r)
    }
}

impl<T> Deref for TakeHelper2<T> {
    type Target = TakeHelper3<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TakeHelper2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TakeHelper3<&'a mut T>
where
    T: Clone,
{
    pub fn take(&mut self) -> T {
        self.0.clone()
    }
}

impl<'a, T> TakeHelper3<&'a RefCell<T>>
where
    T: Clone,
{
    pub fn take(&mut self) -> T {
        let r = self.0.borrow();
        r.clone()
    }
}
