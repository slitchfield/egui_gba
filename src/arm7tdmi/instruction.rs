use super::{memory::Memory, regfile::RegFile};
use std::fmt;

// TODO: Evaluate necessity of Copy
#[derive(Debug, Clone, Copy)]
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

    pub fn execute(
        &self,
        regfile: &mut RegFile,
        memory: &mut Memory,
    ) -> Result<bool, &'static str> {
        self.inner_instr.execute(regfile, memory)
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

impl InstrPayload {
    fn execute(&self, regfile: &mut RegFile, memory: &mut Memory) -> Result<bool, &'static str> {
        match self {
            Self::Undefined => Err("Tried to execute undefined instruction"),
            Self::Branch { offset } => {

                let mut calculated_offset = *offset;
                // sign extend offset (encoded in 24b)
                if calculated_offset & 0x800000 > 0 {
                    calculated_offset |= 0xff000000;
                }

                // r15 += offset * 4
                let cur_pc = regfile.get_register(15);
                regfile.set_register(15, cur_pc + (calculated_offset * 4) );

                // Need to clear pipeline

                //unimplemented!();
                Ok(true)
            }
            Self::BranchAndLink { offset } => {
                unimplemented!()
            }
        }
    }
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
