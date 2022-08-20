use std::cell::{Cell, RefCell};

use omniswap::take;

#[test]
fn test_take_copy() {
    {
        // Clone + Copy + Default
        let mut place = CloneCounter::new(42);
        let taken = take!(&mut place);
        assert_eq!(taken.value, 42);
        assert_eq!(taken.cloned, 0);
        assert_eq!(place.value, 42);
    }

    {
        // Clone + Copy
        let mut place = CloneCounter::new(NoDefault::new(42));
        let taken = take!(&mut place);
        assert_eq!(taken.value.value, 42);
        assert_eq!(taken.cloned, 0);
        assert_eq!(place.value.value, 42);
    }
}

#[test]
fn test_take_cell_copy() {
    {
        // Clone + Copy + Default
        let place = Cell::new(CloneCounter::new(42));
        let taken = take!(&place);
        assert_eq!(taken.value, 42);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value, 42);
    }

    {
        // Clone + Copy
        let place = Cell::new(CloneCounter::new(NoDefault::new(42)));
        let taken = take!(&place);
        assert_eq!(taken.value.value, 42);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value.value, 42);
    }
}

#[test]
fn test_take_ref_cell_copy() {
    {
        // Clone + Copy + Default
        let place = RefCell::new(CloneCounter::new(42));
        let taken = take!(&place);
        assert_eq!(taken.value, 42);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value, 42);
    }

    {
        // Clone + Copy
        let place = RefCell::new(CloneCounter::new(NoDefault::new(42)));
        let taken = take!(&place);
        assert_eq!(taken.value.value, 42);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value.value, 42);
    }
}

#[test]
fn test_take_default() {
    {
        // Clone + Default
        let mut place = CloneCounter::new(vec![42]);
        let taken = take!(&mut place);
        assert_eq!(taken.value, vec![42]);
        assert_eq!(taken.cloned, 0);
        assert_eq!(place.value, vec![]);
    }
    {
        // Default only
        let mut place = NoClone::new(42);
        let taken = take!(&mut place);
        assert_eq!(taken.value, 42);
        assert_eq!(place.value, 0);
    }
}

#[test]
fn test_take_cell_default() {
    {
        // Clone + Default
        let place = Cell::new(CloneCounter::new(vec![42]));
        let taken = take!(&place);
        assert_eq!(taken.value, vec![42]);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value, vec![]);
    }
    {
        // Default only
        let place = Cell::new(NoClone::new(42));
        let taken = take!(&place);
        assert_eq!(taken.value, 42);
        let place = place.into_inner();
        assert_eq!(place.value, 0);
    }
}

#[test]
fn test_take_ref_cell_default() {
    {
        // Clone + Default
        let place = RefCell::new(CloneCounter::new(vec![42]));
        let taken = take!(&place);
        assert_eq!(taken.value, vec![42]);
        assert_eq!(taken.cloned, 0);
        let place = place.into_inner();
        assert_eq!(place.value, vec![]);
    }
    {
        // Default only
        let place = RefCell::new(NoClone::new(42));
        let taken = take!(&place);
        assert_eq!(taken.value, 42);
        let place = place.into_inner();
        assert_eq!(place.value, 0);
    }
}

#[test]
fn test_take_clone() {
    // Clone only
    let mut place = CloneCounter::new(NoDefault::new(vec![42]));
    let taken = take!(&mut place);
    assert_eq!(taken.value.value, vec![42]);
    assert_eq!(taken.cloned, 1);
    assert_eq!(place.value.value, vec![42]);
}

#[test]
fn test_take_ref_cell_clone() {
    // Clone only
    let place = RefCell::new(CloneCounter::new(NoDefault::new(vec![42])));
    let taken = take!(&place);
    assert_eq!(taken.value.value, vec![42]);
    assert_eq!(taken.cloned, 1);
    let place = place.into_inner();
    assert_eq!(place.value.value, vec![42]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoDefault<T> {
    pub value: T,
}

impl<T> NoDefault<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct NoClone<T> {
    pub value: T,
}

impl<T> NoClone<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Debug, Copy, Eq, Default)]
pub struct CloneCounter<T> {
    pub value: T,
    pub cloned: u32,
}

impl<T> CloneCounter<T> {
    pub fn new(value: T) -> Self {
        Self { value, cloned: 0 }
    }
}

impl<T: PartialEq> PartialEq for CloneCounter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone> Clone for CloneCounter<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            cloned: self.cloned + 1,
        }
    }
}
