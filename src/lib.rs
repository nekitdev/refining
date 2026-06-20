//! Refinement types.
//!
//! [Refinement types][refinement-types] are types endowed with predicates that are assumed
//! to hold for any value of the type. This crate provides ways to define and use refinement types.
//!
//! Refinement types are useful for encoding invariants in the type system,
//! which can help catch errors and improve safety along with correctness.
//!
//! Actual checks are performed at **runtime**, so that the type system encodes the invariants,
//! but does not enforce them at compile time. This allows for more flexibility, but also means
//! that the developer needs to be careful when defining and using refinement types.
//!
//! [refinement-types]: https://en.wikipedia.org/wiki/Refinement_type
//!
//! # Predicates
//!
//! This crate provides a variety of predicates for different types, such as integers, strings and collections.
//!
//! ## Custom
//!
//! Defining custom predicates is possible, which is done via the [`Predicate`] trait:
//!
//! ```
//! use core::fmt;
//!
//! use refining::prelude::{Predicate, Refinement};
//!
//! struct Vector {
//!     x: f64,
//!     y: f64,
//! }
//!
//! impl Vector {
//!     fn length(&self) -> f64 {
//!         (self.x * self.x + self.y * self.y).sqrt()
//!     }
//! }
//!
//! // the predicate
//! struct Normalized;
//!
//! // trait implementation
//! impl Predicate<Vector> for Normalized {
//!     fn check(value: &Vector) -> bool {
//!         value.length() == 1.0
//!     }
//!
//!     fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         formatter.write_str("normalized vector")
//!     }
//!
//!     fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         formatter.write_str("normalized")
//!     }
//! }
//!
//! // the refinement type
//! type NormalizedVector = Refinement<Vector, Normalized>;
//! ```
//!
//! [`Predicate`]: crate::prelude::Predicate
//!
//! ## Composition
//!
//! Predicates can be composed using logical combinators, such as [`And`], [`Or`], [`Not`] and [`Xor`].
//!
//! This crate also provides [`Nand`], [`Nor`], [`Xnor`] and [`Implies`] for convenience,
//! along with [`True`] and [`False`] for those who might need them.
//!
//! There are also [`and!`], [`or!`], [`not!`] and [`xor!`] macros for more ergonomic composition.
//!
//! Note that amazing `and!(P, Q, R, S, T)` expands into `And<P, And<Q, And<R, And<S, T>>>>`,
//! so use with care!
//!
//! [`And`]: crate::prelude::And
//! [`Or`]: crate::prelude::Or
//! [`Not`]: crate::prelude::Not
//! [`Xor`]: crate::prelude::Xor
//! [`Nand`]: crate::prelude::Nand
//! [`Nor`]: crate::prelude::Nor
//! [`Xnor`]: crate::prelude::Xnor
//! [`Implies`]: crate::prelude::Implies
//! [`True`]: crate::prelude::True
//! [`False`]: crate::prelude::False
//!
//! [`and!`]: crate::prelude::and
//! [`or!`]: crate::prelude::or
//! [`not!`]: crate::prelude::not
//! [`xor!`]: crate::prelude::xor

//! # Examples
//!
//! Let us see how to use refinement types with some examples. We will be using the provided predicates.

#![cfg_attr(
    feature = "empty",
    doc = r#"
For example, non-empty strings:

```
use refining::prelude::{NonEmpty, Refinement};

type NonEmptyStr = Refinement<str, NonEmpty>;

// for instance, we can safely extract the first character
// without worrying about empty strings
fn first(non_empty: &NonEmptyStr) -> char {
    let first = non_empty.chars().next().unwrap();

    first
}
```

Here, `NonEmptyStr` encodes non-emptiness, so that `first` can not actually panic.

Although for this specific example, one can use [`non-empty-str`][non-empty-str]
for improved ergonomics.

[non-empty-str]: https://docs.rs/non-empty-str
"#
)]
#![cfg_attr(
    feature = "regex",
    doc = r#"
Or match strings against regular expressions!

```
use refining::prelude::{Matches, Refinement, type_regex};

type_regex!(Natural = "^0|[1-9][0-9]*$");

type NaturalStr = Refinement<str, Matches<Natural>>;
```
"#
)]
#![cfg_attr(
    feature = "int",
    doc = r#"
And, of course, refine integers!

```
use refining::prelude::{Refinement, u8};

type Level = Refinement<u8, u8::Closed<0, 100>>;

fn print(level: Level) {
    println!("{level}%");
}
```
"#
)]
#![cfg_attr(
    all(feature = "std", feature = "length", feature = "int", feature = "str"),
    doc = r#"
For the final example, let's look at more complex use cases.

```no_run
use std::fmt;

use anyhow::Result;
use tiny_input::tiny_input;

use refining::prelude::*;

type Name = Refinement<String, And<Ascii, LengthClosed<1, 32>>>;
type Charge = Refinement<u8, u8::Closed<0, 100>>;

struct Device {
    name: Name,
    charge: Charge,
}

impl fmt::Display for Device {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{name} ({charge}%)", name = self.name, charge = self.charge)
    }
}

fn main() -> Result<()> {
    let name = tiny_input!(as String, "device name: ")?.refine()?;
    let charge = tiny_input!(as u8, "device charge: ")?.refine()?;

    let device = Device { name, charge };

    println!("device: {device}");

    Ok(())
}
```

Running this example:

```console
$ cargo run --example device
device name: nekit
device charge: 69
device: nekit (69%)
```

We get the desired output, nice!

For the final note, with the `serde` feature enabled, we can add `#[derive(Serialize, Deserialize)]`
to the `Device` structure, and it will work as expected, so that you can pass around `Device` in APIs.
"#
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use refining_core as core;

#[cfg(feature = "empty")]
pub use refining_empty as empty;

#[cfg(feature = "length")]
pub use refining_length as length;

#[cfg(feature = "int")]
pub use refining_int as int;

#[cfg(feature = "char")]
pub use refining_char as char;

#[cfg(feature = "str")]
pub use refining_str as str;

#[cfg(feature = "regex")]
pub use refining_regex as regex;

pub mod prelude;
