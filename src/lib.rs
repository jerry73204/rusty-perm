//! Rusty permutation that supports `no-std` and compile-time checked size.
//!
//! ## Cargo features
//!
//! To import this crate to your project,
//!
//! ```toml
//! [dependencies]
//! rusty-perm = "0.2"
//! ```
//!
//! It has the following cargo features.
//! - **std** (default): enable the standard library.
//! - **rand** (default): enable random sampling of permutation.
//!
//! To restrict the crate to `no_std`, you can disable the default features.
//!
//! ```toml
//! [dependencies]
//! rusty-perm = { version = "0.2", default-features = false }
//! ```
//!
//! ## Import this crate
//!
//! To import members from this crate,
//! ```rust
//! use rusty_perm::{prelude::*, PermD, PermS};
//! ```
//!
//! Both `PermD` and `PermS` represent permutations, except that
//! `PermS` has an embedded compile-time size in type signature. The static size
//! prevents from applying permutation on arrays of wrong sizes in compile-time, and saves
//! some runtime overheads.
//!
//! ## Identity
//! The identity permutation can be constructed with static or dynamic size.
//!
//! ```rust
//! use rusty_perm::{PermD, PermS};
//! let perm1 = PermS::<10>::identity();
//! let perm2 = PermD::identity(10);
//! ```
//!
//! ## Build by sorting slices and arrays
//!
//! It can extracts the permutation by sorting an array.
//!
//! ```rust
//! use rusty_perm::{prelude::*, PermS};
//!
//! // `perm` is an operator that maps [9, 6, -1, 4] to [-1, 4, 6, 9].
//! let perm = PermS::from_sort(&[9, 6, -1, 4]);
//!
//! // Apply same permutation on another array
//! let mut array = [1, 2, 3, 4];
//! perm.apply(&mut array);
//! assert_eq!(array, [3, 4, 2, 1]);
//! ```
//!
//! You can sort with custom comparing or key function by
//! [from_sort_by](crate::PermFromSorting::from_sort_by),
//! [from_sort_by_key](crate::PermFromSorting::from_sort_by_key) and
//! [from_sort_by_cached_key](crate::PermFromSorting::from_sort_by_cached_key).
//!
//! ```rust
//! use rusty_perm::{prelude::*, PermS};
//!
//! // `perm` is an operator that maps [9, 6, -1, 4] to [9, 6, 4, -1].
//! let perm = PermS::from_sort_by_key(&[9, 6, -1, 4], |val| -val);
//!
//! // Apply same permutation on another array
//! let mut array = [1, 2, 3, 4];
//! perm.apply(&mut array);
//! assert_eq!(array, [1, 2, 4, 3]);
//! ```
//!
//! ## Build by indices
//! The permutation can be constructed by demonstrating the sorted indices.
//!
//! ```rust
//! use rusty_perm::{prelude::*, PermD};
//! let perm = PermD::from_indices([2, 0, 1]).unwrap();
//!
//! let mut array = [-9, -5, 3];
//! perm.apply(&mut array);
//! assert_eq!(array, [3, -9, -5]);
//! ```
//!
//! ## Inverse and composition
//! The example demonstrates the inverse and composition of permutations.
//!
//! ```rust
//! use rusty_perm::{prelude::*, PermD, PermS};
//!
//! // Construct the permutation, its inverse and compose them
//! let perm = PermS::from_indices([2, 0, 1]).unwrap();
//! let inverse = perm.inverse();
//! let composition = &inverse * &perm;
//!
//! // Check that composition with its inverse is identity
//! assert_eq!(PermD::identity(3), composition);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

/// Re-export of common traits.
pub mod prelude {
    pub use super::{
        apply::PermApply, from_indices::PermFromIndices, from_sorting::PermFromSorting,
        perm_trait::Permutation, product::PermProduct,
    };
}

mod apply;
mod common;
mod from_indices;
mod from_sorting;
mod perm_trait;
mod perm_type;
mod product;
mod rand;
pub mod size;

pub use apply::*;
pub use from_indices::*;
pub use from_sorting::*;
pub use perm_trait::*;
pub use perm_type::*;
pub use product::*;
