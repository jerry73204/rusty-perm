#[cfg(feature = "std")]
pub use with_std::*;
pub use without_std::*;

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

    #[derive(Debug, Clone, Copy)]
    pub struct Static<const SIZE: usize>;
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Dynamic;

    impl PermSize for Dynamic {
        type Container = Vec<usize>;
    }
}
