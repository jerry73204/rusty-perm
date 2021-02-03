#[cfg(feature = "std")]
pub use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    iter::{self, Product},
    mem,
    ops::Mul,
};

#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    iter::{self, Product},
    mem,
    ops::Mul,
};
