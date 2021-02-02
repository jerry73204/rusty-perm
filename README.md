# rusty-perm

Rusty permutation that supports `no-std` and compile-time checked size.

## Cargo Features

To import this crate to your project,

```toml
[dependencies]
rusty-perm = "0.1"
```

It has the following cargo features.
- **std** (default): enable the standard library.
- **rand** (default): enable random sampling of permutation.

To restrict the crate to `no_std`, you can disable the default features.

```toml
[dependencies]
rusty-perm = { version = "0.1", default-features = false }
```

## Usage

### Import this crate

To import members from this crate,
```rust
use rusty_perm::{prelude::*, DynamicPerm, StaticPerm};
```

Both `DynamicPerm` and `StaticPerm` represent permutations, except that
`StaticPerm` has an embedded compile-time size in type signature. The static size
prevents from applying permutation on arrays of wrong sizes in compile-time, and saves
some runtime overheads.

### Identity

The identity permutation can be constructed with static or dynamic size.

```rust
use rusty_perm::{DynamicPerm, StaticPerm};
let perm1 = StaticPerm::<10>::identity();
let perm2 = DynamicPerm::identity(10);
```

### Build by sorting slices and arrays

It can extracts the permutation by sorting an array.

```rust
use rusty_perm::{prelude::*, StaticPerm};

// `perm` is an operator that maps [9, 6, -1, 4] to [-1, 4, 6, 9].
let perm = StaticPerm::from_sort(&[9, 6, -1, 4]);

// Apply same permutation on another array
let mut array = [1, 2, 3, 4];
perm.apply(&mut array);
assert_eq!(array, [3, 4, 2, 1]);
```

You can sort with custom comparing or key function by
[from_sort_by](crate::PermFromSorting::from_sort_by),
[from_sort_by_key](crate::PermFromSorting::from_sort_by_key) and
[from_sort_by_cached_key](crate::PermFromSorting::from_sort_by_cached_key).

```rust
use rusty_perm::{prelude::*, StaticPerm};

// `perm` is an operator that maps [9, 6, -1, 4] to [9, 6, 4, -1].
let perm = StaticPerm::from_sort_by_key(&[9, 6, -1, 4], |val| -val);

// Apply same permutation on another array
let mut array = [1, 2, 3, 4];
perm.apply(&mut array);
assert_eq!(array, [1, 2, 4, 3]);
```

### Build by indices

The permutation can be constructed by demonstrating the sorted indices.

```rust
use rusty_perm::{prelude::*, DynamicPerm};
let perm = DynamicPerm::from_indices([2, 0, 1]).unwrap();

let mut array = [-9, -5, 3];
perm.apply(&mut array);
assert_eq!(array, [3, -9, -5]);
```

### Inverse and composition

The example demonstrates the inverse and composition of permutations.

```rust
use rusty_perm::{prelude::*, DynamicPerm, StaticPerm};

// Construct the permutation, its inverse and compose them
let perm = StaticPerm::from_indices([2, 0, 1]).unwrap();
let inverse = perm.inverse();
let composition = &inverse * &perm;

// Check that composition with its inverse is identity
assert_eq!(DynamicPerm::identity(3), composition);
```

## License

Apache 2.0 and MIT dual license.
