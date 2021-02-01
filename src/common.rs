#[cfg(feature = "std")]
pub use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    mem,
    ops::Mul,
};

#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    mem,
    ops::Mul,
};
