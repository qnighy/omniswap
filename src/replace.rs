use std::cell::{Cell, RefCell};
use std::mem;

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
