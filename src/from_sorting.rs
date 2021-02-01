use crate::common::*;

use algorithm::*;

pub trait PermFromSorting<S, T>
where
    S: AsRef<[T]>,
    Self: Sized,
{
    fn from_sort(vec: S) -> Option<Self>
    where
        T: Ord;
    fn from_sort_by<F>(vec: S, compare: F) -> Option<Self>
    where
        F: FnMut(&T, &T) -> Ordering;
    fn from_sort_by_key<B, F>(vec: S, f: F) -> Option<Self>
    where
        B: Ord,
        F: FnMut(&T) -> B;
    fn from_sort_by_cached_key<B, F>(vec: S, f: F) -> Option<Self>
    where
        B: Ord,
        F: FnMut(&T) -> B;
}

mod without_std {
    use super::*;
    use crate::perm_type::StaticPerm;

    impl<T, const SIZE: usize> PermFromSorting<[T; SIZE], T> for StaticPerm<SIZE> {
        fn from_sort(vec: [T; SIZE]) -> Option<Self>
        where
            T: Ord,
        {
            Self::from_sort(&vec)
        }

        fn from_sort_by<F>(vec: [T; SIZE], compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(&vec, compare)
        }

        fn from_sort_by_key<B, F>(vec: [T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(&vec, f)
        }

        fn from_sort_by_cached_key<B, F>(vec: [T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(&vec, f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T; SIZE], T> for StaticPerm<SIZE> {
        fn from_sort(vec: &[T; SIZE]) -> Option<Self>
        where
            T: Ord,
        {
            let mut perm = Self::identity();
            sort(&mut perm.indices, vec);
            Some(perm)
        }

        fn from_sort_by<F>(vec: &[T; SIZE], compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            let mut perm = Self::identity();
            sort_by(&mut perm.indices, vec, compare);
            Some(perm)
        }

        fn from_sort_by_key<B, F>(vec: &[T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity();
            sort_by_key(&mut perm.indices, vec, f);
            Some(perm)
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity();
            sort_by_cached_key(&mut perm.indices, vec, f);
            Some(perm)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T], T> for StaticPerm<SIZE> {
        fn from_sort(vec: &[T]) -> Option<Self>
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

        fn from_sort_by<F>(vec: &[T], compare: F) -> Option<Self>
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

        fn from_sort_by_key<B, F>(vec: &[T], f: F) -> Option<Self>
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

        fn from_sort_by_cached_key<B, F>(vec: &[T], f: F) -> Option<Self>
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
}

#[cfg(not(feature = "no_std"))]
mod with_std {
    use super::*;
    use crate::perm_type::{DynamicPerm, StaticPerm};

    impl<T, const SIZE: usize> PermFromSorting<[T; SIZE], T> for DynamicPerm {
        fn from_sort(vec: [T; SIZE]) -> Option<Self>
        where
            T: Ord,
        {
            Self::from_sort(vec.as_ref())
        }

        fn from_sort_by<F>(vec: [T; SIZE], compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_ref(), compare)
        }

        fn from_sort_by_key<B, F>(vec: [T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_ref(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: [T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_ref(), f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<&[T; SIZE], T> for DynamicPerm {
        fn from_sort(vec: &[T; SIZE]) -> Option<Self>
        where
            T: Ord,
        {
            Self::from_sort(vec.as_ref())
        }

        fn from_sort_by<F>(vec: &[T; SIZE], compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_ref(), compare)
        }

        fn from_sort_by_key<B, F>(vec: &[T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_ref(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T; SIZE], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_ref(), f)
        }
    }

    impl<T> PermFromSorting<&[T], T> for DynamicPerm {
        fn from_sort(vec: &[T]) -> Option<Self>
        where
            T: Ord,
        {
            let mut perm = Self::identity(vec.len());
            sort(&mut perm.indices, vec);
            Some(perm)
        }

        fn from_sort_by<F>(vec: &[T], compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            let mut perm = Self::identity(vec.len());
            sort_by(&mut perm.indices, vec, compare);
            Some(perm)
        }

        fn from_sort_by_key<B, F>(vec: &[T], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity(vec.len());
            sort_by_key(&mut perm.indices, vec, f);
            Some(perm)
        }

        fn from_sort_by_cached_key<B, F>(vec: &[T], f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            let mut perm = Self::identity(vec.len());
            sort_by_cached_key(&mut perm.indices, vec, f);
            Some(perm)
        }
    }

    impl<T> PermFromSorting<Vec<T>, T> for DynamicPerm {
        fn from_sort(vec: Vec<T>) -> Option<Self>
        where
            T: Ord,
        {
            Self::from_sort(vec.as_slice())
        }

        fn from_sort_by<F>(vec: Vec<T>, compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_slice(), compare)
        }

        fn from_sort_by_key<B, F>(vec: Vec<T>, f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_slice(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: Vec<T>, f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_slice(), f)
        }
    }

    impl<T, const SIZE: usize> PermFromSorting<Vec<T>, T> for StaticPerm<SIZE> {
        fn from_sort(vec: Vec<T>) -> Option<Self>
        where
            T: Ord,
        {
            Self::from_sort(vec.as_slice())
        }

        fn from_sort_by<F>(vec: Vec<T>, compare: F) -> Option<Self>
        where
            F: FnMut(&T, &T) -> Ordering,
        {
            Self::from_sort_by(vec.as_slice(), compare)
        }

        fn from_sort_by_key<B, F>(vec: Vec<T>, f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_key(vec.as_slice(), f)
        }

        fn from_sort_by_cached_key<B, F>(vec: Vec<T>, f: F) -> Option<Self>
        where
            B: Ord,
            F: FnMut(&T) -> B,
        {
            Self::from_sort_by_cached_key(vec.as_slice(), f)
        }
    }
}

#[cfg(feature = "no_std")]
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

#[cfg(not(feature = "no_std"))]
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
