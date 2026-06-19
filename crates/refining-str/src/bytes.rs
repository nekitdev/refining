//! Predicates for checking string bytes.

use core::{fmt, marker::PhantomData};

use refining_core::predicate::{Predicate, PredicateExpected};

/// Checks whether all [`bytes`] of the string satisfy the given predicate.
///
/// [`bytes`]: str::bytes
pub struct BytesAll<P: Predicate<u8> + ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: AsRef<str> + ?Sized, P: Predicate<u8> + ?Sized> Predicate<T> for BytesAll<P> {
    fn check(value: &T) -> bool {
        value.as_ref().bytes().all(P::check_value)
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
            "str::bytes::all<{expected:?}>",
            expected = P::expected()
        )
    }
}

/// Checks whether any [`bytes`] of the string satisfy the given predicate.
///
/// [`bytes`]: str::bytes
pub struct BytesAny<P: Predicate<u8> + ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: AsRef<str> + ?Sized, P: Predicate<u8> + ?Sized> Predicate<T> for BytesAny<P> {
    fn check(value: &T) -> bool {
        value.as_ref().bytes().any(P::check_value)
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
            "str::bytes::any<{expected:?}>",
            expected = P::expected()
        )
    }
}
