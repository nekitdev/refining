pub mod import {
    pub use core::{fmt, marker::PhantomData};
}

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

        impl refining_core::predicate::Predicate<char> for $name {
            fn check(value: &char) -> bool {
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
