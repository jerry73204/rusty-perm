use crate::common::*;

/// The permutation composition operator.
pub trait PermProduct<Rhs> {
    type Output;

    fn perm_product(&self, other: &Rhs) -> Self::Output;
}

mod without_std {
    use super::*;
    use crate::perm_type::PermS;

    impl<const SIZE: usize> PermProduct<PermS<SIZE>> for PermS<SIZE> {
        type Output = PermS<SIZE>;

        fn perm_product(&self, other: &PermS<SIZE>) -> Self::Output {
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Self { indices }
        }
    }

    impl<const SIZE: usize> Mul<&PermS<SIZE>> for &PermS<SIZE> {
        type Output = PermS<SIZE>;

        fn mul(self, other: &PermS<SIZE>) -> Self::Output {
            self.perm_product(other)
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::{
        perm_trait::Permutation,
        perm_type::{PermD, PermS},
    };

    impl<const SIZE: usize> PermProduct<PermD> for PermS<SIZE> {
        type Output = Option<PermS<SIZE>>;

        fn perm_product(&self, other: &PermD) -> Self::Output {
            if other.len() != SIZE {
                return None;
            }
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> PermProduct<PermS<SIZE>> for PermD {
        type Output = Option<PermS<SIZE>>;

        fn perm_product(&self, other: &PermS<SIZE>) -> Self::Output {
            if self.len() != SIZE {
                return None;
            }
            let mut indices = [0; SIZE];
            product(&self.indices, &other.indices, &mut indices);
            Some(PermS { indices })
        }
    }

    impl PermProduct<PermD> for PermD {
        type Output = Option<PermD>;

        fn perm_product(&self, other: &PermD) -> Self::Output {
            if self.len() != other.len() {
                return None;
            }
            let mut indices = vec![0; self.len()];
            product(&self.indices, &other.indices, &mut indices);
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> Mul<&PermD> for &PermS<SIZE> {
        type Output = PermS<SIZE>;

        fn mul(self, other: &PermD) -> Self::Output {
            self.perm_product(other).unwrap()
        }
    }

    impl<const SIZE: usize> Mul<&PermS<SIZE>> for &PermD {
        type Output = PermS<SIZE>;

        fn mul(self, other: &PermS<SIZE>) -> Self::Output {
            self.perm_product(other).unwrap()
        }
    }

    impl Mul<&PermD> for &PermD {
        type Output = PermD;

        fn mul(self, other: &PermD) -> Self::Output {
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
