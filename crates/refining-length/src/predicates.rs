//! Predicates based on length.

use core::{fmt, marker::PhantomData};

use refining_core::{
    logical::{And, Not},
    predicate::Predicate,
};

use crate::length::HasLength;

/// Checks whether the value has length equal to `N`.
pub struct LengthEqual<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthEqual<N> {
    fn check(value: &T) -> bool {
        value.length() == N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length == {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::eq<{N}>")
    }
}

/// Checks whether the value has length not equal to `N`.
pub struct LengthNotEqual<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthNotEqual<N> {
    fn check(value: &T) -> bool {
        value.length() != N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length != {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::ne<{N}>")
    }
}

/// Checks whether the value has length less than `N`.
pub struct LengthLess<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthLess<N> {
    fn check(value: &T) -> bool {
        value.length() < N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length < {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::lt<{N}>")
    }
}

/// Checks whether the value has length greater than `N`.
pub struct LengthGreater<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthGreater<N> {
    fn check(value: &T) -> bool {
        value.length() > N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length > {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::gt<{N}>")
    }
}

/// Checks whether the value has length less than or equal to `N`.
pub struct LengthLessOrEqual<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthLessOrEqual<N> {
    fn check(value: &T) -> bool {
        value.length() <= N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length <= {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::le<{N}>")
    }
}

/// Checks whether the value has length greater than or equal to `N`.
pub struct LengthGreaterOrEqual<const N: usize> {
    private: PhantomData<()>,
}

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LengthGreaterOrEqual<N> {
    fn check(value: &T) -> bool {
        value.length() >= N
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length >= {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::ge<{N}>")
    }
}

/// Represents `(M, N)` intervals.
pub type LengthOpen<const M: usize, const N: usize> = And<LengthGreater<M>, LengthLess<N>>;

/// Represents `[M, N)` intervals.
pub type LengthClosedOpen<const M: usize, const N: usize> =
    And<LengthGreaterOrEqual<M>, LengthLess<N>>;

/// Represents `(M, N]` intervals.
pub type LengthOpenClosed<const M: usize, const N: usize> =
    And<LengthGreater<M>, LengthLessOrEqual<N>>;

/// Represents `[M, N]` intervals.
pub type LengthClosed<const M: usize, const N: usize> =
    And<LengthGreaterOrEqual<M>, LengthLessOrEqual<N>>;

/// Zero (`0`) length.
pub const ZERO: usize = 0;

/// Checks whether the given value has zero length.
pub type LengthZero = LengthEqual<ZERO>;

/// Checks whether the given value has non-zero length.
pub type LengthNonZero = LengthNotEqual<ZERO>;

/// Checks whether the given value has length that has modulo `M` when divided by `D`.
pub struct LengthModulo<const D: usize, const M: usize> {
    private: PhantomData<()>,
}

impl<const D: usize, const M: usize, T: HasLength + ?Sized> Predicate<T> for LengthModulo<D, M> {
    fn check(value: &T) -> bool {
        value.length() % D == M
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length % {D} == {M}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::mod<{D}, {M}>")
    }
}

/// Checks whether the given value length is divisible by `D`.
pub type LengthDivisible<const D: usize> = LengthModulo<D, ZERO>;

/// Two (`2`) length.
pub const TWO: usize = 2;

/// Checks whether the given value length is even.
pub type LengthEven = LengthDivisible<TWO>;

/// Checks whether the given value length is odd.
pub type LengthOdd = Not<LengthEven>;
