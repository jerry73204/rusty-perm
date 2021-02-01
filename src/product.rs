use crate::common::*;

pub trait PermProduct<Rhs> {
    type Output;

    fn perm_product(&self, other: &Rhs) -> Self::Output;
}

mod without_std {
    use super::*;
    use crate::perm_type::StaticPerm;

    impl<const SIZE: usize> PermProduct<StaticPerm<SIZE>> for StaticPerm<SIZE> {
        type Output = StaticPerm<SIZE>;

        fn perm_product(&self, other: &StaticPerm<SIZE>) -> Self::Output {
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Self { indices }
        }
    }

    impl<const SIZE: usize> Mul<&StaticPerm<SIZE>> for &StaticPerm<SIZE> {
        type Output = StaticPerm<SIZE>;

        fn mul(self, other: &StaticPerm<SIZE>) -> Self::Output {
            self.perm_product(other)
        }
    }
}

#[cfg(not(feature = "no_std"))]
mod with_std {
    use super::*;
    use crate::perm_trait::Permutation;
    use crate::perm_type::{DynamicPerm, StaticPerm};

    impl<const SIZE: usize> PermProduct<DynamicPerm> for StaticPerm<SIZE> {
        type Output = Option<StaticPerm<SIZE>>;

        fn perm_product(&self, other: &DynamicPerm) -> Self::Output {
            if other.len() != SIZE {
                return None;
            }
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> PermProduct<StaticPerm<SIZE>> for DynamicPerm {
        type Output = Option<StaticPerm<SIZE>>;

        fn perm_product(&self, other: &StaticPerm<SIZE>) -> Self::Output {
            if self.len() != SIZE {
                return None;
            }
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Some(StaticPerm { indices })
        }
    }

    impl PermProduct<DynamicPerm> for DynamicPerm {
        type Output = Option<DynamicPerm>;

        fn perm_product(&self, other: &DynamicPerm) -> Self::Output {
            if self.len() != other.len() {
                return None;
            }
            let mut indices = vec![0; self.len()];
            product(&self.indices, &other.indices, &mut indices);
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> Mul<&DynamicPerm> for &StaticPerm<SIZE> {
        type Output = StaticPerm<SIZE>;

        fn mul(self, other: &DynamicPerm) -> Self::Output {
            self.perm_product(other).unwrap()
        }
    }

    impl<const SIZE: usize> Mul<&StaticPerm<SIZE>> for &DynamicPerm {
        type Output = StaticPerm<SIZE>;

        fn mul(self, other: &StaticPerm<SIZE>) -> Self::Output {
            self.perm_product(other).unwrap()
        }
    }

    impl Mul<&DynamicPerm> for &DynamicPerm {
        type Output = DynamicPerm;

        fn mul(self, other: &DynamicPerm) -> Self::Output {
            self.perm_product(other).unwrap()
        }
    }
}

fn product(lhs: &[usize], rhs: &[usize], output: &mut [usize]) {
    let len = output.len();
    (0..len).for_each(|src| {
        let dst = lhs[rhs[src]];
        output[src] = dst;
    });
}
