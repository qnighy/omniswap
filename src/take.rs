use std::cell::{Cell, RefCell};
use std::mem;
use std::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! take {
    ($p: expr) => {
        $crate::TakeHelper::new($p).take()
    };
    ($p: expr,) => {
        $crate::take!($p)
    };
}

pub struct TakeHelper<T>(TakeHelper2<T>);

pub struct TakeHelper2<T>(TakeHelper3<T>);

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
