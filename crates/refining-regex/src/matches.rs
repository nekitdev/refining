//! Predicates based on regular expression matches.

use core::{fmt, marker::PhantomData};

use refining_core::{predicate::Predicate, types::StaticStr};

use crate::types::{StaticRegex, TypeRegex};

/// Checks whether the given string matches the regular expression `R`.
pub struct Matches<R: TypeRegex + ?Sized> {
    regex: PhantomData<R>,
}

/// The `regex::matches` literal.
pub const MATCHES: &str = "regex::matches";

impl<R: TypeRegex + ?Sized> Matches<R> {
    /// Returns the compiled [`StaticRegex`].
    #[must_use]
    pub fn regex() -> StaticRegex {
        R::get()
    }

    /// Returns the regular expression pattern.
    #[must_use]
    pub fn pattern() -> StaticStr {
        Self::regex().as_str()
    }
}

impl<T: AsRef<str> + ?Sized, R: TypeRegex + ?Sized> Predicate<T> for Matches<R> {
    fn check(value: &T) -> bool {
        Self::regex().is_match(value.as_ref())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string matching `{pattern}` pattern",
            pattern = Self::pattern()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(MATCHES)
    }
}
