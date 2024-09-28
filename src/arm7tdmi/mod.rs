mod instruction;
mod memory;
mod regfile;

use instruction::Instruction;

#[derive(Debug, Default)]
pub enum OpMode {
    User,
    _Fiq,
    Supervisor,
    _Abort,
    _Irq,
    _System,
    #[default]
    Undefined,
}
/*
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

use instruction::Instruction;
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
*/
pub struct Arm7TDMI {
    pub clock_cycle: usize,
    pub opmode: OpMode,
    pub regfile: regfile::RegFile,
    pub memory: memory::Memory,
    //pub procstate: ProcessorState,
    pub is_idle: bool,
    pub fetch_instr: Instruction,
    pub decode_instr: Instruction,
    pub exec_instr: Instruction,
}

impl Default for Arm7TDMI {
    fn default() -> Self {
        let mut constructed_val = Self {
            clock_cycle: 0usize,
            opmode: OpMode::User,
            regfile: regfile::RegFile::default(),
            memory: memory::Memory::default(),
            //procstate: ProcessorState::Idle,
            is_idle: false,
            fetch_instr: Instruction::default(),
            decode_instr: Instruction::default(),
            exec_instr: Instruction::default(),
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
        let cur_pc = self.regfile.get_register(15);
        let cur_cpsr = self.get_cpsr();
        let _ = self.set_mode(OpMode::Supervisor);
        self.regfile.set_register(14, cur_pc);
        self.regfile.set_register(17, cur_cpsr);

        // 2. Forces M[4:0] to b10011, Supervisor mode, sets the I and F bits, and clears the
        // T-bit in the CPSR.
        let _ = self.disable_fiq();
        let _ = self.disable_irq();
        let _ = self.enter_arm_mode();

        // 3. Forces the PC to fetch the next instruction from address 0x00.
        self.set_pc(0u32);

        // 4. Reverts to ARM state if necessary and resumes execution.
        // After reset, all register values except the PC and CPSR are indeterminate.

        // Reset emulation specific structures
        self.clock_cycle = 0usize;
        //self.procstate = ProcessorState::Idle;
        self.is_idle = true;
    }

    pub fn tick_clock(&mut self, num_ticks: usize) -> Result<(), &'static str> {
        if num_ticks > 1 {
            unimplemented!()
        } // TODO: Add support for running multiple cycles at once

        if self.is_idle {
            // Load initial pipeline contents
            let cur_pc = self
                .regfile
                .get_register(15);

            let raw_fetch_instr = self.memory.get_word((cur_pc + 8) as usize);
            self.fetch_instr = instruction::Instruction::from_bytes(raw_fetch_instr);

            let raw_decode_instr = self.memory.get_word((cur_pc + 4) as usize);
            self.decode_instr = instruction::Instruction::from_bytes(raw_decode_instr);

            let raw_exec_instr = self.memory.get_word(cur_pc as usize);
            self.exec_instr = instruction::Instruction::from_bytes(raw_exec_instr);

            self.is_idle = false;
        }

        // Execute Exec instr
        let control_flow_change = self
            .exec_instr
            .execute(&mut self.regfile, &mut self.memory)?;

        if control_flow_change {
            // Flush and reload pipeline
        } else {
            self.exec_instr = self.decode_instr;
            self.decode_instr = self.fetch_instr;
            let cur_pc = self
                .regfile
                .get_register(15);
            let raw_instr = self.memory.get_word(cur_pc as usize);
            self.fetch_instr = instruction::Instruction::from_bytes(raw_instr);
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
                self.regfile.get_register(0u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R8:  {:08x}\n",
                self.regfile.get_register(8u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R1:  {:08x}\t",
                self.regfile.get_register(1u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R9:  {:08x}\n",
                self.regfile.get_register(9u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R2:  {:08x}\t",
                self.regfile.get_register(2u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R10: {:08x}\n",
                self.regfile.get_register(10u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R3:  {:08x}\t",
                self.regfile.get_register(3u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R11: {:08x}\n",
                self.regfile.get_register(11u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R4:  {:08x}\t",
                self.regfile.get_register(4u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R12: {:08x}\n",
                self.regfile.get_register(12u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R5:  {:08x}\t",
                self.regfile.get_register(5u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R13: {:08x}\n",
                self.regfile.get_register(13u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R6:  {:08x}\t",
                self.regfile.get_register(6u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R14: {:08x}\n",
                self.regfile.get_register(14u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R7:  {:08x}\t",
                self.regfile.get_register(7u8)
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R15: {:08x}\n",
                self.regfile.get_register(15u8)
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

        ret_str.push_str(
            format!(
                "Current Exec State: {}\n",
                if self.is_idle { "IDLE" } else { "EXEC" }
            )
            .as_str(),
        );
        ret_str.push_str(format!("Clock Cycle: {}\n", self.clock_cycle).as_str());

        if !self.is_idle {
            ret_str.push_str(
                format!(
                    "Cur instrs:\nFET: {}\nDEC: {}\nEXE: {}\n",
                    self.fetch_instr, self.decode_instr, self.exec_instr
                )
                .as_str(),
            );
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
