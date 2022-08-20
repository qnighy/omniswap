#[macro_export]
macro_rules! swap {
    ($x: expr, $y: expr) => {
        $crate::swap_cycle!($x, $y)
    };
    ($x: expr, $y: expr,) => {
        $crate::swap_cycle!($x, $y)
    };
}

#[macro_export]
macro_rules! swap_cycle {
    ($x: expr, $($y: expr),*) => {
        {
            let value = $crate::take!($x);
            $(
                let value = $crate::Replace::replace($y, value);
            )*
            let _ = $crate::Replace::replace($x, value);
        }
    };
}
