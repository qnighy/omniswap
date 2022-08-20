use std::cell::{Cell, RefCell};

use omniswap::{rotate, swap};

#[test]
fn test_swap() {
    {
        let mut x = 42;
        let mut y = 84;
        swap!(&mut x, &mut y);
        assert_eq!((x, y), (84, 42));
    }
    {
        let mut x = 42;
        let mut y = 84;
        swap!(&mut x, &mut y,);
        assert_eq!((x, y), (84, 42));
    }
}

#[test]
fn test_swap_array() {
    {
        let mut a = [3, 5, 2, 6, 7];
        swap!(&mut a[0], &mut a[2]);
        assert_eq!(a, [2, 5, 3, 6, 7]);
    }
    {
        let mut a = [3, 5, 2, 6, 7];
        swap!(&mut a[0], &mut a[0]);
        assert_eq!(a, [3, 5, 2, 6, 7]);
    }
}

#[test]
fn test_swap_2d_array() {
    {
        let mut a = [[1, 2], [3, 4]];
        swap!(&mut a[0][0], &mut a[0][0]);
        assert_eq!(a, [[1, 2], [3, 4]]);
    }
    {
        let mut a = [[1, 2], [3, 4]];
        swap!(&mut a[0][0], &mut a[0][1]);
        assert_eq!(a, [[2, 1], [3, 4]]);
    }
    {
        let mut a = [[1, 2], [3, 4]];
        swap!(&mut a[0][0], &mut a[1][1]);
        assert_eq!(a, [[4, 2], [3, 1]]);
    }
}

#[test]
fn test_swap_noncopy() {
    let mut x = vec![1, 2, 3];
    let mut y = vec![4, 5];
    swap!(&mut x, &mut y);
    assert_eq!((x, y), (vec![4, 5], vec![1, 2, 3]));
}

#[test]
fn test_swap_cell() {
    {
        let mut x = 42;
        let y = Cell::new(84);
        swap!(&mut x, &y);
        let y = y.into_inner();
        assert_eq!((x, y), (84, 42));
    }
    {
        let x = Cell::new(42);
        let mut y = 84;
        swap!(&x, &mut y);
        let x = x.into_inner();
        assert_eq!((x, y), (84, 42));
    }
    {
        let x = Cell::new(42);
        let y = Cell::new(84);
        swap!(&x, &y);
        let x = x.into_inner();
        let y = y.into_inner();
        assert_eq!((x, y), (84, 42));
    }
}

#[test]
fn test_swap_ref_cell() {
    {
        let mut x = 42;
        let y = RefCell::new(84);
        swap!(&mut x, &y);
        let y = y.into_inner();
        assert_eq!((x, y), (84, 42));
    }
    {
        let x = RefCell::new(42);
        let mut y = 84;
        swap!(&x, &mut y);
        let x = x.into_inner();
        assert_eq!((x, y), (84, 42));
    }
    {
        let x = RefCell::new(42);
        let y = RefCell::new(84);
        swap!(&x, &y);
        let x = x.into_inner();
        let y = y.into_inner();
        assert_eq!((x, y), (84, 42));
    }
}

#[test]
fn test_swap_eval_order() {
    let mut log = vec![];
    let mut x = 42;
    let mut y = 84;
    swap!(
        {
            log.push(100);
            &mut x
        },
        {
            log.push(200);
            &mut y
        }
    );
    assert_eq!((x, y), (84, 42));
    assert_eq!(log, vec![100, 200, 100]);
}

#[test]
fn test_swap_cycle() {
    {
        let mut x = 42;
        let mut y = 84;
        rotate!(&mut x, &mut y);
        assert_eq!((x, y), (84, 42));
    }
    {
        let mut x = 42;
        let mut y = 84;
        swap!(&mut x, &mut y,);
        assert_eq!((x, y), (84, 42));
    }

    {
        let mut x = 42;
        rotate!(&mut x);
        assert_eq!(x, 42);
    }
    {
        let mut x = 42;
        rotate!(&mut x,);
        assert_eq!(x, 42);
    }

    {
        let mut x = 1;
        let mut y = 2;
        let mut z = 3;
        rotate!(&mut x, &mut y, &mut z);
        assert_eq!((x, y, z), (3, 1, 2));
    }
    {
        let mut x = 1;
        let mut y = 2;
        let mut z = 3;
        rotate!(&mut x, &mut y, &mut z,);
        assert_eq!((x, y, z), (3, 1, 2));
    }
}

#[test]
fn test_swap_cycle_eval_order() {
    {
        let mut log = vec![];
        let mut x = 42;
        let mut y = 84;
        rotate!(
            {
                log.push(100);
                &mut x
            },
            {
                log.push(200);
                &mut y
            }
        );
        assert_eq!((x, y), (84, 42));
        assert_eq!(log, vec![100, 200, 100]);
    }

    {
        let mut log = vec![];
        let mut x = 42;
        rotate!({
            log.push(100);
            &mut x
        });
        assert_eq!(x, 42);
        assert_eq!(log, vec![100, 100]);
    }

    {
        let mut log = vec![];
        let mut x = 1;
        let mut y = 2;
        let mut z = 3;
        rotate!(
            {
                log.push(100);
                &mut x
            },
            {
                log.push(200);
                &mut y
            },
            {
                log.push(300);
                &mut z
            }
        );
        assert_eq!((x, y, z), (3, 1, 2));
        assert_eq!(log, vec![100, 200, 300, 100]);
    }
}
