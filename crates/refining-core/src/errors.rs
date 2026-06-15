//! Errors returned during refining.

use core::{fmt, marker::PhantomData, mem::transmute};

use thiserror::Error;

use crate::{
    context::NoContext,
    predicate::{Predicate, PredicateExpected},
    types::TypeStr,
};

/// The `Error` literal.
pub const ERROR: &str = "Error";

/// The `expected` literal.
pub const EXPECTED: &str = "expected";

/// The `context` literal.
pub const CONTEXT: &str = "context";

/// Represents errors that occur when the value of type `T` does not satisfy the predicate `P`.
///
/// The context `C` provides additional information about the error, and defaults to `NoContext`.
///
/// For instance, [`False`] expects `nothing` and its code is `false`, so the error message is:
///
/// ```text
/// expected nothing (no context) [false]
/// ```
///
/// [`False`]: crate::logical::False
#[derive(Error)]
#[error(
    "{EXPECTED} {expected} ({context}) [{expected:?}]",
    expected = P::expected(),
    context = C::VALUE
)]
#[repr(transparent)]
pub struct Error<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized = NoContext> {
    predicate: PhantomData<P>,
    context: PhantomData<C>,
    value: T,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    // XXX: do not expose this function
    pub(crate) const fn new_ref(value: &T) -> &Self {
        // SAFETY: layouts of `T` and `Self` are the same, so this is safe
        unsafe { transmute(value) }
    }

    /// Returns the contained value reference.
    pub const fn get_ref(&self) -> &T {
        &self.value
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    // XXX: do not expose this function
    pub(crate) const fn new(value: T) -> Self {
        Self {
            predicate: PhantomData,
            context: PhantomData,
            value,
        }
    }

    /// Returns the contained value.
    pub fn get(self) -> T {
        self.value
    }
}

/// Represents recoverable reference results.
pub type RecoverableRef<'a, T, P, C = NoContext, R = T> = Result<&'a T, &'a Error<R, P, C>>;

/// Represents recoverable results.
pub type Recoverable<T, P, C = NoContext, R = T> = Result<T, Error<R, P, C>>;

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Debug for Error<T, P, C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expected = P::expected();
        let context = C::VALUE;

        formatter
            .debug_struct(ERROR)
            .field(EXPECTED, &expected)
            .field(CONTEXT, &context)
            .finish_non_exhaustive()
    }
}
