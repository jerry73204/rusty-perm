#[cfg(not(feature = "no_std"))]
pub use dynamic::*;
pub use static_::*;

pub trait PermSize
where
    Self::Container: AsRef<[usize]>,
{
    type Container;
}

mod static_ {
    use super::*;

    impl<const SIZE: usize> PermSize for Static<SIZE> {
        type Container = [usize; SIZE];
    }

    pub struct Static<const SIZE: usize>;
}

#[cfg(not(feature = "no_std"))]
mod dynamic {
    use super::*;

    pub struct Dynamic;

    impl PermSize for Dynamic {
        type Container = Vec<usize>;
    }
}
