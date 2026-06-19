//! Predicates for checking string characters.

use core::{fmt, marker::PhantomData};

use refining_core::predicate::{Predicate, PredicateExpected};

/// Checks whether all [`chars`] of the string satisfy the given predicate.
///
/// [`chars`]: str::chars
pub struct CharsAll<P: Predicate<char> + ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: AsRef<str> + ?Sized, P: Predicate<char> + ?Sized> Predicate<T> for CharsAll<P> {
    fn check(value: &T) -> bool {
        value.as_ref().chars().all(P::check_value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string with all ({expected})",
            expected = P::expected()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "str::chars::all<{expected:?}>",
            expected = P::expected()
        )
    }
}

/// Checks whether any [`chars`] of the string satisfy the given predicate.
///
/// [`chars`]: str::chars
pub struct CharsAny<P: Predicate<char> + ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: AsRef<str> + ?Sized, P: Predicate<char> + ?Sized> Predicate<T> for CharsAny<P> {
    fn check(value: &T) -> bool {
        value.as_ref().chars().any(P::check_value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string with any ({expected})",
            expected = P::expected()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "str::chars::any<{expected:?}>",
            expected = P::expected()
        )
    }
}
