//! Type-level strings.

#[doc(hidden)]
pub mod import {
    pub use core::marker::PhantomData;
}

/// Represents static strings (as in the [`VALUE`] of [`TypeStr`]).
///
/// [`VALUE`]: TypeStr::VALUE
pub type StaticStr = &'static str;

/// Represents type-level strings.
pub trait TypeStr {
    /// The string value.
    const VALUE: StaticStr;
}

/// Lifts static strings to type-level strings.
///
/// # Examples
///
/// ```
/// use refining_core::type_str;
///
/// type_str!(pub HelloWorld = "Hello, world!" => "The classic greeting.");
/// ```
///
/// Is equivalent to:
///
/// ```
/// use core::marker::PhantomData;
///
/// use refining_core::types::{StaticStr, TypeStr};
///
/// /// The classic greeting.
/// pub struct HelloWorld {
///     private: PhantomData<()>,
/// }
///
/// impl TypeStr for HelloWorld {
///     const VALUE: StaticStr = "Hello, world!";
/// }
/// ```
#[macro_export]
macro_rules! type_str {
    ($visibility: vis $name: ident = $value: expr $(=> $documentation: expr)?) => {
        $(
            #[doc = $documentation]
        )?
        $visibility struct $name {
            private: $crate::types::import::PhantomData<()>,
        }

        impl $crate::types::TypeStr for $name {
            const VALUE: $crate::types::StaticStr = $value;
        }
    };
}
