//! Character predicates.

use core::{fmt, marker::PhantomData};

use refining_core::{logical::And, predicate::Predicate};

/// Checks whether the character is equal to `C`.
pub struct CharEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharEqual<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is not equal to `C`.
pub struct CharNotEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharNotEqual<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is less than `C`.
pub struct CharLess<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharLess<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is greater than `C`.
pub struct CharGreater<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharGreater<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is less than or equal to `C`.
pub struct CharLessOrEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharLessOrEqual<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is greater than or equal to `C`.
pub struct CharGreaterOrEqual<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> CharGreaterOrEqual<C> {
    /// Returns the character `C` for which this predicate checks.
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

/// Checks whether the character is [`NULL`].
pub type CharNull = CharEqual<NULL>;

/// Checks whether the character is non-[`NULL`].
pub type CharNonNull = CharNotEqual<NULL>;
