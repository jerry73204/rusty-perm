use crate::{common::*, perm_trait::Permutation, size::PermSize};

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
    pub type PermS<const SIZE: usize> = Perm<Static<{ SIZE }>>;

    /// Permutation type with static size 0.
    pub type Perm0 = PermS<0>;

    /// Permutation type with static size 1.
    pub type Perm1 = PermS<1>;

    /// Permutation type with static size 2.
    pub type Perm2 = PermS<2>;

    /// Permutation type with static size 4.
    pub type Perm4 = PermS<4>;

    /// Permutation type with static size 5.
    pub type Perm5 = PermS<5>;

    /// Permutation type with static size 6.
    pub type Perm6 = PermS<6>;

    /// Permutation type with static size 7.
    pub type Perm7 = PermS<7>;

    /// Permutation type with static size 8.
    pub type Perm8 = PermS<8>;

    /// Permutation type with static size 9.
    pub type Perm9 = PermS<9>;

    /// Permutation type with static size 10.
    pub type Perm10 = PermS<10>;

    /// Permutation type with static size 11.
    pub type Perm11 = PermS<11>;

    /// Permutation type with static size 12.
    pub type Perm12 = PermS<12>;

    /// Permutation type with static size 13.
    pub type Perm13 = PermS<13>;

    /// Permutation type with static size 14.
    pub type Perm14 = PermS<14>;

    /// Permutation type with static size 15.
    pub type Perm15 = PermS<15>;

    /// Permutation type with static size 16.
    pub type Perm16 = PermS<16>;

    /// Permutation type with static size 17.
    pub type Perm17 = PermS<17>;

    /// Permutation type with static size 18.
    pub type Perm18 = PermS<18>;

    /// Permutation type with static size 19.
    pub type Perm19 = PermS<19>;

    /// Permutation type with static size 20.
    pub type Perm20 = PermS<20>;

    /// Permutation type with static size 21.
    pub type Perm21 = PermS<21>;

    /// Permutation type with static size 22.
    pub type Perm22 = PermS<22>;

    /// Permutation type with static size 23.
    pub type Perm23 = PermS<23>;

    /// Permutation type with static size 24.
    pub type Perm24 = PermS<24>;

    /// Permutation type with static size 25.
    pub type Perm25 = PermS<25>;

    /// Permutation type with static size 26.
    pub type Perm26 = PermS<26>;

    /// Permutation type with static size 27.
    pub type Perm27 = PermS<27>;

    /// Permutation type with static size 28.
    pub type Perm28 = PermS<28>;

    /// Permutation type with static size 29.
    pub type Perm29 = PermS<29>;

    /// Permutation type with static size 30.
    pub type Perm30 = PermS<30>;

    /// Permutation type with static size 31.
    pub type Perm31 = PermS<31>;

    /// Permutation type with static size 32.
    pub type Perm32 = PermS<32>;

    impl<const SIZE: usize> PermS<SIZE> {
        pub fn identity() -> Self {
            let mut indices = [0; SIZE];
            (0..SIZE).for_each(|index| indices[index] = index);
            Self { indices }
        }

        pub fn swap(first: usize, second: usize) -> Option<Self> {
            if first >= SIZE || second >= SIZE || first == second {
                return None;
            }

            let min = first.min(second);
            let max = first.max(second);

            let mut indices = [0; SIZE];

            indices[min] = max;
            indices[max] = min;
            (0..min).for_each(|index| {
                indices[index] = index;
            });
            ((min + 1)..max).for_each(|index| {
                indices[index] = index;
            });
            ((max + 1)..SIZE).for_each(|index| {
                indices[index] = index;
            });

            Some(Self { indices })
        }

        pub fn cycle() -> Self {
            let mut indices = [0; SIZE];
            iter::once(SIZE - 1)
                .chain(0..(SIZE - 1))
                .enumerate()
                .for_each(|(dst, src)| {
                    indices[dst] = src;
                });
            Self { indices }
        }

        pub fn reverse_cycle() -> Self {
            let mut indices = [0; SIZE];
            (1..SIZE)
                .chain(iter::once(0))
                .enumerate()
                .for_each(|(dst, src)| {
                    indices[dst] = src;
                });
            Self { indices }
        }

        pub fn permute_indices(&self, perm: &PermS<SIZE>) -> Self {
            self.conjugate_with(perm)
        }

        pub fn conjugate_with(&self, other: &PermS<SIZE>) -> Self {
            &(&other.inverse() * self) * other
        }

        pub fn to_size<const NEW_SIZE: usize>(&self) -> Option<PermS<NEW_SIZE>> {
            if SIZE > NEW_SIZE {
                for dst in NEW_SIZE..SIZE {
                    let src = self.indices[dst];
                    if src != dst {
                        return None;
                    }
                }

                let mut new_indices = [0; NEW_SIZE];
                new_indices.copy_from_slice(&self.indices[..NEW_SIZE]);
                Some(PermS {
                    indices: new_indices,
                })
            } else {
                let mut new_indices = [0; NEW_SIZE];
                new_indices[..SIZE].copy_from_slice(&self.indices[..SIZE]);

                (SIZE..NEW_SIZE).for_each(|index| {
                    new_indices[index] = index;
                });

                Some(PermS {
                    indices: new_indices,
                })
            }
        }
    }

    impl Perm0 {
        pub fn empty() -> Self {
            Self { indices: [] }
        }
    }

    impl Perm1 {
        pub fn unit() -> Self {
            Self { indices: [0] }
        }
    }

    impl Perm2 {
        pub fn swap2() -> Self {
            Self { indices: [1, 0] }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{apply::PermApply, from_indices::PermFromIndices};
        use rand::prelude::*;

        #[test]
        fn static_identity() {
            const SIZE: usize = 2014;

            let perm = PermS::<SIZE>::identity();

            let mut rng = rand::thread_rng();
            let mut orig = [0usize; SIZE];
            rng.fill(orig.as_mut());

            let mut new = orig.clone();
            perm.apply(&mut new);

            assert_eq!(orig, new);
        }

        #[test]
        fn static_swap() {
            let perm = PermS::<6>::swap(5, 3).unwrap();
            assert_eq!(perm.indices(), &[0, 1, 2, 5, 4, 3]);
        }

        #[test]
        fn static_permute_indices() {
            let index_map = PermS::from_indices([3, 5, 0, 1, 2, 4]).unwrap();
            let orig = PermS::<6>::swap(5, 3).unwrap();
            let new = orig.permute_indices(&index_map);
            assert_eq!(new, PermS::<6>::swap(0, 1).unwrap());
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::{product::PermProduct, size::Dynamic};

    /// Permutation type with runtime size.
    pub type PermD = Perm<Dynamic>;

    impl PermD {
        pub fn empty() -> Self {
            Self { indices: vec![] }
        }

        pub fn unit() -> Self {
            Self { indices: vec![0] }
        }

        pub fn swap(size: usize, first: usize, second: usize) -> Option<Self> {
            if first >= size || second >= size || first == second {
                return None;
            }

            let min = first.min(second);
            let max = first.max(second);

            let indices: Vec<_> = (0..min)
                .chain(iter::once(max))
                .chain((min + 1)..max)
                .chain(iter::once(min))
                .chain((max + 1)..size)
                .collect();

            Some(Self { indices })
        }

        pub fn identity(size: usize) -> Self {
            let mut indices = vec![0; size];
            (0..size).for_each(|index| indices[index] = index);
            Self { indices }
        }

        pub fn cycle(size: usize) -> Self {
            let mut indices = vec![0; size];
            iter::once(size - 1)
                .chain(0..(size - 1))
                .enumerate()
                .for_each(|(dst, src)| {
                    indices[dst] = src;
                });
            Self { indices }
        }

        pub fn reverse_cycle(size: usize) -> Self {
            let mut indices = vec![0; size];
            (1..size)
                .chain(iter::once(0))
                .enumerate()
                .for_each(|(dst, src)| {
                    indices[dst] = src;
                });
            Self { indices }
        }

        pub fn permute_indices(&self, perm: &PermD) -> Option<Self> {
            self.conjugate_with(perm)
        }

        pub fn conjugate_with(&self, other: &PermD) -> Option<Self> {
            other.inverse().perm_product(self)?.perm_product(other)
        }

        pub fn to_size(&self, new_size: usize) -> Option<PermD> {
            let orig_size = self.indices.len();
            if orig_size > new_size {
                for dst in new_size..orig_size {
                    let src = self.indices[dst];
                    if src != dst {
                        return None;
                    }
                }

                let mut new_indices = vec![0; new_size];
                new_indices.copy_from_slice(&self.indices[..new_size]);
                Some(PermD {
                    indices: new_indices,
                })
            } else {
                let mut new_indices = vec![0; new_size];
                new_indices[..orig_size].copy_from_slice(&self.indices[..orig_size]);

                (orig_size..new_size).for_each(|index| {
                    new_indices[index] = index;
                });

                Some(PermD {
                    indices: new_indices,
                })
            }
        }

        pub fn into_static<const SIZE: usize>(self) -> Option<PermS<SIZE>> {
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
        use crate::{apply::PermApply, from_indices::PermFromIndices};
        use rand::prelude::*;

        #[test]
        fn dynamic_identity() {
            const SIZE: usize = 2014;

            let perm = PermD::identity(SIZE);

            let mut rng = rand::thread_rng();
            let mut orig = [0usize; SIZE];
            rng.fill(orig.as_mut());

            let mut new = orig.clone();
            perm.apply(&mut new).unwrap();

            assert_eq!(orig, new);
        }

        #[test]
        fn dynamic_swap() {
            let perm = PermD::swap(6, 5, 3).unwrap();
            assert_eq!(perm.indices(), &[0, 1, 2, 5, 4, 3]);
        }

        #[test]
        fn dynamic_permute_indices() {
            let index_map = PermD::from_indices([3, 5, 0, 1, 2, 4]).unwrap();
            let orig = PermD::swap(6, 5, 3).unwrap();
            let new = orig.permute_indices(&index_map).unwrap();
            assert_eq!(new, PermD::swap(6, 0, 1).unwrap());
        }
    }
}
