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
        write!(
            formatter,
            "{name} ({charge}%)",
            name = self.name,
            charge = self.charge
        )
    }
}

fn main() -> Result<()> {
    let name = tiny_input!(as String, "device name: ")?.refine()?;
    let charge = tiny_input!(as u8, "device charge: ")?.refine()?;

    let device = Device { name, charge };

    println!("device: {device}");

    Ok(())
}
