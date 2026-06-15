//! Predicates for bytes.

use core::{fmt, marker::PhantomData};

use refining_core::{logical::Not, predicate::Predicate};

use crate::{internals::predicate, predicates::u8};

/// Represents the null byte.
pub const NULL: u8 = b'\0';

/// Checks whether the byte is [`NULL`].
pub struct ByteNull {
    private: PhantomData<()>,
}

/// The `null byte` literal.
pub const NULL_MESSAGE: &str = "null byte";

/// The `byte::null` literal.
pub const NULL_CODE: &str = "byte::null";

impl Predicate<u8> for ByteNull {
    fn check(value: &u8) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: u8) -> bool {
        value == NULL
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(NULL_MESSAGE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(NULL_CODE)
    }
}

/// Checks whether the byte is non-[`NULL`].
pub type ByteNonNull = Not<ByteNull>;

predicate! {
    Name = ByteAscii,
    Method = is_ascii,
    Doc = "Checks whether the given byte is within the ASCII range.",
    Expected = "ascii byte",
    Code = byte::ascii,
}

predicate! {
    Name = ByteAsciiAlphabetic,
    Method = is_ascii_alphabetic,
    Doc = "Checks whether the given byte is an ASCII alphabetic character.",
    Expected = "ascii alphabetic byte",
    Code = byte::ascii::alphabetic,
}

predicate! {
    Name = ByteAsciiAlphanumeric,
    Method = is_ascii_alphanumeric,
    Doc = "Checks whether the given byte is an ASCII alphanumeric character.",
    Expected = "ascii alphanumeric byte",
    Code = byte::ascii::alphanumeric,
}

predicate! {
    Name = ByteAsciiControl,
    Method = is_ascii_control,
    Doc = "Checks whether the given byte is an ASCII control character.",
    Expected = "ascii control byte",
    Code = byte::ascii::control,
}

predicate! {
    Name = ByteAsciiDigit,
    Method = is_ascii_digit,
    Doc = "Checks whether the given byte is an ASCII digit.",
    Expected = "ascii digit byte",
    Code = byte::ascii::digit,
}

predicate! {
    Name = ByteAsciiGraphic,
    Method = is_ascii_graphic,
    Doc = "Checks whether the given byte is an ASCII graphic character.",
    Expected = "ascii graphic byte",
    Code = byte::ascii::graphic,
}

predicate! {
    Name = ByteAsciiLowercase,
    Method = is_ascii_lowercase,
    Doc = "Checks whether the given byte is an ASCII lowercase character.",
    Expected = "ascii lowercase byte",
    Code = byte::ascii::lowercase,
}

predicate! {
    Name = ByteAsciiPunctuation,
    Method = is_ascii_punctuation,
    Doc = "Checks whether the given byte is an ASCII punctuation character.",
    Expected = "ascii punctuation byte",
    Code = byte::ascii::punctuation,
}

predicate! {
    Name = ByteAsciiUppercase,
    Method = is_ascii_uppercase,
    Doc = "Checks whether the given byte is an ASCII uppercase character.",
    Expected = "ascii uppercase byte",
    Code = byte::ascii::uppercase,
}

predicate! {
    Name = ByteAsciiWhitespace,
    Method = is_ascii_whitespace,
    Doc = "Checks whether the given byte is an ASCII whitespace character.",
    Expected = "ascii whitespace byte",
    Code = byte::ascii::whitespace,
}
