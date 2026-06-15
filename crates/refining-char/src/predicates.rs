use core::fmt;
use std::marker::PhantomData;

use refining_core::{logical::And, predicate::Predicate};

pub struct CharEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharEqual<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharEqual<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value == Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character == `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::eq<{character}>",
            character = Self::character()
        )
    }
}

pub struct CharNotEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharNotEqual<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharNotEqual<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value != Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character != `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::ne<{character}>",
            character = Self::character()
        )
    }
}

pub struct CharLess<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharLess<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharLess<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value < Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character < `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::lt<{character}>",
            character = Self::character()
        )
    }
}

pub struct CharGreater<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharGreater<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharGreater<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value > Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character > `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::gt<{character}>",
            character = Self::character()
        )
    }
}

pub struct CharLessOrEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharLessOrEqual<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharLessOrEqual<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value <= Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character <= `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::le<{character}>",
            character = Self::character()
        )
    }
}

pub struct CharGreaterOrEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharGreaterOrEqual<C> {
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<const C: char> Predicate<char> for CharGreaterOrEqual<C> {
    fn check(value: &char) -> bool {
        Self::check_value(*value)
    }

    fn check_value(value: char) -> bool {
        value >= Self::character()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character >= `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "char::ge<{character}>",
            character = Self::character()
        )
    }
}

/// Represents `(C, D)` intervals.
pub type CharOpen<const C: char, const D: char> = And<CharGreater<C>, CharLess<D>>;

/// Represents `[C, D)` intervals.
pub type CharClosedOpen<const C: char, const D: char> = And<CharGreaterOrEqual<C>, CharLess<D>>;

/// Represents `(C, D]` intervals.
pub type CharOpenClosed<const C: char, const D: char> = And<CharGreater<C>, CharLessOrEqual<D>>;

/// Represents `[C, D]` intervals.
pub type CharClosed<const C: char, const D: char> = And<CharGreaterOrEqual<C>, CharLessOrEqual<D>>;

/// Represents the null character.
pub const NULL: char = '\0';

pub type CharNull = CharEqual<NULL>;

pub type CharNonNull = CharNotEqual<NULL>;
