use std::cell::{Cell, RefCell};
use std::mem;

/// Takes out the value from a reference, leaving another value you supplied.
///
/// This is a generalization of [`mem::replace`] that
/// additionally supports [`Cell`] and [`RefCell`].
///
/// ## Usage
///
/// ```rust
/// # use omniswap::Replace;
/// let mut x = 42;
/// assert_eq!(Replace::replace(&mut x, 84), 42);
/// assert_eq!(x, 84);
/// ```
pub trait Replace<T> {
    fn replace(self, value: T) -> T;
}

impl<'a, T> Replace<T> for &'a mut T {
    fn replace(self, value: T) -> T {
        mem::replace(self, value)
    }
}

impl<'a, T> Replace<T> for &'a Cell<T> {
    fn replace(self, value: T) -> T {
        self.replace(value)
    }
}

impl<'a, T> Replace<T> for &'a RefCell<T> {
    fn replace(self, value: T) -> T {
        let mut r = self.borrow_mut();
        mem::replace(&mut *r, value)
    }
}
