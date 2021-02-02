/// An abstract representation of permutation data structure.
pub trait Permutation {
    /// Gets the size of permutation.
    fn len(&self) -> usize;

    /// Builds the inverse of permutation.
    fn inverse(&self) -> Self;

    /// Gets the reference to the internal permuted indices.
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apply::PermApply;
        use rand::prelude::*;

        #[test]
        fn static_inverse() {
            const SIZE: usize = 1024;
            let mut rng = rand::thread_rng();

            let mut perm = StaticPerm::<SIZE>::identity();
            perm.indices.shuffle(&mut rng);
            let inverse = perm.inverse();

            let mut orig = [0usize; SIZE];
            rng.fill(&mut orig);

            let mut new = orig.clone();
            perm.apply(&mut new);
            inverse.apply(&mut new);

            assert_eq!(orig, new);
        }
    }
}

#[cfg(feature = "std")]
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apply::PermApply;
        use rand::prelude::*;

        #[test]
        fn dynamic_inverse() {
            const SIZE: usize = 1024;
            let mut rng = rand::thread_rng();

            let mut perm = DynamicPerm::identity(SIZE);
            perm.indices.shuffle(&mut rng);
            let inverse = perm.inverse();

            let mut orig = [0usize; SIZE];
            rng.fill(&mut orig);

            let mut new = orig.clone();
            perm.apply(&mut new).unwrap();
            inverse.apply(&mut new).unwrap();

            assert_eq!(orig, new);
        }
    }
}

fn inverse_indices(indices: &[usize], inverse_indices: &mut [usize]) {
    indices.iter().enumerate().for_each(|(dst, &src)| {
        inverse_indices[src] = dst;
    });
}
