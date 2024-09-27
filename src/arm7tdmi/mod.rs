mod instruction;
mod memory;
mod regfile;

#[derive(Debug)]
pub enum OpMode {
    User,
    _Fiq,
    Supervisor,
    _Abort,
    _Irq,
    _System,
    _Undefined,
}

#[derive(Debug, Default)]
pub enum ProcessorState {
    #[default]
    Idle,
    Executing {
        fetch_instr: instruction::Instruction,
        decode_instr: instruction::Instruction,
        exec_instr: instruction::Instruction,
    },
}

use std::fmt;
impl fmt::Display for ProcessorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => {
                write!(f, "Idle")
            }
            Self::Executing { .. } => {
                write!(f, "Executing")
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub struct Arm7TDMI {
    pub clock_cycle: usize,
    pub opmode: OpMode,
    pub regfile: regfile::RegFile,
    pub memory: memory::Memory,
    pub procstate: ProcessorState,
}

impl Default for Arm7TDMI {
    fn default() -> Self {
        let mut constructed_val = Self {
            clock_cycle: 0usize,
            opmode: OpMode::User,
            regfile: regfile::RegFile::default(),
            memory: memory::Memory::default(),
            procstate: ProcessorState::Idle,
        };
        constructed_val.reset();
        constructed_val
    }
}

impl Arm7TDMI {
    pub fn load_bios_rom(&mut self, bios_rom_bytes: &Vec<u8>) -> Result<(), &'static str> {
        self.memory.load_bios_rom(&bios_rom_bytes)
    }

    pub fn reset(&mut self) {
        // When the nRESET signal goes LOW a reset occurs, and the ARM7TDMI core
        //   abandons the executing instruction and continues to increment the address bus as if still
        //   fetching word or halfword instructions. nMREQ and SEQ indicates internal cycles
        //   during this time.

        // When nRESET goes HIGH again, the ARM7TDMI processor:
        // 1. Overwrites R14_svc and SPSR_svc by copying the current values of the PC and
        // CPSR into them. The values of the PC and CPSR are indeterminate.
        let cur_pc = self.regfile.get_register(&self.opmode, 15).unwrap();
        let cur_cpsr = self.get_cpsr();
        self.regfile.r14_svc = cur_pc;
        self.regfile.spsr_svc = cur_cpsr;

        // 2. Forces M[4:0] to b10011, Supervisor mode, sets the I and F bits, and clears the
        // T-bit in the CPSR.
        let _ = self.set_mode(OpMode::Supervisor);
        let _ = self.disable_fiq();
        let _ = self.disable_irq();
        let _ = self.enter_arm_mode();

        // 3. Forces the PC to fetch the next instruction from address 0x00.
        self.set_pc(0u32);

        // 4. Reverts to ARM state if necessary and resumes execution.
        // After reset, all register values except the PC and CPSR are indeterminate.

        // Reset emulation specific structures
        self.clock_cycle = 0usize;
        self.procstate = ProcessorState::Idle;
    }

    pub fn tick_clock(&mut self, num_ticks: usize) -> Result<(), &'static str> {
        if num_ticks > 1 {
            unimplemented!()
        } // TODO: Add support for running multiple cycles at once

        match self.procstate {
            ProcessorState::Idle => {
                // Fetch instruction and begin executing
                let cur_pc = self
                    .regfile
                    .get_register(&self.opmode, 15)
                    .ok_or("Could not retrieve PC?")?;
                let raw_instr = self.memory.get_word(cur_pc as usize);
                self.procstate = ProcessorState::Executing {
                    fetch_instr: instruction::Instruction::from_bytes(raw_instr),
                    decode_instr: instruction::Instruction::default(),
                    exec_instr: instruction::Instruction::default(),
                };
            }
            ProcessorState::Executing { .. } => {}
            _ => {
                unimplemented!()
            }
        }

        self.clock_cycle += 1usize;
        Ok(())
    }
    pub fn print_state(&self) -> String {
        let mut ret_str: String = String::new();
        ret_str.push_str(format!("Current State: {:?}\n", &self.opmode).as_str());
        ret_str.push_str(
            format!(
                "R0:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 0u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R8:  {:08x}\n",
                self.regfile.get_register(&self.opmode, 8u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R1:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 1u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R9:  {:08x}\n",
                self.regfile.get_register(&self.opmode, 9u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R2:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 2u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R10: {:08x}\n",
                self.regfile.get_register(&self.opmode, 10u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R3:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 3u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R11: {:08x}\n",
                self.regfile.get_register(&self.opmode, 11u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R4:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 4u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R12: {:08x}\n",
                self.regfile.get_register(&self.opmode, 12u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R5:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 5u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R13: {:08x}\n",
                self.regfile.get_register(&self.opmode, 13u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R6:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 6u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R14: {:08x}\n",
                self.regfile.get_register(&self.opmode, 14u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R7:  {:08x}\t",
                self.regfile.get_register(&self.opmode, 7u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R15: {:08x}\n",
                self.regfile.get_register(&self.opmode, 15u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str("\n");
        ret_str.push_str(self.regfile.print_cpsr_state().as_str());
        ret_str.push_str("\n\n");

        ret_str.push_str(self.memory.print_memory(64).as_str());

        return ret_str;
    }

    pub fn print_exec_state(&self) -> String {
        let mut ret_str = String::new();

        ret_str.push_str(format!("Current Exec State: {}\n", self.procstate).as_str());
        ret_str.push_str(format!("Clock Cycle: {}\n", self.clock_cycle).as_str());

        match &self.procstate {
            ProcessorState::Idle => {}
            ProcessorState::Executing { fetch_instr, decode_instr, exec_instr } => {
                ret_str.push_str(format!("Cur instrs:\nFET: {}\nDEC: {}\nEXE: {}\n", fetch_instr, decode_instr, exec_instr).as_str());
            }
        }

        ret_str
    }
    pub fn get_cpsr(&self) -> u32 {
        return self.regfile.get_cpsr();
    }

    pub fn set_mode(&mut self, opmode: OpMode) -> Result<(), &'static str> {
        self.opmode = opmode;
        self.regfile.set_cpsr_mode(&self.opmode)
    }

    pub fn disable_fiq(&mut self) -> Result<(), &'static str> {
        self.regfile.set_cpsr_bits(7, 1, 0b1)
    }

    pub fn disable_irq(&mut self) -> Result<(), &'static str> {
        self.regfile.set_cpsr_bits(6, 1, 0b1)
    }

    pub fn enter_arm_mode(&mut self) -> Result<(), &'static str> {
        self.regfile.set_cpsr_bits(5, 1, 0b0)
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.regfile.set_pc(new_pc);
    }
}
