use crate::apply::PermApply;

pub trait MapPerm {
    fn map_perm<P>(&mut self, perm: &P) -> P::Output
    where
        P: PermApply<Self>;
}

impl<T> MapPerm for T {
    fn map_perm<P>(&mut self, perm: &P) -> P::Output
    where
        P: PermApply<Self>,
    {
        perm.apply(self)
    }
}
