//! Type-level regular expressions.

pub use regex::Regex;

#[doc(hidden)]
pub mod import {
    pub use core::marker::PhantomData;

    pub use std::sync::LazyLock;
}

/// Represents static regular expressions (as returned in [`get`] of [`TypeRegex`]).
///
/// [`get`]: TypeRegex::get
pub type StaticRegex = &'static Regex;

/// Represents type-level regular expressions.
pub trait TypeRegex {
    /// Returns the compiled regular expression.
    #[must_use]
    fn get() -> StaticRegex;
}

/// The `invalid regex` literal.
pub const INVALID: &str = "invalid regex";

/// Lifts strings to type-level regular expressions.
///
/// ```
/// use refining_regex::type_regex;
///
/// type_regex!(pub Natural = "^0|[1-9][0-9]*$" => "Matches non-negative integers.");
/// ```
///
/// Is essentially equivalent to:
///
/// ```
/// use std::{marker::PhantomData, sync::LazyLock};
///
/// use refining_regex::types::{Regex, StaticRegex, TypeRegex, INVALID};
///
/// /// Matches non-negative integers.
/// pub struct Natural {
///     private: PhantomData<()>,
/// }
///
/// impl TypeRegex for Natural {
///     fn get() -> StaticRegex {
///         static REGEX: LazyLock<Regex> = LazyLock::new(|| {
///             Regex::new("^0|[1-9][0-9]*$").expect(INVALID)
///         });
///
///         LazyLock::force(&REGEX)
///     }
/// }
/// ```
#[macro_export]
macro_rules! type_regex {
    ($vis: vis $name: ident = $regex: expr $(=> $doc: expr)?) => {
        $(
            #[doc = $doc]
        )?
        $vis struct $name {
            private: $crate::types::import::PhantomData<()>,
        }

        impl $crate::types::TypeRegex for $name {
            fn get() -> $crate::types::StaticRegex {
                use $crate::types::import::LazyLock;

                static REGEX: LazyLock<$crate::types::Regex> = LazyLock::new(|| {
                    $crate::types::Regex::new($regex).expect($crate::types::INVALID)
                });

                LazyLock::force(&REGEX)
            }
        }
    };
}
