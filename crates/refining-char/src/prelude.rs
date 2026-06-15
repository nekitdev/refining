#[doc(inline)]
pub use crate::{
    ascii::{
        CharAsciiAlphabetic, CharAsciiAlphanumeric, CharAsciiControl, CharAsciiGraphic,
        CharAsciiLowercase, CharAsciiPunctuation, CharAsciiUppercase, CharAsciiWhitespace,
        CharDecDigit, CharDigit, CharHexDigit, CharOctDigit,
    },
    predicates::{
        CharClosed, CharClosedOpen, CharEqual, CharGreater, CharGreaterOrEqual, CharLess,
        CharLessOrEqual, CharNonNull, CharNotEqual, CharNull, CharOpen, CharOpenClosed,
    },
    unicode::{
        CharAlphabetic, CharAlphanumeric, CharAscii, CharControl, CharLowercase, CharNumeric,
        CharUppercase, CharWhitespace,
    },
};
