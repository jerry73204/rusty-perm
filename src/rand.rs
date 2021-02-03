#![cfg(feature = "rand")]

use crate::perm_type::PermS;
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

impl<const SIZE: usize> Distribution<PermS<SIZE>> for Standard {
    /// Sample an random static permutation.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermS<SIZE> {
        let mut perm = PermS::<SIZE>::identity();
        perm.indices.shuffle(rng);
        perm
    }
}
