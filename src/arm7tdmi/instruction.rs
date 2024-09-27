#[derive(Debug)]
pub struct Instruction {
    raw_bytes: u32,
    cond: u8,
    inner_instr: InstrPayload,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            raw_bytes: 0u32,
            cond: 0u8,
            inner_instr: InstrPayload::Undefined,
        }
    }
}

use std::fmt;
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction:\n\traw: {:#04x}\n", self.raw_bytes)?;
        write!(f, "\t{}", self.inner_instr)
    }
}

impl Instruction {
    pub fn from_bytes(raw_bytes: u32) -> Self {
        let cond: u8 = ((raw_bytes & 0xF0000000u32) >> 28) as u8;
        if cond == 0xfu8 {
            unimplemented!()
        } // COND==1111 results in UNPREDICTABLE for ARMv4

        let high_bits = ((raw_bytes & 0x0e000000u32) >> 25) as u8;
        let inner_instr = match high_bits {
            0b000 => {
                // data processing immediate shift
                unimplemented!()
            }
            0b101 => {
                // Branch and branch with link
                let l_bit = ((raw_bytes & 0x01000000u32) >> 24) as u8;
                let offset = raw_bytes & 0x00ffffff;
                if l_bit == 1 {
                    InstrPayload::BranchAndLink { offset }
                } else if l_bit == 0 {
                    InstrPayload::Branch { offset }
                } else {
                    unreachable!()
                }
            }
            _ => {
                println!("received high bits of {:#03b}", high_bits);
                unimplemented!()
            }
        };

        Self {
            cond,
            raw_bytes,
            inner_instr,
        }
    }
}

#[derive(Debug, Default)]
enum InstrPayload {
    #[default]
    Undefined,
    Branch {
        offset: u32,
    },
    BranchAndLink {
        offset: u32,
    },
}

impl fmt::Display for InstrPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => {
                write!(f, "UNDEFINED")
            }
            Self::Branch { offset } => {
                write!(f, "Branch (offset = {:#08x})", offset)
            }
            Self::BranchAndLink { offset } => {
                write!(f, "Branch And Link (offset = {:#08x})", offset)
            }
            _ => {
                println!("Tried to print instruction enum {:?}", self);
                unimplemented!()
            }
        }
    }
}
