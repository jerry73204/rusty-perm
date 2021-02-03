use crate::common::*;

/// An operator that builds a permutation from a list indexes.
pub trait PermFromIndices<T>
where
    Self: Sized,
{
    fn from_indices(indices: T) -> Option<Self>;
}

mod without_std {
    use super::*;
    use crate::perm_type::PermS;

    impl<const SIZE: usize> PermFromIndices<[usize; SIZE]> for PermS<SIZE> {
        /// Builds a static permutation from an owned array of indices.
        fn from_indices(indices: [usize; SIZE]) -> Option<Self> {
            if !check_indices(indices.as_ref(), &mut [false; SIZE]) {
                return None;
            }
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> PermFromIndices<&[usize; SIZE]> for PermS<SIZE> {
        /// Builds a static permutation from a borrowed array of indices.
        fn from_indices(indices: &[usize; SIZE]) -> Option<Self> {
            Self::from_indices(indices.as_ref())
        }
    }

    impl<const SIZE: usize> PermFromIndices<&[usize]> for PermS<SIZE> {
        /// Builds a static permutation from a slice of indices.
        fn from_indices(indices: &[usize]) -> Option<Self> {
            if indices.len() != SIZE {
                return None;
            }
            if !check_indices(indices, &mut [false; SIZE]) {
                return None;
            }
            Some(Self {
                indices: indices.try_into().unwrap(),
            })
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::perm_type::{PermD, PermS};

    impl PermFromIndices<Cow<'_, [usize]>> for PermD {
        /// Builds a dynamic permutation from a copy-on-write slice of indices.
        fn from_indices(indices: Cow<'_, [usize]>) -> Option<Self> {
            if !check_indices(indices.as_ref(), &mut vec![false; indices.len()]) {
                return None;
            }
            Some(Self {
                indices: indices.into_owned(),
            })
        }
    }

    impl PermFromIndices<Vec<usize>> for PermD {
        /// Builds a dynamic permutation from a vector of indices.
        fn from_indices(indices: Vec<usize>) -> Option<Self> {
            if !check_indices(indices.as_slice(), &mut vec![false; indices.len()]) {
                return None;
            }
            Some(Self { indices })
        }
    }

    impl PermFromIndices<&'_ [usize]> for PermD {
        /// Builds a dynamic permutation from a slice of indices.
        fn from_indices(indices: &[usize]) -> Option<Self> {
            Self::from_indices(Cow::<'_, [usize]>::from(indices))
        }
    }

    impl<const SIZE: usize> PermFromIndices<[usize; SIZE]> for PermD {
        /// Builds a dynamic permutation from an array of indices.
        fn from_indices(indices: [usize; SIZE]) -> Option<Self> {
            Self::from_indices(indices.as_ref())
        }
    }

    impl<const SIZE: usize> PermFromIndices<Vec<usize>> for PermS<SIZE> {
        /// Builds a static permutation from a vector indices.
        fn from_indices(indices: Vec<usize>) -> Option<Self> {
            let indices: &[usize] = indices.as_ref();
            Self::from_indices(indices)
        }
    }

    impl<const SIZE: usize> PermFromIndices<Cow<'_, [usize]>> for PermS<SIZE> {
        /// Builds a static permutation from a copy-on-write slice of indices.
        fn from_indices(indices: Cow<'_, [usize]>) -> Option<Self> {
            Self::from_indices(indices.as_ref())
        }
    }
}

fn check_indices(indices: &[usize], visited: &mut [bool]) -> bool {
    let len = indices.len();
    indices.iter().all(|&index| {
        if index >= len || visited[index] {
            false
        } else {
            visited[index] = true;
            true
        }
    })
}
