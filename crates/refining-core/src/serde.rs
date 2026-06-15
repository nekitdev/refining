//! Implementations of [`Serialize`] and [`Deserialize`] for [`Refinement`].

#[cfg(not(feature = "serde"))]
compile_error!("expected `serde` to be enabled");

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{
    predicate::Predicate,
    refinement::{Refine, Refinement},
    types::TypeStr,
};

impl<T: Serialize + ?Sized, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Serialize
    for Refinement<T, P, C>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get_ref().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Deserialize<'de>
    for Refinement<T, P, C>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;

        let refined = value.refine().map_err(D::Error::custom)?;

        Ok(refined)
    }
}
