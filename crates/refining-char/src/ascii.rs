//! ASCII character predicates.

use core::{fmt, marker::PhantomData};

use refining_core::predicate::Predicate;

use crate::internals::predicate;

/// Represents integer base for checks.
pub type Base = u32;

/// The octal base.
pub const OCTAL: Base = 8;

/// The decimal base.
pub const DECIMAL: Base = 10;

/// The hexadecimal base.
pub const HEXADECIMAL: Base = 16;

/// Checks whether the given character is a digit in the specified base `B`.
///
/// The default base is [`DECIMAL`].
pub struct CharDigit<const B: Base = DECIMAL> {
    private: PhantomData<()>,
}

/// [`CharDigit<B>`] with `B` set to [`DECIMAL`].
pub type CharDecDigit = CharDigit<DECIMAL>;

/// [`CharDigit<B>`] with `B` set to [`OCTAL`].
pub type CharOctDigit = CharDigit<OCTAL>;

/// [`CharDigit<B>`] with `B` set to [`HEXADECIMAL`].
pub type CharHexDigit = CharDigit<HEXADECIMAL>;

impl<const B: Base> CharDigit<B> {
    /// Returns the base `B` for which this predicate checks.
    #[must_use]
    pub const fn base() -> Base {
        B
    }
}

impl<const B: Base> Predicate<char> for CharDigit<B> {
    fn check(value: &char) -> bool {
        value.is_digit(Self::base())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "digit in base `{base}`", base = Self::base())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "char::digit<{base}>", base = Self::base())
    }
}

predicate! {
    Name = CharAsciiAlphabetic,
    Method = is_ascii_alphabetic,
    Doc = "Checks whether the given character is ASCII alphabetic.",
    Expected = "ascii alphabetic character",
    Code = char::ascii::alphabetic,
}

predicate! {
    Name = CharAsciiAlphanumeric,
    Method = is_ascii_alphanumeric,
    Doc = "Checks whether the given character is ASCII alphanumeric.",
    Expected = "ascii alphanumeric character",
    Code = char::ascii::alphanumeric,
}

predicate! {
    Name = CharAsciiControl,
    Method = is_ascii_control,
    Doc = "Checks whether the given character is ASCII control.",
    Expected = "ascii control character",
    Code = char::ascii::control,
}

predicate! {
    Name = CharAsciiGraphic,
    Method = is_ascii_graphic,
    Doc = "Checks whether the given character is ASCII graphic.",
    Expected = "ascii graphic character",
    Code = char::ascii::graphic,
}

predicate! {
    Name = CharAsciiPunctuation,
    Method = is_ascii_punctuation,
    Doc = "Checks whether the given character is ASCII punctuation.",
    Expected = "ascii punctuation character",
    Code = char::ascii::punctuation,
}

predicate! {
    Name = CharAsciiLowercase,
    Method = is_ascii_lowercase,
    Doc = "Checks whether the given character is ASCII lowercase.",
    Expected = "ascii lowercase character",
    Code = char::ascii::lowercase,
}

predicate! {
    Name = CharAsciiUppercase,
    Method = is_ascii_uppercase,
    Doc = "Checks whether the given character is ASCII uppercase.",
    Expected = "ascii uppercase character",
    Code = char::ascii::uppercase,
}

predicate! {
    Name = CharAsciiWhitespace,
    Method = is_ascii_whitespace,
    Doc = "Checks whether the given character is ASCII whitespace.",
    Expected = "ascii whitespace character",
    Code = char::ascii::whitespace,
}
