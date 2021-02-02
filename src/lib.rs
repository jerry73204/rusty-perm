#![cfg_attr(not(feature = "std"), no_std)]

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
