# `refining`

[![License][License Badge]][License]
[![Version][Version Badge]][Crate]
[![Downloads][Downloads Badge]][Crate]
[![Test][Test Badge]][Actions]

> *Refinement types.*

## Installation

### `cargo`

You can add `refining` as a dependency with the following command:

```console
$ cargo add refining
```

Or by directly specifying it in the configuration like so:

```toml
[dependencies]
refining = "0.1.0"
```

Alternatively, you can add it directly from the source:

```toml
[dependencies.refining]
git = "https://github.com/nekitdev/refining.git"
```

## Examples

```rust
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

## Features

### `std`

### `empty`

### `length`

### `int`

### `char`

### `str`

### `regex`

## Documentation

You can find the documentation [here][Documentation].

## Support

If you need support with the library, you can send an [email][Email].

## Changelog

You can find the changelog [here][Changelog].

## Security Policy

You can find the Security Policy of `refining` [here][Security].

## Contributing

If you are interested in contributing to `refining`, make sure to take a look at the
[Contributing Guide][Contributing Guide], as well as the [Code of Conduct][Code of Conduct].

## License

`refining` is licensed under the MIT License terms. See [License][License] for details.

[Email]: mailto:support@nekit.dev

[Discord]: https://nekit.dev/chat

[Actions]: https://github.com/nekitdev/refining/actions

[Changelog]: https://github.com/nekitdev/refining/blob/main/CHANGELOG.md
[Code of Conduct]: https://github.com/nekitdev/refining/blob/main/CODE_OF_CONDUCT.md
[Contributing Guide]: https://github.com/nekitdev/refining/blob/main/CONTRIBUTING.md
[Security]: https://github.com/nekitdev/refining/blob/main/SECURITY.md

[License]: https://github.com/nekitdev/refining/blob/main/LICENSE

[Crate]: https://crates.io/crates/refining
[Documentation]: https://docs.rs/refining

[License Badge]: https://img.shields.io/crates/l/refining
[Version Badge]: https://img.shields.io/crates/v/refining
[Downloads Badge]: https://img.shields.io/crates/dr/refining
[Test Badge]: https://github.com/nekitdev/refining/workflows/test/badge.svg
