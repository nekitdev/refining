//! Refinement types.

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::Deref,
};

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use crate::{
    context::NoContext,
    errors::{Error, Recoverable, RecoverableRef},
    predicate::Predicate,
    types::TypeStr,
};

/// Represents refinement types.
#[repr(transparent)]
pub struct Refinement<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized = NoContext> {
    predicate: PhantomData<P>,
    context: PhantomData<C>,
    value: T,
}

impl<T: Clone, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Clone for Refinement<T, P, C> {
    fn clone(&self) -> Self {
        // SAFETY: `clone` does not change the value, so the predicate is still satisfied
        unsafe { Self::unchecked(self.get_ref().clone()) }
    }
}

impl<T: Copy, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Copy for Refinement<T, P, C> {}

impl<T: fmt::Debug + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Debug
    for Refinement<T, P, C>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get_ref().fmt(formatter)
    }
}

impl<T: fmt::Display + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Display
    for Refinement<T, P, C>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get_ref().fmt(formatter)
    }
}

impl<T: PartialEq + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> PartialEq
    for Refinement<T, P, C>
{
    fn eq(&self, other: &Self) -> bool {
        self.get_ref().eq(other.get_ref())
    }
}

impl<T: Eq + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Eq for Refinement<T, P, C> {}

impl<T: PartialOrd + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> PartialOrd
    for Refinement<T, P, C>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_ref().partial_cmp(other.get_ref())
    }
}

impl<T: Ord + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Ord for Refinement<T, P, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_ref().cmp(other.get_ref())
    }
}

impl<T: Hash + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Hash for Refinement<T, P, C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ref().hash(state);
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> AsRef<T> for Refinement<T, P, C> {
    fn as_ref(&self) -> &T {
        self.get_ref()
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Deref for Refinement<T, P, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Represents recoverable refining results.
pub type RecoverableRefinement<R> =
    Recoverable<R, <R as Refining>::Predicate, <R as Refining>::Context, <R as Refining>::Value>;

/// Represents recoverable reference refining results.
pub type RecoverableRefinementRef<'a, R> = RecoverableRef<
    'a,
    R,
    <R as Refining>::Predicate,
    <R as Refining>::Context,
    <R as Refining>::Value,
>;

/// Refinement methods.
///
/// This sealed trait is used for generics over [`Refinement<T, P, C>`].
pub trait Refining: Deref<Target = Self::Value> + sealed::Sealed {
    /// The refinement value (`T` in [`Refinement<T, P, C>`]).
    type Value: ?Sized;

    /// The refinement predicate (`P` in [`Refinement<T, P, C>`]).
    type Predicate: Predicate<Self::Value> + ?Sized;

    /// The refinement context (`C` in [`Refinement<T, P, C>`]).
    type Context: TypeStr + ?Sized;

    /// Constructs [`Self`] from [`Self::Value`] by reference without checking.
    ///
    /// # Safety
    ///
    /// This method should only be called for references that satisfy [`Self::Predicate`].
    ///
    /// Alternatively, this can be checked using the [`is_fine`] method.
    ///
    /// [`is_fine`]: Self::is_fine
    unsafe fn unchecked_ref(value: &Self::Value) -> &Self;

    /// Constructs [`Self`] from [`Self::Value`] without checking.
    ///
    /// # Safety
    ///
    /// This method should only be called for values that satisfy [`Self::Predicate`].
    ///
    /// Alternatively, this can be checked using the [`is_fine`] method.
    ///
    /// [`is_fine`]: Self::is_fine
    unsafe fn unchecked(value: Self::Value) -> Self
    where
        Self::Value: Sized;

    /// Constructs [`Self`] from default [`Self::Value`] without checking.
    ///
    /// # Safety
    ///
    /// This method should only be called if the default value satisfies [`Self::Predicate`].
    ///
    /// Alternatively, this can be checked using the [`is_fine`] method.
    ///
    /// [`is_fine`]: Self::is_fine
    unsafe fn unchecked_default() -> Self
    where
        Self::Value: Default,
        Self: Sized,
    {
        unsafe { Self::unchecked(Self::Value::default()) }
    }

    /// Checks whether the given [`Self::Value`] satisfies [`Self::Predicate`].
    ///
    /// This is the same as calling [`Self::Predicate::check`] on the value.
    ///
    /// [`Self::Predicate::check`]: Predicate::check
    fn is_fine(value: &Self::Value) -> bool {
        Self::Predicate::check(value)
    }

    /// Returns the contained value reference.
    fn get_ref(&self) -> &Self::Value;

    /// Returns the contained value.
    fn get(self) -> Self::Value
    where
        Self::Value: Sized;

    /// Constructs [`Self`] from [`Self::Value`] by reference.
    ///
    /// # Errors
    ///
    /// Returns [`Error<Self::Value, Self::Predicate, Self::Context>`](Error)
    /// reference in case the reference does not satisfy the predicate.
    fn checked_ref(value: &Self::Value) -> RecoverableRefinementRef<'_, Self>;

    /// Constructs [`Self`] from [`Self::Value`].
    ///
    /// # Errors
    ///
    /// Returns [`Error<Self::Value, Self::Predicate, Self::Context>`](Error)
    /// in case the value does not satisfy the predicate.
    fn checked(value: Self::Value) -> RecoverableRefinement<Self>
    where
        Self::Value: Sized,
        Self: Sized;

    /// Constructs [`Self`] from default [`Self::Value`].
    ///
    /// # Errors
    ///
    /// Returns [`Error<Self::Value, Self::Predicate, Self::Context>`](Error)
    /// in case the default value does not satisfy the predicate.
    fn checked_default() -> RecoverableRefinement<Self>
    where
        Self::Value: Default,
        Self: Sized,
    {
        Self::checked(Self::Value::default())
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Constructs [`Self`] by reference without checking.
    ///
    /// # Safety
    ///
    /// This method should only be called for references that satisfy `P`,
    /// which can be checked via [`is_fine`].
    ///
    /// [`is_fine`]: Self::is_fine
    pub const unsafe fn unchecked_ref(value: &T) -> &Self {
        // SAFETY: layout of `T` matches `Self` due to `repr(transparent)`, so this is safe
        // moreover, the caller guarantees that `value` satisfies `P`
        unsafe { transmute(value) }
    }

    /// Returns the contained value reference.
    ///
    /// If the `unsafe-assert` feature is enabled, [`assert_unchecked`] is called on the reference
    /// before returning.
    ///
    /// The [`get_ref_no_assert`] method is provided in case the assertion is explicitly not needed.
    ///
    /// [`assert_unchecked`]: core::hint::assert_unchecked
    /// [`get_ref_no_assert`]: Self::get_ref_no_assert
    #[allow(clippy::missing_const_for_fn)] // conditionally const
    pub fn get_ref(&self) -> &T {
        let reference = self.get_ref_no_assert();

        #[cfg(feature = "unsafe-assert")]
        unsafe {
            assert_unchecked(Self::is_fine(reference));
        }

        reference
    }

    /// Returns the contained value.
    pub const fn get_ref_no_assert(&self) -> &T {
        &self.value
    }

    /// Checks whether the provided value satisfies `P`.
    ///
    /// This is the same as calling [`P::check`] on it.
    ///
    /// [`P::check`]: Predicate::check
    pub fn is_fine(value: &T) -> bool {
        P::check(value)
    }

    /// Constructs [`Self`] by reference.
    ///
    /// # Errors
    ///
    /// Returns [`Error<T, P, C>`] in case the reference does not satisfy `P`.
    ///
    /// [`Error<T, P, C>]: Error
    pub fn checked_ref(value: &T) -> RecoverableRef<'_, Self, P, C, T> {
        if Self::is_fine(value) {
            Ok(unsafe { Self::unchecked_ref(value) })
        } else {
            Err(Error::new_ref(value))
        }
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Constructs [`Self`] without checking the value.
    ///
    /// # Safety
    ///
    /// This method should only be called for values that satisfy the predicate `P`,
    /// which can be checked using [`is_fine`].
    ///
    /// [`is_fine`]: Self::is_fine
    pub const unsafe fn unchecked(value: T) -> Self {
        Self {
            predicate: PhantomData,
            context: PhantomData,
            value,
        }
    }

    /// Returns the contained value.
    ///
    /// If the `unsafe-assert` feature is enabled, [`assert_unchecked`] is called on the reference
    /// before returning the value.
    ///
    /// The [`get_no_assert`] method is provided in case the assertion is explicitly not needed.
    ///
    /// [`assert_unchecked`]: core::hint::assert_unchecked
    /// [`get_no_assert`]: Self::get_no_assert
    pub fn get(self) -> T {
        let value = self.get_no_assert();

        #[cfg(feature = "unsafe-assert")]
        unsafe {
            assert_unchecked(Self::is_fine(&value));
        }

        value
    }

    /// Returns the contained value.
    pub fn get_no_assert(self) -> T {
        self.value
    }

    /// Constructs [`Self`] from the given value.
    ///
    /// # Errors
    ///
    /// Returns [`Error<T, P, C>`] in case the value does not satisfy `P`.
    ///
    /// [`Error<T, P, C>]: Error
    pub fn checked(value: T) -> Recoverable<Self, P, C, T> {
        if Self::is_fine(&value) {
            Ok(unsafe { Self::unchecked(value) })
        } else {
            Err(Error::new(value))
        }
    }
}

impl<T: Default, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Constructs [`Self`] from the default value.
    ///
    /// # Safety
    ///
    /// This method should only be called if the default value satisfies `P`,
    /// which can be checked using [`is_fine`].
    ///
    /// [`is_fine`]: Self::is_fine
    pub unsafe fn unchecked_default() -> Self {
        unsafe { Self::unchecked(T::default()) }
    }

    /// Constructs [`Self`] from the default value.
    ///
    /// # Errors
    ///
    /// Returns [`Error<T, P, C>`] in case the default value does not satisfy `P`.
    ///
    /// [`Error<T, P, C>]: Error
    pub fn checked_default() -> Recoverable<Self, P, C, T> {
        Self::checked(T::default())
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> sealed::Sealed
    for Refinement<T, P, C>
{
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refining for Refinement<T, P, C> {
    type Value = T;
    type Predicate = P;
    type Context = C;

    unsafe fn unchecked_ref(value: &Self::Value) -> &Self {
        unsafe { Self::unchecked_ref(value) }
    }

    unsafe fn unchecked(value: Self::Value) -> Self
    where
        Self::Value: Sized,
    {
        unsafe { Self::unchecked(value) }
    }

    fn is_fine(value: &Self::Value) -> bool {
        Self::is_fine(value)
    }

    fn get_ref(&self) -> &Self::Value {
        self.get_ref()
    }

    fn get(self) -> Self::Value
    where
        Self::Value: Sized,
    {
        self.get()
    }

    fn checked_ref(value: &Self::Value) -> RecoverableRefinementRef<'_, Self> {
        Self::checked_ref(value)
    }

    fn checked(value: Self::Value) -> RecoverableRefinement<Self>
    where
        Self::Value: Sized,
    {
        Self::checked(value)
    }
}

/// Extends any type `T` to allow refining its values with `R` implementing [`Refining<Value = T>`].
pub trait Refine {
    /// Refines the given value with `R` by reference.
    ///
    /// This is the same as calling [`R::checked_ref`] on `Self` reference.
    ///
    /// # Errors
    ///
    /// See [`checked_ref`].
    ///
    /// [`R::checked_ref`]: Refining::checked_ref
    /// [`checked_ref`]: Refining::checked_ref
    fn refine_ref<R: Refining<Value = Self> + ?Sized>(&self) -> RecoverableRefinementRef<'_, R> {
        R::checked_ref(self)
    }

    /// Refines the given value with `R`.
    ///
    /// This is the same as calling [`R::checked`] on `Self` value.
    ///
    /// # Errors
    ///
    /// See [`checked`].
    ///
    /// [`R::checked`]: Refining::checked
    /// [`checked`]: Refining::checked
    fn refine<R: Refining<Value = Self>>(self) -> RecoverableRefinement<R>
    where
        Self: Sized,
    {
        R::checked(self)
    }
}

impl<T: ?Sized> Refine for T {}
