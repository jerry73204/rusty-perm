use crate::size::PermSize;

#[cfg(feature = "std")]
pub use with_std::*;
pub use without_std::*;

/// Generic permutation data structure.
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

    /// Permutation type with static size known in compile time.
    pub type StaticPerm<const SIZE: usize> = Perm<Static<{ SIZE }>>;

    impl<const SIZE: usize> StaticPerm<SIZE> {
        pub fn identity() -> Self {
            let mut indices = [0; SIZE];
            (0..SIZE).for_each(|index| indices[index] = index);
            Self { indices }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apply::PermApply;
        use rand::prelude::*;

        #[test]
        fn static_identity() {
            const SIZE: usize = 2014;

            let perm = StaticPerm::<SIZE>::identity();

            let mut rng = rand::thread_rng();
            let mut orig = [0usize; SIZE];
            rng.fill(orig.as_mut());

            let mut new = orig.clone();
            perm.apply(&mut new);

            assert_eq!(orig, new);
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::{common::*, size::Dynamic};

    /// Permutation type with runtime size.
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
            Some(PermS { indices })
        }
    }

    impl<const SIZE: usize> PermS<SIZE> {
        pub fn into_dynamic(self) -> PermD {
            let Self { indices } = self;
            PermD {
                indices: Vec::from(indices),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apply::PermApply;
        use rand::prelude::*;

        #[test]
        fn static_identity() {
            const SIZE: usize = 2014;

            let perm = PermD::identity(SIZE);

            let mut rng = rand::thread_rng();
            let mut orig = [0usize; SIZE];
            rng.fill(orig.as_mut());

            let mut new = orig.clone();
            perm.apply(&mut new).unwrap();

            assert_eq!(orig, new);
        }
    }
}
