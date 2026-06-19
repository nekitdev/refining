//! Refinement types core.

#![no_std]
#![deny(missing_docs)]

pub mod context;
pub mod errors;
#[macro_use]
pub mod logical;
pub mod predicate;
pub mod prelude;
pub mod refinement;
#[macro_use]
pub mod types;

#[cfg(feature = "serde")]
pub mod serde;
