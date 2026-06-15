//! The `refining-length` prelude.

#[doc(inline)]
pub use crate::{
    length::HasLength,
    predicates::{
        LengthClosed, LengthClosedOpen, LengthDivisible, LengthEqual, LengthEven, LengthGreater,
        LengthGreaterOrEqual, LengthLess, LengthLessOrEqual, LengthModulo, LengthNonZero,
        LengthNotEqual, LengthOdd, LengthOpen, LengthOpenClosed, LengthZero,
    },
};
