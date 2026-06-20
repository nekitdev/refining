//! The `refining-core` prelude.

#[doc(inline)]
pub use crate::{
    and,
    context::NoContext,
    errors::{Error, Recoverable, RecoverableRef},
    logical::{And, False, Implies, Nand, Nor, Not, Or, True, Xnor, Xor},
    not, or,
    predicate::{Check, Predicate, PredicateExpected},
    refinement::{RecoverableRefinement, RecoverableRefinementRef, Refine, Refinement, Refining},
    type_str,
    types::{StaticStr, TypeStr},
    xor,
};
