/// An abstract representation of permutation data structure.
pub trait Permutation {
    /// Gets the size of permutation.
    fn len(&self) -> usize;

    /// Builds the inverse of permutation.
    fn inverse(&self) -> Self;

    /// Gets the reference to the internal permuted indices.
    fn indices(&self) -> &[usize];

    fn pow(&self, exp: u32) -> Self;
}

mod without_std {
    use super::*;
    use crate::perm_type::PermS;

    impl<const SIZE: usize> Permutation for PermS<SIZE> {
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

        fn pow(&self, exp: u32) -> Self {
            let mut mask = u32_leading_bit(exp);
            let mut pow = Self::identity();

            // power by squaring
            loop {
                if exp & mask != 0 {
                    pow = &pow * self;
                }

                mask >>= 1;

                if mask == 0 {
                    break;
                } else {
                    pow = &pow * &pow;
                }
            }

            pow
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
            {
                let mut rng = rand::thread_rng();

                let mut perm = PermS::<SIZE>::identity();
                perm.indices.shuffle(&mut rng);
                let inverse = perm.inverse();

                let mut orig = [0usize; SIZE];
                rng.fill(&mut orig);

                let mut new = orig.clone();
                perm.apply(&mut new);
                inverse.apply(&mut new);

                assert_eq!(orig, new);
            }

            {
                assert_eq!(
                    PermS::<SIZE>::cycle().inverse(),
                    PermS::<SIZE>::inverse_cycle()
                );
            }
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::perm_type::PermD;

    impl Permutation for PermD {
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

        fn pow(&self, exp: u32) -> Self {
            let mut mask = u32_leading_bit(exp);
            let mut pow = Self::identity(self.indices.len());

            // power by squaring
            loop {
                if exp & mask != 0 {
                    pow = &pow * self;
                }

                mask >>= 1;

                if mask == 0 {
                    break;
                } else {
                    pow = &pow * &pow;
                }
            }

            pow
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{apply::PermApply, perm_type::PermS};
        use rand::prelude::*;
        use std::collections::HashSet;

        #[test]
        fn dynamic_inverse() {
            const SIZE: usize = 1024;
            {
                let mut rng = rand::thread_rng();

                let mut perm = PermD::identity(SIZE);
                perm.indices.shuffle(&mut rng);
                let inverse = perm.inverse();

                let mut orig = [0usize; SIZE];
                rng.fill(&mut orig);

                let mut new = orig.clone();
                perm.apply(&mut new).unwrap();
                inverse.apply(&mut new).unwrap();

                assert_eq!(orig, new);
            }

            {
                assert_eq!(PermD::cycle(SIZE).inverse(), PermD::inverse_cycle(SIZE));
            }
        }

        #[test]
        fn dynamic_pow() {
            let cycle = PermD::cycle(6);
            assert_eq!(cycle.pow(5), PermD::inverse_cycle(6));
            assert_eq!(cycle.pow(6), PermD::identity(6));

            let set: HashSet<_> = (0..6).map(|exp| cycle.pow(exp)).collect();
            assert_eq!(set.len(), 6);
        }

        #[test]
        fn static_pow() {
            let cycle = PermS::<6>::cycle();
            assert_eq!(cycle.pow(5), PermS::<6>::inverse_cycle());
            assert_eq!(cycle.pow(6), PermS::<6>::identity());

            let set: HashSet<_> = (0..6).map(|exp| cycle.pow(exp)).collect();
            assert_eq!(set.len(), 6);
        }
    }
}

fn inverse_indices(indices: &[usize], inverse_indices: &mut [usize]) {
    indices.iter().enumerate().for_each(|(dst, &src)| {
        inverse_indices[src] = dst;
    });
}

fn u32_leading_bit(n: u32) -> u32 {
    let n = n as u64;
    let n = n | (n >> 1);
    let n = n | (n >> 2);
    let n = n | (n >> 4);
    let n = n | (n >> 8);
    let n = n | (n >> 16);
    ((n + 1) >> 1) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_leading_bit_test() {
        assert_eq!(u32_leading_bit(0b1), 0b1);
        assert_eq!(u32_leading_bit(0b10), 0b10);
        assert_eq!(u32_leading_bit(0b11), 0b10);
        assert_eq!(u32_leading_bit(0b100111), 0b100000);
        assert_eq!(
            u32_leading_bit(0b_11111111_11111111_11111111_11111111),
            0b_10000000_00000000_00000000_00000000
        );
    }
}
