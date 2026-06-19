//! Unicode character predicates.

use crate::internals::predicate;

predicate! {
    Name = CharAscii,
    Method = is_ascii,
    Doc = "Checks whether the given character is within the ASCII range.",
    Expected = "ascii character",
    Code = char::ascii,
}

predicate! {
    Name = CharAlphabetic,
    Method = is_alphabetic,
    Doc = "Checks whether the given character is alphabetic.",
    Expected = "alphabetic character",
    Code = char::alphabetic,
}

predicate! {
    Name = CharAlphanumeric,
    Method = is_alphanumeric,
    Doc = "Checks whether the given character is alphanumeric.",
    Expected = "alphanumeric character",
    Code = char::alphanumeric,
}

predicate! {
    Name = CharControl,
    Method = is_control,
    Doc = "Checks whether the given character is control.",
    Expected = "control character",
    Code = char::control,
}

predicate! {
    Name = CharNumeric,
    Method = is_numeric,
    Doc = "Checks whether the given character is numeric.",
    Expected = "numeric character",
    Code = char::numeric,
}

predicate! {
    Name = CharLowercase,
    Method = is_lowercase,
    Doc = "Checks whether the given character is lowercase.",
    Expected = "lowercase character",
    Code = char::lowercase,
}

predicate! {
    Name = CharUppercase,
    Method = is_uppercase,
    Doc = "Checks whether the given character is uppercase.",
    Expected = "uppercase character",
    Code = char::uppercase,
}

predicate! {
    Name = CharWhitespace,
    Method = is_whitespace,
    Doc = "Checks whether the given character is whitespace.",
    Expected = "whitespace character",
    Code = char::whitespace,
}
