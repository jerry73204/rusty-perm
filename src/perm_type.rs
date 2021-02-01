use crate::size::PermSize;

#[cfg(not(feature = "no_std"))]
pub use with_std::*;
pub use without_std::*;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Perm<S>
where
    S: PermSize,
{
    pub(super) indices: S::Container,
}

impl<SL, SR> PartialEq<Perm<SR>> for Perm<SL>
where
    SL: PermSize,
    SR: PermSize,
{
    fn eq(&self, other: &Perm<SR>) -> bool {
        self.indices.as_ref() == other.indices.as_ref()
    }
}

mod without_std {
    use super::*;
    use crate::size::Static;

    pub type StaticPerm<const SIZE: usize> = Perm<Static<{ SIZE }>>;

    impl<const SIZE: usize> StaticPerm<SIZE> {
        pub fn identity() -> Self {
            let mut indices = [0; SIZE];
            (0..SIZE).for_each(|index| indices[index] = index);
            Self { indices }
        }

        #[cfg(not(feature = "no_std"))]
        pub fn into_dynamic(self) -> DynamicPerm {
            let Self { indices } = self;
            DynamicPerm {
                indices: Vec::from(indices),
            }
        }
    }
}

#[cfg(not(feature = "no_std"))]
mod with_std {
    use super::*;
    use crate::common::*;
    use crate::size::Dynamic;

    pub type DynamicPerm = Perm<Dynamic>;

    impl DynamicPerm {
        pub fn identity(len: usize) -> Self {
            let mut indices = vec![0; len];
            (0..len).for_each(|index| indices[index] = index);
            Self { indices }
        }

        pub fn into_static<const SIZE: usize>(self) -> Option<StaticPerm<SIZE>> {
            let Self { indices } = self;
            let indices = <[usize; SIZE]>::try_from(indices).ok()?;
            Some(StaticPerm { indices })
        }
    }
}
