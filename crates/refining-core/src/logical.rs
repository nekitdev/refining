//! Logical composition.

use core::{fmt, marker::PhantomData};

use crate::predicate::{Predicate, PredicateExpected};

/// The `nothing` literal.
pub const NOTHING: &str = "nothing";

/// The `everything` literal.
pub const ANYTHING: &str = "anything";

/// The `false` literal.
pub const FALSE: &str = "false";

/// The `true` literal.
pub const TRUE: &str = "true";

/// Represents predicates that are never satisfied.
pub struct False {
    private: PhantomData<()>,
}

/// Represents predicates that are always satisfied.
pub struct True {
    private: PhantomData<()>,
}

impl<T: ?Sized> Predicate<T> for False {
    fn check(_value: &T) -> bool {
        false
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(NOTHING)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(FALSE)
    }
}

impl<T: ?Sized> Predicate<T> for True {
    fn check(_value: &T) -> bool {
        true
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ANYTHING)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRUE)
    }
}

/// The `not` literal.
pub const NOT: &str = "not";

/// Represents predicates that are satisfied whenever `P` is *not* satisfied.
pub struct Not<P: ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Predicate<T> for Not<P> {
    fn check(value: &T) -> bool {
        !P::check(value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{NOT} ({expected})", expected = P::expected())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{NOT}<{expected:?}>", expected = P::expected())
    }
}

/// The `and` literal.
pub const AND: &str = "and";

/// Represents predicates that are satisfied when both `P` *and* `Q` are satisfied.
pub struct And<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for And<P, Q> {
    fn check(value: &T) -> bool {
        P::check(value) && Q::check(value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({left}) {AND} ({right})",
            left = P::expected(),
            right = Q::expected(),
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{AND}<{left:?}, {right:?}>",
            left = P::expected(),
            right = Q::expected(),
        )
    }
}

/// The `or` literal.
pub const OR: &str = "or";

/// Represents predicates that are satisfied whenever at least `P` *or* `Q` are satisfied.
pub struct Or<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for Or<P, Q> {
    fn check(value: &T) -> bool {
        P::check(value) || Q::check(value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({left}) {OR} ({right})",
            left = P::expected(),
            right = Q::expected(),
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{OR}<{left:?}, {right:?}>",
            left = P::expected(),
            right = Q::expected(),
        )
    }
}

/// The `xor` literal.
pub const XOR: &str = "xor";

/// Represents predicates that are satisfied whenever `P` *or* `Q` (but *not* both) are satisfied.
pub struct Xor<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for Xor<P, Q> {
    fn check(value: &T) -> bool {
        P::check(value) ^ Q::check(value)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({left}) {XOR} ({right})",
            left = P::expected(),
            right = Q::expected(),
        )
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{XOR}<{left:?}, {right:?}>",
            left = P::expected(),
            right = Q::expected(),
        )
    }
}

/// Represents [`Not<And<P, Q>>`].
pub type Nand<P, Q> = Not<And<P, Q>>;

/// Represents [`Not<Or<P, Q>>`].
pub type Nor<P, Q> = Not<Or<P, Q>>;

/// Represents [`Not<Xor<P, Q>>`].
pub type Xnor<P, Q> = Not<Xor<P, Q>>;

/// Represents logical implication `P -> Q`, that is, [`Or<Not<P>, Q>`].
pub type Implies<P, Q> = Or<Not<P>, Q>;

/// Negates the given predicate.
///
/// For predicate `P`, `not!(P)` is [`Not<P>`].
#[macro_export]
macro_rules! not {
    ($predicate: ty) => {
        $crate::logical::Not<$predicate>
    };
}

/// Given two or more predicates, composes them together with [`And`].
///
/// For predicates `P` and `Q`, `and!(P, Q)` is [`And<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `and!(P, Q, R)` is [`And<P, And<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! and {
    ($left: ty, $right: ty) => {
        $crate::logical::And<$left, $right>
    };
    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::and!($first, $crate::and!($second, $($rest),+))
    };
}

/// Given two or more predicates, composes them together with [`Or`].
///
/// For predicates `P` and `Q`, `or!(P, Q)` is [`Or<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `or!(P, Q, R)` is [`Or<P, Or<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! or {
    ($left: ty, $right: ty) => {
        $crate::logical::Or<$left, $right>
    };
    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::or!($first, $crate::or!($second, $($rest),+))
    };
}

/// Given two or more predicates, composes them together with [`Xor`].
///
/// For predicates `P` and `Q`, `xor!(P, Q)` is [`Xor<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `xor!(P, Q, R)` is [`Xor<P, Xor<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! xor {
    ($left: ty, $right: ty) => {
        $crate::logical::Xor<$left, $right>
    };
    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::xor!($first, $crate::xor!($second, $($rest),+))
    };
}
