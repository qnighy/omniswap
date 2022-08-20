# omniswap: a crate to swap values between possibly-overlapping references

## Motivating Example

You cannot simply use `std::mem::swap` to replace values within an array:

```rust
let mut a = [1, 2, 3];
// You cannot prove their disjointness!
std::mem::swap(&mut a[0], &mut a[2]);
```

You can use the dedicated `<[T]>::swap` instead:

```rust
let mut a = [1, 2, 3];
a.swap(0, 2);
```

But how about two-dimensional arrays?

```rust
let mut a = [[1, 2], [3, 4]];
// You cannot prove their disjointness!
std::mem::swap(&mut a[0][0], &mut a[1][1]);
```

This is not as simple as the first one.

## Solution

This crate solves the problem by providing a generic framework for
**sentinel-based swapping**.

The idea is simple: it leaves a dummy value behind to safely
move values around:

```rust
let mut a = [[1, 2], [3, 4]];
let tmp = std::mem::replace(&mut a[0][0], 0);
let tmp = std::mem::replace(&mut a[1][1], tmp);
a[0][0] = tmp;
# assert_eq!(a, [[4, 2], [3, 1]]);
```

However, in Rust, the best sentinel value differs between types.

The macro `swap!` automatically chooses the best sentinel and
provides the same interface as `std::mem::swap`:

```rust
let mut a = [[1, 2], [3, 4]];
omniswap::swap!(&mut a[0][0], &mut a[1][1]);
# assert_eq!(a, [[4, 2], [3, 1]]);
```

## Usage

Simply use `swap!` where you want to use `std::mem::swap`:

```rust
let mut x = 42;
let mut y = 84;
omniswap::swap!(&mut x, &mut y);
```

See `swap!` for detailed usages.

## Other APIs

The crate provides the following variants:

- `rotate!` -- swaps more than two values at once


The crate also exposes `take!` and `Replace`.
These are primitives used in `swap!` and `rotate!`.
