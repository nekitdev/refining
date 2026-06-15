pub mod import {
    pub use core::{fmt, marker::PhantomData};
}

macro_rules! human {
    (<) => {
        "less than"
    };
    (<=) => {
        "less than or equal to"
    };
    (>) => {
        "greater than"
    };
    (>=) => {
        "greater than or equal to"
    };
    (==) => {
        "equal to"
    };
    (!=) => {
        "not equal to"
    };
}

pub(crate) use human;

macro_rules! reference {
    ($int: ty) => {
        concat!("[`prim@", stringify!($int), "`]")
    };
}

pub(crate) use reference;

macro_rules! compare {
    ($int: ty => $name: ident [$code: ident] ($operation: tt)) => {
        #[doc = concat!(
            "Checks whether the given value is ",
            $crate::internals::human!($operation),
            " `N`."
        )]
        pub struct $name<const N: $int> {
            private: $crate::internals::import::PhantomData<()>,
        }

        impl<const N: $int> refining_core::predicate::Predicate<$int> for $name<N> {
            fn check(value: &$int) -> bool {
                Self::check_value(*value)
            }

            fn check_value(value: $int) -> bool {
                value $operation N
            }

            fn expect(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>
            ) -> $crate::internals::import::fmt::Result {
                write!(
                    formatter,
                    "{int} {operation} {N}",
                    int = stringify!($int),
                    operation = stringify!($operation)
                )
            }

            fn expect_code(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>
            ) -> $crate::internals::import::fmt::Result {
                write!(
                    formatter,
                    "{int}::{code}<{N}>",
                    int = stringify!($int),
                    code = stringify!($code)
                )
            }
        }
    };
}

pub(crate) use compare;

macro_rules! less {
    ($int: ty) => {
        $crate::internals::compare!($int => Less [lt] (<));
    }
}

pub(crate) use less;

macro_rules! less_or_equal {
    ($int: ty) => {
        $crate::internals::compare!($int => LessOrEqual [le] (<=));
    }
}

pub(crate) use less_or_equal;

macro_rules! greater {
    ($int: ty) => {
        $crate::internals::compare!($int => Greater [gt] (>));
    }
}

pub(crate) use greater;

macro_rules! greater_or_equal {
    ($int: ty) => {
        $crate::internals::compare!($int => GreaterOrEqual [ge] (>=));
    }
}

pub(crate) use greater_or_equal;

macro_rules! equal {
    ($int: ty) => {
        $crate::internals::compare!($int => Equal [eq] (==));
    }
}

pub(crate) use equal;

macro_rules! not_equal {
    ($int: ty) => {
        $crate::internals::compare!($int => NotEqual [ne] (!=));
    }
}

pub(crate) use not_equal;

macro_rules! comparing {
    ($int: ty) => {
        $crate::internals::less!($int);
        $crate::internals::less_or_equal!($int);
        $crate::internals::greater!($int);
        $crate::internals::greater_or_equal!($int);
        $crate::internals::equal!($int);
        $crate::internals::not_equal!($int);
    };
}

pub(crate) use comparing;

macro_rules! interval {
    (
        $int: ty => $name: ident<
            $left: ident as $open: literal, $right: ident as $close: literal
        >
    ) => {
        #[doc = concat!("Represents `", $open, "M, N", $close, "` intervals.")]
        pub type $name<const M: $int, const N: $int> =
            refining_core::logical::And<$left<M>, $right<N>>;
    };
}

pub(crate) use interval;

macro_rules! open {
    ($int: ty) => {
        $crate::internals::interval!($int => Open<Greater as "(", Less as ")">);
    };
}

pub(crate) use open;

macro_rules! open_closed {
    ($int: ty) => {
        $crate::internals::interval!($int => OpenClosed<Greater as "(", LessOrEqual as "]">);
    };
}

pub(crate) use open_closed;

macro_rules! closed_open {
    ($int: ty) => {
        $crate::internals::interval!($int => ClosedOpen<GreaterOrEqual as "[", Less as ")">);
    };
}

pub(crate) use closed_open;

macro_rules! closed {
    ($int: ty) => {
        $crate::internals::interval!($int => Closed<GreaterOrEqual as "[", LessOrEqual as "]">);
    };
}

pub(crate) use closed;

macro_rules! intervals {
    ($int: ty) => {
        $crate::internals::open!($int);
        $crate::internals::open_closed!($int);
        $crate::internals::closed_open!($int);
        $crate::internals::closed!($int);
    };
}

pub(crate) use intervals;

macro_rules! zeros {
    ($int: ty) => {
        /// Represents zero (`0`).
        pub const ZERO: $int = 0;

        /// Checks whether the given value is equal to zero (`0`).
        pub type Zero = Equal<ZERO>;

        /// Checks whether the given value is not equal to zero (`0`).
        pub type NonZero = NotEqual<ZERO>;
    };
}

pub(crate) use zeros;

macro_rules! modulo {
    ($int: ty) => {
        #[doc = concat!(
            "Checks whether ", $crate::internals::reference!($int), " divided by `D` has modulo `M`."
        )]
        pub struct Modulo<const D: $int, const M: $int> {
            private: $crate::internals::import::PhantomData<()>,
        }

        impl<const D: $int, const M: $int> refining_core::predicate::Predicate<$int> for Modulo<D, M> {
            fn check(value: &$int) -> bool {
                *value % D == M
            }

            fn expect(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
            ) -> $crate::internals::import::fmt::Result {
                write!(formatter, "{int} % {D} == {M}", int = stringify!($int))
            }

            fn expect_code(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
            ) -> $crate::internals::import::fmt::Result {
                write!(formatter, "{int}::mod<{D}, {M}>", int = stringify!($int))
            }
        }
    };
}

pub(crate) use modulo;

macro_rules! divisible {
    ($int: ty) => {
        /// Represents two (`2`).
        pub const TWO: $int = 0;

        /// Checks whether the given value is divisible by `D`.
        pub type Divisible<const D: $int> = Modulo<D, ZERO>;

        /// Checks whether the given value is even.
        pub type Even = Divisible<TWO>;

        /// Checks whether the given value is odd.
        pub type Odd = refining_core::logical::Not<Even>;
    };
}

pub(crate) use divisible;

macro_rules! common {
    ($int: ty) => {
        $crate::internals::comparing!($int);
        $crate::internals::intervals!($int);
        $crate::internals::zeros!($int);
        $crate::internals::modulo!($int);
        $crate::internals::divisible!($int);
    };
}

pub(crate) use common;

macro_rules! unsigned {
    ($int: ty) => {
        $crate::internals::common!($int);
    };
}

pub(crate) use unsigned;

macro_rules! around {
    ($int: ty) => {
        /// Checks whether the given value is positive.
        pub type Positive = Greater<ZERO>;

        /// Checks whether the given value is negative.
        pub type Negative = Less<ZERO>;

        /// Checks whether the given value is non-negative (positive or zero).
        pub type NonNegative = GreaterOrEqual<ZERO>;

        /// Checks whether the given value is non-positive (negative or zero).
        pub type NonPositive = LessOrEqual<ZERO>;
    };
}

pub(crate) use around;

macro_rules! signed {
    ($int: ty) => {
        $crate::internals::common!($int);
        $crate::internals::around!($int);
    };
}

pub(crate) use signed;

macro_rules! unsigned_mod {
    ($int: ty => $name: ident) => {
        #[doc = concat!("Predicates for ", $crate::internals::reference!($int), " values.")]
        pub mod $name {
            $crate::internals::unsigned!($int);
        }
    };
}

pub(crate) use unsigned_mod;

macro_rules! signed_mod {
    ($int: ty => $name: ident) => {
        #[doc = concat!("Predicates for ", $crate::internals::reference!($int), " values.")]
        pub mod $name {
            $crate::internals::signed!($int);
        }
    };
}

pub(crate) use signed_mod;

macro_rules! predicate {
    (
        Name = $name: ident,
        Method = $method: ident,
        Doc = $doc: expr,
        Expected = $expected: expr,
        Code = $code: path,
    ) => {
        #[doc = $doc]
        pub struct $name {
            private: $crate::internals::import::PhantomData<()>,
        }

        impl refining_core::predicate::Predicate<u8> for $name {
            fn check(value: &u8) -> bool {
                value.$method()
            }

            fn expect(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
            ) -> $crate::internals::import::fmt::Result {
                write!(formatter, $expected)
            }

            fn expect_code(
                formatter: &mut $crate::internals::import::fmt::Formatter<'_>,
            ) -> $crate::internals::import::fmt::Result {
                write!(formatter, stringify!($code))
            }
        }
    };
}

pub(crate) use predicate;
