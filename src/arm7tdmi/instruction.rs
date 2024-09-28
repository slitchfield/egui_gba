use super::{memory::Memory, regfile::RegFile};
use std::fmt;

// TODO: Evaluate necessity of Copy
#[allow(dead_code)] // Read of cond
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    src_addr: u32,
    raw_bytes: u32,
    cond: u8,
    inner_instr: InstrPayload,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            src_addr: 0u32,
            raw_bytes: 0u32,
            cond: 0u8,
            inner_instr: InstrPayload::Undefined,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruction @ {:#010x}:\n\traw: {:#08x}\n",
            self.src_addr, self.raw_bytes
        )?;
        write!(f, "\t{}", self.inner_instr)
    }
}

impl Instruction {
    pub fn from_bytes(fetch_addr: u32, raw_bytes: u32) -> Self {
        let cond: u8 = ((raw_bytes & 0xF0000000u32) >> 28) as u8;
        if cond == 0xfu8 {
            unimplemented!()
        } // COND==1111 results in UNPREDICTABLE for ARMv4

        println!("Parsing instruction: {:#04x}", raw_bytes);
        let high_bits = ((raw_bytes & 0x0e000000u32) >> 25) as u8;
        let inner_instr = match high_bits {
            0b000 => {
                // data processing immediate shift
                unimplemented!()
            }
            0b001 => {
                // data processing immediate | undef | move immed to status
                let b2324 = (raw_bytes & 0x01800000) >> 22;
                if b2324 == 0b10 {
                    // Undefined | move immed to status
                    unimplemented!();
                } else {
                    // data processing immed
                    let opcode: u8 = ((raw_bytes & 0x01e00000) >> 21) as u8;
                    let s: bool = ((raw_bytes & 0x00100000) >> 20) != 0;
                    let rn: u8 = ((raw_bytes & 0x000f0000) >> 16) as u8;
                    let rd: u8 = ((raw_bytes & 0x0000f000) >> 12) as u8;
                    let rotate: u8 = ((raw_bytes & 0x00000f00) >> 8) as u8;
                    let immed: u8 = (raw_bytes & 0x000000ff) as u8;
                    match opcode {
                        0b0010 =>
                        // SUBI
                        {
                            InstrPayload::SubI {
                                s,
                                rn,
                                rd,
                                rotate,
                                immed,
                            }
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                }
            }
            0b100 => {
                // Load/store multiple
                let p: bool = (raw_bytes & 0x01000000) != 0;
                let u: bool = (raw_bytes & 0x00800000) != 0;
                let s: bool = (raw_bytes & 0x00400000) != 0;
                let w: bool = (raw_bytes & 0x00200000) != 0;
                let l: bool = (raw_bytes & 0x00100000) != 0;
                let rn: u8 = ((raw_bytes & 0x000f0000) >> 16) as u8;
                let reglist: u16 = (raw_bytes & 0x0000ffff) as u16;
                InstrPayload::LSMultiple {
                    p,
                    u,
                    s,
                    w,
                    l,
                    rn,
                    reglist,
                }
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
            src_addr: fetch_addr,
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
    LSMultiple {
        p: bool,
        u: bool,
        s: bool,
        w: bool,
        l: bool,
        rn: u8,
        reglist: u16,
    },
    SubI {
        s: bool,
        rn: u8,
        rd: u8,
        rotate: u8,
        immed: u8,
    },
}

impl InstrPayload {
    fn execute(&self, regfile: &mut RegFile, _memory: &mut Memory) -> Result<bool, &'static str> {
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
                regfile.set_register(15, cur_pc + (calculated_offset * 4));

                // Need to clear pipeline, so return true
                Ok(true)
            }
            Self::BranchAndLink { offset: _ } => {
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }
}

#[allow(unreachable_patterns)] // Allow _ catch all for future proofing
impl fmt::Display for InstrPayload {
    #[allow(clippy::print_in_format_impl)] // Println! lives in panic case
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => {
                write!(f, "UNDEFINED")
            }
            Self::Branch { offset } => {
                write!(
                    f,
                    "Branch (offset = {:#08x}) PC <= PC + {}",
                    offset,
                    4 * (*offset as i32)
                )
            }
            Self::BranchAndLink { offset } => {
                write!(f, "Branch And Link (offset = {:#08x})", offset)
            }
            Self::LSMultiple {
                p,
                u,
                s,
                w,
                l,
                rn,
                reglist,
            } => {
                //todo: Update printed output
                write!(
                    f,
                    "LSMU (p:{}) (u:{}) (s:{}) (w:{}) (l:{}) -> Store {} at Mem(R{}) TODO",
                    p, u, s, w, l, reglist, rn
                )
            }
            Self::SubI {
                s,
                rn,
                rd,
                rotate,
                immed,
            } => {
                write!(
                    f,
                    "SUBI (s:{}) -> R{} = R{} - ({} >> {}*2)",
                    s, rn, rd, immed, rotate
                )
            }
            _ => {
                println!("Tried to print instruction enum {:?}", self);
                unimplemented!()
            }
        }
    }
}
