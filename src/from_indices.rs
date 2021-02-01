use crate::common::*;

pub trait PermFromIndices<T>
where
    Self: Sized,
{
    fn from_indices(indices: T) -> Option<Self>;
}

mod static_ {
    use super::*;
    use crate::perm_type::StaticPerm;

    impl<const SIZE: usize> PermFromIndices<[usize; SIZE]> for StaticPerm<SIZE> {
        fn from_indices(indices: [usize; SIZE]) -> Option<Self> {
            if !check_indices(indices.as_ref(), &mut [false; SIZE]) {
                return None;
            }
            Some(Self { indices })
        }
    }

    impl<const SIZE: usize> PermFromIndices<&[usize]> for StaticPerm<SIZE> {
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

#[cfg(not(feature = "no_std"))]
mod dynamic {
    use super::*;
    use crate::perm_type::{DynamicPerm, StaticPerm};

    impl PermFromIndices<Cow<'_, [usize]>> for DynamicPerm {
        fn from_indices(indices: Cow<'_, [usize]>) -> Option<Self> {
            if !check_indices(indices.as_ref(), &mut vec![false; indices.len()]) {
                return None;
            }
            Some(Self {
                indices: indices.into_owned(),
            })
        }
    }

    impl PermFromIndices<Vec<usize>> for DynamicPerm {
        fn from_indices(indices: Vec<usize>) -> Option<Self> {
            if !check_indices(indices.as_slice(), &mut vec![false; indices.len()]) {
                return None;
            }
            Some(Self { indices })
        }
    }

    impl PermFromIndices<&'_ [usize]> for DynamicPerm {
        fn from_indices(indices: &[usize]) -> Option<Self> {
            Self::from_indices(Cow::<'_, [usize]>::from(indices))
        }
    }

    impl<const SIZE: usize> PermFromIndices<[usize; SIZE]> for DynamicPerm {
        fn from_indices(indices: [usize; SIZE]) -> Option<Self> {
            Self::from_indices(indices.as_ref())
        }
    }

    impl<const SIZE: usize> PermFromIndices<Vec<usize>> for StaticPerm<SIZE> {
        fn from_indices(indices: Vec<usize>) -> Option<Self> {
            Self::from_indices(indices.as_ref())
        }
    }

    impl<const SIZE: usize> PermFromIndices<Cow<'_, [usize]>> for StaticPerm<SIZE> {
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
