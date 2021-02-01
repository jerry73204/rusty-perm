#![cfg(feature = "rand")]

use crate::perm_type::StaticPerm;
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

impl<const SIZE: usize> Distribution<StaticPerm<SIZE>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StaticPerm<SIZE> {
        let mut perm = StaticPerm::<SIZE>::identity();
        perm.indices.shuffle(rng);
        perm
    }
}
