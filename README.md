# iterable_assertions

Assertions for comparing iterable collections in tests. The crate provides
macros for comparing collections as sets, maps, or ordered sequences, and for
checking that items are unique.

## Installation

Add `iterable_assertions` to your `Cargo.toml`:

```toml
[dev-dependencies]
iterable_assertions = "0.1"
```

## Usage

Import the macros you need:

```rust
use iterable_assertions::{
    assert_eq_maps, assert_eq_seqs, assert_eq_sets, assert_unique,
};
```

### Compare collections as sets

`assert_eq_sets!` checks that two iterables contain the same elements,
regardless of order. Repeated elements are ignored, as they are in a set.

```rust
assert_eq_sets!(vec![1, 2, 3], vec![3, 2, 1]);
assert_eq_sets!([1, 2, 2], [2, 1]);
```

### Compare maps

`assert_eq_maps!` compares iterables of key-value pairs. It checks that each
key has the same value in both inputs and panics if either input contains a
duplicate key.

```rust
assert_eq_maps!(
    [("name", "Ada"), ("language", "Rust")],
    [("language", "Rust"), ("name", "Ada")],
);
```

### Compare ordered sequences

`assert_eq_seqs!` checks that two iterables have the same elements in the same
order.

```rust
assert_eq_seqs!([1, 2, 3], [1, 2, 3]);
```

### Check uniqueness

`assert_unique!` panics if an item occurs more than once in the iterable.

```rust
assert_unique!(vec!["red", "green", "blue"]);
```

## Failure messages

Each assertion macro panics on failure and reports the relevant differences.
An optional format string and arguments can be supplied to add context to the
failure message:

```rust
let actual = vec![1, 2, 4];
assert_eq_sets!(actual, [1, 2, 3], "checking the result of {}", "my test");

let values = [1, 2, 2];
assert_unique!(values, "input from {}", "the fixture");
```

The format string must be a string literal, followed by any arguments it
requires.

## Requirements

Items compared by these macros must implement `Eq`, `Hash`, and `Clone`.
For map comparisons, values must also implement `Eq` and `Clone`.

## License

Licensed under the [MIT License](LICENSE).
