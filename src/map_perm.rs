use crate::{perm_trait::Permutation, perm_type::Perm, size::PermSize};

pub trait MapPerm
where
    Self: Sized,
{
    type Error;

    fn map_perm<S>(self, perm: Perm<S>) -> Result<Self, Self::Error>
    where
        Perm<S>: Permutation,
        S: PermSize;
}

mod without_std {
    use super::*;

    impl<T> MapPerm for &mut [T] {
        type Error = &'static str;

        fn map_perm<S>(self, perm: Perm<S>) -> Result<Self, Self::Error>
        where
            Perm<S>: Permutation,
            S: PermSize,
        {
            perm.apply(self)
        }
    }
}

#[cfg(not(feature = "no_std"))]
mod with_std {
    use super::*;

    impl<T> MapPerm for &mut Vec<T> {
        type Error = &'static str;

        fn map_perm<S>(self, perm: Perm<S>) -> Result<Self, Self::Error>
        where
            Perm<S>: Permutation,
            S: PermSize,
        {
            perm.apply(self.as_mut_slice())?;
            Ok(self)
        }
    }
}
