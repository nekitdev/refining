//! Predicates for strings.

use core::{fmt, marker::PhantomData};

use refining_core::{
    predicate::Predicate,
    types::{StaticStr, TypeStr},
};

/// Checks if the string starts with the specified prefix `P`.
pub struct StartsWith<P: TypeStr + ?Sized> {
    prefix: PhantomData<P>,
}

impl<P: TypeStr + ?Sized> StartsWith<P> {
    /// Returns the expected prefix.
    #[must_use]
    pub const fn prefix() -> StaticStr {
        P::VALUE
    }
}

pub const STARTS_WITH: &str = "str::starts_with";

impl<T: AsRef<str> + ?Sized, P: TypeStr + ?Sized> Predicate<T> for StartsWith<P> {
    fn check(value: &T) -> bool {
        value.as_ref().starts_with(Self::prefix())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string starting with `{prefix}`",
            prefix = Self::prefix()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(STARTS_WITH)
    }
}

/// Checks if the string ends with the specified suffix `S`.
pub struct EndsWith<S: TypeStr + ?Sized> {
    suffix: PhantomData<S>,
}

impl<S: TypeStr + ?Sized> EndsWith<S> {
    /// Returns the expected suffix.
    #[must_use]
    pub const fn suffix() -> StaticStr {
        S::VALUE
    }
}

pub const ENDS_WITH: &str = "str::ends_with";

impl<T: AsRef<str> + ?Sized, S: TypeStr + ?Sized> Predicate<T> for EndsWith<S> {
    fn check(value: &T) -> bool {
        value.as_ref().ends_with(Self::suffix())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string ending with `{suffix}`",
            suffix = Self::suffix()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ENDS_WITH)
    }
}

/// Checks if the string contains the specified string `S`.
pub struct Contains<S: TypeStr + ?Sized> {
    string: PhantomData<S>,
}

impl<S: TypeStr + ?Sized> Contains<S> {
    /// Returns the expected string.
    #[must_use]
    pub const fn string() -> StaticStr {
        S::VALUE
    }
}

pub const CONTAINS: &str = "str::contains";

impl<T: AsRef<str> + ?Sized, S: TypeStr + ?Sized> Predicate<T> for Contains<S> {
    fn check(value: &T) -> bool {
        value.as_ref().contains(Self::string())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string containing `{string}`",
            string = Self::string()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(CONTAINS)
    }
}

/// Checks if the string starts with the specified character `C`.
pub struct StartsWithChar<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> StartsWithChar<C> {
    /// Returns the expected character.
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for StartsWithChar<C> {
    fn check(value: &T) -> bool {
        value.as_ref().starts_with(Self::character())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string starting with `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "str::starts_with<{character}>",
            character = Self::character()
        )
    }
}

/// Checks if the string ends with the specified character `C`.
pub struct EndsWithChar<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> EndsWithChar<C> {
    /// Returns the expected character.
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for EndsWithChar<C> {
    fn check(value: &T) -> bool {
        value.as_ref().ends_with(Self::character())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string ending with `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "str::ends_with<{character}>",
            character = Self::character()
        )
    }
}

/// Checks if the string contains the specified character `C`.
pub struct ContainsChar<const C: char> {
    private: PhantomData<()>,
}

impl<const C: char> ContainsChar<C> {
    /// Returns the expected character.
    #[must_use]
    pub const fn character() -> char {
        C
    }
}

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for ContainsChar<C> {
    fn check(value: &T) -> bool {
        value.as_ref().contains(Self::character())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string containing `{character}`",
            character = Self::character()
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "str::contains<{character}>",
            character = Self::character()
        )
    }
}

/// Checks if the string is trimmed at the start.
pub struct TrimmedStart {
    private: PhantomData<()>,
}

pub const TRIMMED_START_MESSAGE: &str = "string trimmed at the start";
pub const TRIMMED_START: &str = "str::trimmed_start";

impl<T: AsRef<str> + ?Sized> Predicate<T> for TrimmedStart {
    fn check(value: &T) -> bool {
        let string = value.as_ref();

        string.trim_start() == string
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED_START_MESSAGE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED_START)
    }
}

/// Checks if the string is trimmed at the end.
pub struct TrimmedEnd {
    private: PhantomData<()>,
}

pub const TRIMMED_END_MESSAGE: &str = "string trimmed at the end";
pub const TRIMMED_END: &str = "str::trimmed_end";

impl<T: AsRef<str> + ?Sized> Predicate<T> for TrimmedEnd {
    fn check(value: &T) -> bool {
        let string = value.as_ref();

        string.trim_end() == string
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED_END_MESSAGE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED_END)
    }
}

/// Checks if the string is trimmed.
pub struct Trimmed {
    private: PhantomData<()>,
}

pub const TRIMMED_MESSAGE: &str = "trimmed string";
pub const TRIMMED: &str = "str::trimmed";

impl<T: AsRef<str> + ?Sized> Predicate<T> for Trimmed {
    fn check(value: &T) -> bool {
        let string = value.as_ref();

        string.trim() == string
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED_MESSAGE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRIMMED)
    }
}

/// Checks if the string is valid ASCII.
pub struct Ascii {
    private: PhantomData<()>,
}

pub const ASCII_MESSAGE: &str = "ascii string";
pub const ASCII: &str = "str::ascii";

impl<T: AsRef<str> + ?Sized> Predicate<T> for Ascii {
    fn check(value: &T) -> bool {
        value.as_ref().is_ascii()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ASCII_MESSAGE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ASCII)
    }
}
