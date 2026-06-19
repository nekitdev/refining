//! Refinement types.

#![cfg_attr(
    feature = "default",
    doc = r#"
# Examples

```
use core::fmt;

use anyhow::Result;

use refining::prelude::*;

type_str!(DeviceName = "device name");
type_str!(DeviceCharge = "device charge");

type Name = Refinement<str, And<Ascii, LengthClosed<1, 32>>, DeviceName>;
type Charge = Refinement<u8, u8::Closed<1, 100>, DeviceCharge>;

struct Device<'a> {
    name: &'a Name,
    charge: Charge,
}

impl fmt::Display for Device<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{name} ({charge}%)",
            name = self.name,
            charge = self.charge
        )
    }
}

fn main() -> Result<()> {
    let name = "nekit".refine_ref()?;
    let charge = 42.refine()?;

    let device = Device { name, charge };

    println!("{device}");

    Ok(())
}
```

Running the example will print the following output:

```text
nekit (42%)
```
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
