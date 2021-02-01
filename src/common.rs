#[cfg(not(feature = "no_std"))]
pub use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    mem,
    ops::Mul,
};

#[cfg(feature = "no_std")]
pub use core::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    mem,
    ops::Mul,
};
