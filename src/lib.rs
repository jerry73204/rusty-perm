#![cfg_attr(feature = "no_std", no_std)]

pub mod prelude {
    pub use super::apply::PermApply;
    pub use super::from_indices::PermFromIndices;
    pub use super::from_sorting::PermFromSorting;
    pub use super::map_perm::MapPerm;
    pub use super::perm_trait::Permutation;
    pub use super::product::PermProduct;
}

mod apply;
mod common;
mod from_indices;
mod from_sorting;
mod map_perm;
mod perm_trait;
mod perm_type;
mod product;
mod rand;
pub mod size;

pub use apply::*;
pub use from_indices::*;
pub use from_sorting::*;
pub use map_perm::*;
pub use perm_trait::*;
pub use perm_type::*;
pub use product::*;
