pub trait Permutation {
    fn len(&self) -> usize;
    fn inverse(&self) -> Self;
    fn indices(&self) -> &[usize];
}

mod without_std {
    use super::*;
    use crate::perm_type::StaticPerm;

    impl<const SIZE: usize> Permutation for StaticPerm<SIZE> {
        fn indices(&self) -> &[usize] {
            self.indices.as_ref()
        }

        fn len(&self) -> usize {
            SIZE
        }

        fn inverse(&self) -> Self {
            let mut inversed = [0; SIZE];
            inverse_indices(self.indices.as_ref(), &mut inversed);
            Self { indices: inversed }
        }
    }
}

#[cfg(not(feature = "no_std"))]
mod with_std {
    use super::*;
    use crate::perm_type::DynamicPerm;

    impl Permutation for DynamicPerm {
        fn indices(&self) -> &[usize] {
            self.indices.as_ref()
        }

        fn len(&self) -> usize {
            self.indices.len()
        }

        fn inverse(&self) -> Self {
            let mut inversed = vec![0; self.indices.len()];
            inverse_indices(self.indices.as_slice(), &mut inversed);
            Self { indices: inversed }
        }
    }
}

fn inverse_indices(indices: &[usize], inverse_indices: &mut [usize]) {
    indices.iter().enumerate().for_each(|(dst, &src)| {
        inverse_indices[src] = dst;
    });
}
