//! Predicates for integers.

use crate::internals::{signed_mod, unsigned_mod};

signed_mod!(i8 => i8);
signed_mod!(i16 => i16);
signed_mod!(i32 => i32);
signed_mod!(i64 => i64);
signed_mod!(i128 => i128);
signed_mod!(isize => isize);

unsigned_mod!(u8 => u8);
unsigned_mod!(u16 => u16);
unsigned_mod!(u32 => u32);
unsigned_mod!(u64 => u64);
unsigned_mod!(u128 => u128);
unsigned_mod!(usize => usize);
