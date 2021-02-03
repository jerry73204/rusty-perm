//! Permutation size markers.

#[cfg(feature = "std")]
pub use with_std::*;
pub use without_std::*;

/// The permutation size marker trait.
pub trait PermSize
where
    Self::Container: AsRef<[usize]>,
{
    type Container;
}

mod without_std {
    use super::*;

    impl<const SIZE: usize> PermSize for Static<SIZE> {
        type Container = [usize; SIZE];
    }

    /// The static size marker type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Static<const SIZE: usize>;
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;

    /// The dynamic size marker type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Dynamic;

    impl PermSize for Dynamic {
        type Container = Vec<usize>;
    }
}
