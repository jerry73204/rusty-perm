use crate::common::*;

use algorithm::*;

/// An operator that builds a permutation type by sorting slice-like types.
pub trait PermFromSorting<S, T>
where
    S: AsRef<[T]>,
    Self: Sized,
{
    type Output;

    /// Builds a permutation by sorting a slice-like type.
    fn from_sort(vec: S) -> Self::Output
    where
        T: Ord;

    /// Builds a permutation by sorting a slice-like type with a comparing function.
    fn from_sort_by<F>(vec: S, compare: F) -> Self::Output
    where
        F: FnMut(&T, &T) -> Ordering;

    /// Builds a permutation by sorting a slice-like type with a key function.
    fn from_sort_by_key<B, F>(vec: S, f: F) -> Self::Output
    where
        B: Ord,
        F: FnMut(&T) -> B;

    /// Builds a permutation by sorting a slice-like type with a key function. The key is not re-computed twice.
    fn from_sort_by_cached_key<B, F>(vec: S, f: F) -> Self::Output
    where
        B: Ord,
        F: FnMut(&T) -> B;
}

mod without_std {
    use super::*;
    use crate::perm_type::StaticPerm;

    impl<T, const SIZE: usize> PermFromSorting<[T; SIZE], T> for StaticPerm<SIZE> {
        type Output = Self;

        fn from_sort(vec: [T; SIZE]) -> Self::Output
        where
            T: Ord,
        {
            Self::from_sort(&vec)
        }

        fn from_sort_by<F>(vec: [T; SIZE], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(&vec, compare)
        }

        fn from_sort_by_key<B, F>(vec: [T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(&vec, f)
        }

        fn from_sort_by_cached_key<B, F>(vec: [T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(&vec, f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T; SIZE], T> for StaticPerm<SIZE> {
        type Output = Self;

        fn from_sort(vec: &[T; SIZE]) -> Self::Output
        where
            T: Ord,
        {
            let mut perm = Self::identity();
            sort(&mut perm.indices, vec);
            perm
        }

        fn from_sort_by<F>(vec: &[T; SIZE], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            let mut perm = Self::identity();
            sort_by(&mut perm.indices, vec, compare);
            perm
        }

        fn from_sort_by_key<B, F>(vec: &[T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity();
            sort_by_key(&mut perm.indices, vec, f);
            perm
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity();
            sort_by_cached_key(&mut perm.indices, vec, f);
            perm
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T], T> for StaticPerm<SIZE> {
        type Output = Option<Self>;

        fn from_sort(vec: &[T]) -> Self::Output
        where
            T: Ord,
        {
            if vec.len() != SIZE {
                return None;
            }
            let mut perm = Self::identity();
            sort(&mut perm.indices, vec);
            Some(perm)
        }

        fn from_sort_by<F>(vec: &[T], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            if vec.len() != SIZE {
                return None;
            }
            let mut perm = Self::identity();
            sort_by(&mut perm.indices, vec, compare);
            Some(perm)
        }

        fn from_sort_by_key<B, F>(vec: &[T], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            if vec.len() != SIZE {
                return None;
            }
            let mut perm = Self::identity();
            sort_by_key(&mut perm.indices, vec, f);
            Some(perm)
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            if vec.len() != SIZE {
                return None;
            }
            let mut perm = Self::identity();
            sort_by_cached_key(&mut perm.indices, vec, f);
            Some(perm)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::perm_trait::Permutation;
        use rand::prelude::*;

        #[test]
        fn static_perm_from_array() {
            const SIZE: usize = 1024;
            let mut rng = rand::thread_rng();

            for _ in 0..100 {
                let array = {
                    let mut array = [0isize; SIZE];
                    rng.fill(&mut array);
                    array
                };

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort();
                        array
                    };

                    let perm = StaticPerm::from_sort(&array);
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by(|lhs, rhs| lhs.cmp(rhs));
                        array
                    };

                    let perm = StaticPerm::from_sort_by(&array, |lhs, rhs| lhs.cmp(rhs));
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_key(|value| -value);
                        array
                    };

                    let perm = StaticPerm::from_sort_by_key(&array, |value| -value);
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_cached_key(|value| -value);
                        array
                    };

                    let perm = StaticPerm::from_sort_by_cached_key(&array, |value| -value);
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }
            }
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::perm_type::{DynamicPerm, StaticPerm};

    impl<T, const SIZE: usize> PermFromSorting<[T; SIZE], T> for DynamicPerm {
        type Output = Self;

        fn from_sort(vec: [T; SIZE]) -> Self::Output
        where
            T: Ord,
        {
            Self::from_sort(vec.as_ref())
        }

        fn from_sort_by<F>(vec: [T; SIZE], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_ref(), compare)
        }

        fn from_sort_by_key<B, F>(vec: [T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_ref(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: [T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_ref(), f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T; SIZE], T> for DynamicPerm {
        type Output = Self;

        fn from_sort(vec: &[T; SIZE]) -> Self::Output
        where
            T: Ord,
        {
            Self::from_sort(vec.as_ref())
        }

        fn from_sort_by<F>(vec: &[T; SIZE], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_ref(), compare)
        }

        fn from_sort_by_key<B, F>(vec: &[T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_ref(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T; SIZE], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_ref(), f)
        }
    }

    impl<T> PermFromSorting<&[T], T> for DynamicPerm {
        type Output = Self;

        fn from_sort(vec: &[T]) -> Self::Output
        where
            T: Ord,
        {
            let mut perm = Self::identity(vec.len());
            sort(&mut perm.indices, vec);
            perm
        }

        fn from_sort_by<F>(vec: &[T], compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            let mut perm = Self::identity(vec.len());
            sort_by(&mut perm.indices, vec, compare);
            perm
        }

        fn from_sort_by_key<B, F>(vec: &[T], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity(vec.len());
            sort_by_key(&mut perm.indices, vec, f);
            perm
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T], f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity(vec.len());
            sort_by_cached_key(&mut perm.indices, vec, f);
            perm
        }
    }

    impl<T> PermFromSorting<Vec<T>, T> for DynamicPerm {
        type Output = Self;

        fn from_sort(vec: Vec<T>) -> Self::Output
        where
            T: Ord,
        {
            Self::from_sort(vec.as_slice())
        }

        fn from_sort_by<F>(vec: Vec<T>, compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_slice(), compare)
        }

        fn from_sort_by_key<B, F>(vec: Vec<T>, f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_slice(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: Vec<T>, f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_slice(), f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<Vec<T>, T> for StaticPerm<SIZE> {
        type Output = Option<Self>;

        fn from_sort(vec: Vec<T>) -> Self::Output
        where
            T: Ord,
        {
            Self::from_sort(vec.as_slice())
        }

        fn from_sort_by<F>(vec: Vec<T>, compare: F) -> Self::Output
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_slice(), compare)
        }

        fn from_sort_by_key<B, F>(vec: Vec<T>, f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_slice(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: Vec<T>, f: F) -> Self::Output
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_slice(), f)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::perm_trait::Permutation;
        use rand::prelude::*;

        #[test]
        fn static_perm_from_vec() {
            const SIZE: usize = 1024;
            let mut rng = rand::thread_rng();

            for _ in 0..100 {
                let array = {
                    let mut array = vec![0isize; SIZE];
                    rng.fill(array.as_mut_slice());
                    array
                };

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort();
                        array
                    };

                    let perm = StaticPerm::<SIZE>::from_sort(array.as_slice()).unwrap();
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by(|lhs, rhs| lhs.cmp(rhs));
                        array
                    };

                    let perm =
                        StaticPerm::<SIZE>::from_sort_by(array.as_slice(), |lhs, rhs| lhs.cmp(rhs))
                            .unwrap();
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_key(|value| -value);
                        array
                    };

                    let perm =
                        StaticPerm::<SIZE>::from_sort_by_key(array.as_slice(), |value| -value)
                            .unwrap();
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_cached_key(|value| -value);
                        array
                    };

                    let perm =
                        StaticPerm::<SIZE>::from_sort_by_cached_key(array.as_slice(), |value| {
                            -value
                        })
                        .unwrap();
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }
            }
        }

        #[test]
        fn dynamic_perm_from_vec() {
            const SIZE: usize = 1024;
            let mut rng = rand::thread_rng();

            for _ in 0..100 {
                let array = {
                    let mut array = vec![0isize; SIZE];
                    rng.fill(array.as_mut_slice());
                    array
                };

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort();
                        array
                    };

                    let perm = DynamicPerm::from_sort(array.as_slice());
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by(|lhs, rhs| lhs.cmp(rhs));
                        array
                    };

                    let perm = DynamicPerm::from_sort_by(array.as_slice(), |lhs, rhs| lhs.cmp(rhs));
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_key(|value| -value);
                        array
                    };

                    let perm = DynamicPerm::from_sort_by_key(array.as_slice(), |value| -value);
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }

                {
                    let sorted = {
                        let mut array = array.clone();
                        array.sort_by_cached_key(|value| -value);
                        array
                    };

                    let perm =
                        DynamicPerm::from_sort_by_cached_key(array.as_slice(), |value| -value);
                    perm.indices().iter().enumerate().for_each(|(dst, &src)| {
                        assert_eq!(sorted[dst], array[src]);
                    });
                }
            }
        }
    }
}

#[cfg(not(feature = "std"))]
mod algorithm {
    use super::*;

    pub fn sort<T>(identity: &mut [usize], vec: &[T])
    where
        T: Ord,
    {
        quicksort(identity, |&lhs, &rhs| vec[lhs].cmp(&vec[rhs]));
    }

    pub fn sort_by<T, F>(identity: &mut [usize], vec: &[T], mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        quicksort(identity, |&lhs, &rhs| compare(&vec[lhs], &vec[rhs]));
    }

    pub fn sort_by_key<T, B, F>(identity: &mut [usize], vec: &[T], mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        quicksort(identity, |&lhs, &rhs| f(&vec[lhs]).cmp(&f(&vec[rhs])));
    }

    pub fn sort_by_cached_key<T, B, F>(identity: &mut [usize], vec: &[T], mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        quicksort(identity, |&lhs, &rhs| f(&vec[lhs]).cmp(&f(&vec[rhs])));
    }

    fn quicksort<T, F>(slice: &mut [T], mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        unsafe {
            quicksort_unsafe(slice.as_mut_ptr(), slice.len(), &mut compare);
        }
    }

    unsafe fn quicksort_unsafe<T, F>(slice: *mut T, len: usize, compare: &mut F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        if len <= 1 {
            return;
        }

        let mut lid = 1;
        let mut rid = len;

        // partition
        while lid < rid {
            match compare(slice.add(lid).as_ref().unwrap(), slice.as_ref().unwrap()) {
                Ordering::Less | Ordering::Equal => lid += 1,
                Ordering::Greater => {
                    rid -= 1;
                    mem::swap(
                        slice.add(lid).as_mut().unwrap(),
                        slice.add(rid).as_mut().unwrap(),
                    );
                }
            }
        }

        // move pivot
        mem::swap(
            slice.as_mut().unwrap(),
            slice.add(lid - 1).as_mut().unwrap(),
        );

        quicksort_unsafe(slice, lid, compare);
        quicksort_unsafe(slice.add(lid), len - lid, compare);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::prelude::*;

        #[test]
        fn quicksort_test() {
            let mut rng = rand::thread_rng();

            {
                let mut values = [0; 0];
                quicksort(&mut values, |lhs, rhs| lhs.cmp(rhs));
            }

            {
                let value: usize = rng.gen();
                let mut values = [value];
                quicksort(&mut values, |lhs, rhs| lhs.cmp(rhs));
                assert!(values[0] == value);
            }

            for _ in 0..100 {
                let first: usize = rng.gen();
                let second: usize = rng.gen();
                let mut values = [first, second];
                quicksort(&mut values, |lhs, rhs| lhs.cmp(rhs));

                if first < second {
                    assert!(values == [first, second]);
                } else {
                    assert!(values == [second, first]);
                }
            }

            {
                for _ in 0..1000 {
                    let mut values = [0usize; 1024];
                    rng.fill(&mut values[..]);
                    quicksort(&mut values, |lhs, rhs| lhs.cmp(rhs));
                    let is_correct = values
                        .iter()
                        .scan(None, |prev, &curr| {
                            let orig_prev = *prev;
                            *prev = Some(curr);
                            let correct = orig_prev.map(|prev| prev <= curr).unwrap_or(true);
                            Some(correct)
                        })
                        .all(|correct| correct);
                    assert!(is_correct);
                }
            }
        }
    }
}

#[cfg(feature = "std")]
mod algorithm {
    use super::*;

    pub fn sort<T>(identity: &mut [usize], vec: &[T])
    where
        T: Ord,
    {
        identity.sort_by_key(|&index| &vec[index]);
    }

    pub fn sort_by<T, F>(identity: &mut [usize], vec: &[T], mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        identity.sort_by(|&lhs, &rhs| compare(&vec[lhs], &vec[rhs]));
    }

    pub fn sort_by_key<T, B, F>(identity: &mut [usize], vec: &[T], mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        identity.sort_by_key(|&index| f(&vec[index]));
    }

    pub fn sort_by_cached_key<T, B, F>(identity: &mut [usize], vec: &[T], mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        identity.sort_by_cached_key(|&index| f(&vec[index]));
    }
}
