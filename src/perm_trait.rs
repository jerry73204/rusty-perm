use crate::common::*;

pub trait Permutation {
    fn len(&self) -> usize;
    fn inverse(&self) -> Self;
    fn indices(&self) -> &[usize];
    fn apply<'a, T>(&self, slice: &'a mut [T]) -> Result<&'a mut [T], &'static str>;
}

mod static_ {
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

        fn apply<'a, T>(&self, slice: &'a mut [T]) -> Result<&'a mut [T], &'static str> {
            if slice.len() != SIZE {
                return Err("input slice length mismatch");
            }
            let mut visited = [false; SIZE];
            apply(&self.inverse().indices, &mut visited, slice);
            Ok(slice)
        }
    }
}

#[cfg(not(feature = "no_std"))]
mod dynamic {
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

        fn apply<'a, T>(&self, slice: &'a mut [T]) -> Result<&'a mut [T], &'static str> {
            let len = self.indices.len();
            if slice.len() != len {
                return Err("input slice length mismatch");
            }
            let mut visited = vec![false; len];
            apply(&self.inverse().indices, &mut visited, slice);
            Ok(slice)
        }
    }
}

fn inverse_indices(src_indices: &[usize], dst_indices: &mut [usize]) {
    src_indices.iter().enumerate().for_each(|(src, &dst)| {
        dst_indices[dst] = src;
    });
}

fn apply<T>(inverse_indices: &[usize], visited: &mut [bool], slice: &mut [T]) {
    unsafe { apply_unsafe(inverse_indices, visited, slice.as_mut_ptr()) }
}

unsafe fn apply_unsafe<T>(inverse_indices: &[usize], visited: &mut [bool], slice: *mut T) {
    let len = inverse_indices.len();

    for idx in 0..len {
        let mut src = idx;

        loop {
            let dst = inverse_indices[src];
            if visited[dst] {
                break;
            }

            mem::swap(
                slice.add(src).as_mut().unwrap(),
                slice.add(dst).as_mut().unwrap(),
            );
            visited[src] = true;
            src = dst;
        }
    }
}
