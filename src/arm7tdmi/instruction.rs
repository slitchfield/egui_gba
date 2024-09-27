
#[derive(Debug)]
pub struct Instruction {
    raw_bytes: u32
}

impl Default for Instruction {
    fn default() -> Self {
        Self { raw_bytes: 0u32 }
    }
}

use std::fmt;
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction:\n\traw: {:#04x}\n", self.raw_bytes)
    }
}

impl Instruction {
    pub fn from_bytes(raw_bytes: u32) -> Self {
        Self { raw_bytes }
    }
}