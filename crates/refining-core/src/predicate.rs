//! Core predicate traits.

use core::{fmt, marker::PhantomData};

/// Represents predicates over values of type `T`.
pub trait Predicate<T: ?Sized> {
    /// Checks the given reference to value of type `T`.
    ///
    /// Returns [`true`] if the predicate is satisfied by this value, [`false`] otherwise.
    fn check(value: &T) -> bool;

    /// Checks the given value of type `T`.
    ///
    /// Returns [`true`] if the predicate is satisfied by this value, [`false`] otherwise.
    ///
    /// The default implementation simply calls [`check`] with value reference.
    ///
    /// [`check`]: Self::check
    fn check_value(value: T) -> bool
    where
        T: Sized,
    {
        Self::check(&value)
    }

    /// Formats the expectation of this predicate into the given formatter.
    ///
    /// # Errors
    ///
    /// Returns [`fmt::Error`] in case formatting fails.
    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Formats the expectation code of this predicate into the given formatter.
    ///
    /// # Errors
    ///
    /// Returns [`fmt::Error`] in case formatting fails.
    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// Extends values `P` implementing [`Predicate<T>`] to return [`Expected<T, P>`] for formatting.
pub trait PredicateExpected<T: ?Sized>: Predicate<T> {
    /// Returns the [`Expected<T, Self>`] value used for formatting.
    #[must_use]
    fn expected() -> Expected<T, Self> {
        Expected::new()
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> PredicateExpected<T> for P {}

/// Represents the expected type used for formatting.
///
/// Returned by the [`expected`] method of the [`PredicateExpected`] extension trait.
///
/// This type delegates the following traits:
///
/// - [`fmt::Debug`] to [`expect_code`];
/// - [`fmt::Display`] to [`expect`].
///
/// [`expected`]: PredicateExpected::expected
/// [`expect_code`]: Predicate::expect_code
/// [`expect`]: Predicate::expect
pub struct Expected<T: ?Sized, P: Predicate<T> + ?Sized> {
    value: PhantomData<T>,
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Default for Expected<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Expected<T, P> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: PhantomData,
            predicate: PhantomData,
        }
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> fmt::Debug for Expected<T, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::expect_code(formatter)
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> fmt::Display for Expected<T, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::expect(formatter)
    }
}

/// Extends any type `T` to check whether values satisfy given [`Predicate<T>`].
pub trait Check {
    /// Checks whether the [`Self`] value satisfies the given predicate.
    ///
    /// This method calls [`P::check`] in its implementation.
    ///
    /// [`P::check`]: Predicate::check
    fn satisfies<P: Predicate<Self> + ?Sized>(&self) -> bool {
        P::check(self)
    }
}

impl<T: ?Sized> Check for T {}
