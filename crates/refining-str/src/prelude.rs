//! The `refining-str` prelude.

#[doc(inline)]
pub use crate::{
    bytes::{BytesAll, BytesAny},
    chars::{CharsAll, CharsAny},
    predicates::{
        Ascii, Contains, ContainsChar, EndsWith, EndsWithChar, StartsWith, StartsWithChar, Trimmed,
        TrimmedEnd, TrimmedStart,
    },
};
