use std::cell::{Cell, RefCell};

use omniswap::Replace;

#[test]
fn test_replace_mut() {
    let mut place = 42;
    let taken = Replace::replace(&mut place, 84);
    assert_eq!(taken, 42);
    assert_eq!(place, 84);
}

#[test]
fn test_replace_cell() {
    let place = Cell::new(42);
    let taken = Replace::replace(&place, 84);
    assert_eq!(taken, 42);
    let place = place.into_inner();
    assert_eq!(place, 84);
}

#[test]
fn test_replace_ref_cell() {
    let place = RefCell::new(42);
    let taken = Replace::replace(&place, 84);
    assert_eq!(taken, 42);
    let place = place.into_inner();
    assert_eq!(place, 84);
}
