use crate::common::*;

/// The permutation operator on slice-like types.
pub trait PermApply<T>
where
    T: ?Sized,
{
    type Output;

    fn apply(&self, input: &mut T) -> Self::Output;
}

mod without_std {
    use super::*;
    use crate::perm_type::PermS;

    impl<T, const SIZE: usize> PermApply<[T; SIZE]> for PermS<SIZE> {
        type Output = ();

        fn apply(&self, input: &mut [T; SIZE]) -> Self::Output {
            let mut visited = [false; SIZE];
            apply(&self.indices, &mut visited, input);
        }
    }

    impl<T, const SIZE: usize> PermApply<[T]> for PermS<SIZE> {
        type Output = Result<(), &'static str>;

        fn apply(&self, input: &mut [T]) -> Self::Output {
            if input.len() != SIZE {
                return Err("input slice length mismatch");
            }
            let mut visited = [false; SIZE];
            apply(&self.indices, &mut visited, input);
            Ok(())
        }
    }
}

#[cfg(feature = "std")]
mod with_std {
    use super::*;
    use crate::perm_type::{PermD, PermS};

    impl<T, const SIZE: usize> PermApply<[T; SIZE]> for PermD {
        type Output = Result<(), &'static str>;

        fn apply(&self, input: &mut [T; SIZE]) -> Self::Output {
            let len = self.indices.len();
            if len != SIZE {
                return Err("input slice length mismatch");
            }
            let mut visited = vec![false; len];
            apply(&self.indices, &mut visited, input);
            Ok(())
        }
    }

    impl<T> PermApply<[T]> for PermD {
        type Output = Result<(), &'static str>;

        fn apply(&self, input: &mut [T]) -> Self::Output {
            let len = self.indices.len();
            if len != input.len() {
                return Err("input slice length mismatch");
            }
            let mut visited = vec![false; len];
            apply(&self.indices, &mut visited, input);
            Ok(())
        }
    }

    impl<T> PermApply<Vec<T>> for PermD {
        type Output = Result<(), &'static str>;

        fn apply(&self, input: &mut Vec<T>) -> Self::Output {
            let len = self.indices.len();
            if len != input.len() {
                return Err("input slice length mismatch");
            }
            let mut visited = vec![false; len];
            apply(&self.indices, &mut visited, input);
            Ok(())
        }
    }

    impl<T, const SIZE: usize> PermApply<Vec<T>> for PermS<SIZE> {
        type Output = Result<(), &'static str>;

        fn apply(&self, input: &mut Vec<T>) -> Self::Output {
            self.apply(input.as_mut_slice())
        }
    }
}

fn apply<T>(indices: &[usize], visited: &mut [bool], slice: &mut [T]) {
    unsafe { apply_unsafe(indices, visited, slice.as_mut_ptr()) }
}

unsafe fn apply_unsafe<T>(indices: &[usize], visited: &mut [bool], slice: *mut T) {
    let len = indices.len();

    for idx in 0..len {
        let mut dst = idx;

        if visited[dst] {
            continue;
        }

        loop {
            visited[dst] = true;

            let src = indices[dst];
            if visited[src] {
                break;
            }

            mem::swap(
                slice.add(src).as_mut().unwrap(),
                slice.add(dst).as_mut().unwrap(),
            );
            dst = src;
        }
    }
}
