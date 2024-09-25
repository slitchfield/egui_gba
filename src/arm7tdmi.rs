use crate::util;

#[derive(Debug)]
pub enum Status {
    User,
    _Fiq,
    Supervisor,
    _Abort,
    _Irq,
    _System,
    _Undefined,
}

#[derive(Default)]
pub struct RegFile {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    r14: u32,
    r15_pc: u32,
    cpsr: u32,
    // FIQ Op mode
    r8_fiq: u32,
    r9_fiq: u32,
    r10_fiq: u32,
    r11_fiq: u32,
    r12_fiq: u32,
    r13_fiq: u32,
    r14_fiq: u32,
    spsr_fiq: u32,
    // Supervisor Op mode
    r13_svc: u32,
    r14_svc: u32,
    spsr_svc: u32,
    // Abord Op mode
    r13_abt: u32,
    r14_abt: u32,
    spsr_abt: u32,
    // IRQ Op mode
    r13_irq: u32,
    r14_irq: u32,
    spsr_irq: u32,
    // Undefined Op mode
    r13_und: u32,
    r14_und: u32,
    spsr_und: u32,
}

impl RegFile {
    pub fn get_register(&self, mode: &Status, idx: u8) -> Option<u32> {
        assert!(idx <= 15);
        match mode {
            Status::User => match idx {
                0 => Some(self.r0),
                1 => Some(self.r1),
                2 => Some(self.r2),
                3 => Some(self.r3),
                4 => Some(self.r4),
                5 => Some(self.r5),
                6 => Some(self.r6),
                7 => Some(self.r7),
                8 => Some(self.r8),
                9 => Some(self.r9),
                10 => Some(self.r10),
                11 => Some(self.r11),
                12 => Some(self.r12),
                13 => Some(self.r13),
                14 => Some(self.r14),
                15 => Some(self.r15_pc),

                _ => unimplemented!(),
            },
            Status::Supervisor => match idx {
                0 => Some(self.r0),
                1 => Some(self.r1),
                2 => Some(self.r2),
                3 => Some(self.r3),
                4 => Some(self.r4),
                5 => Some(self.r5),
                6 => Some(self.r6),
                7 => Some(self.r7),
                8 => Some(self.r8),
                9 => Some(self.r9),
                10 => Some(self.r10),
                11 => Some(self.r11),
                12 => Some(self.r12),
                13 => Some(self.r13_svc),
                14 => Some(self.r14_svc),
                15 => Some(self.r15_pc),
                
                _ => unimplemented!(),
             },
            _ => unimplemented!(),
        }
    }

    pub fn set_cpsr_bits(&mut self, offset: u8, num: u8, bits: u32) -> Result<(), &'static str> {
        self.cpsr = util::set_bits(self.cpsr, offset, num, bits);
        Ok(())
    }

    pub fn set_mode(&mut self, mode: &Status) -> Result<(), &'static str> {
        match mode {
            Status::User => self.set_cpsr_bits(0, 5, 0b10000),
            Status::Supervisor => self.set_cpsr_bits(0, 5, 0b10011),
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn print_cpsr_state(&self) -> String {
        let mut ret_str = String::new();
        ret_str.push_str("N|Z|C|V|---|I|F|T|MODES\n");
        ret_str.push_str(format!("{}|{}|{}|{}|---|{}|{}|{}|{:05b}", util::get_bits(self.cpsr, 31, 1), util::get_bits(self.cpsr, 30, 1), util::get_bits(self.cpsr, 29, 1), util::get_bits(self.cpsr, 28, 1), util::get_bits(self.cpsr, 7, 1), util::get_bits(self.cpsr, 6, 1), util::get_bits(self.cpsr, 5, 1), util::get_bits(self.cpsr, 0, 5)).as_str());
        ret_str
    }
}

pub struct Arm7TDMI {
    pub status: Status,
    pub regfile: RegFile,
}

impl Default for Arm7TDMI {
    fn default() -> Self {
        let mut constructed_val = Self {
            status: Status::User,
            regfile: RegFile::default(),
        };
        constructed_val.reset();
        constructed_val
    }
}

impl Arm7TDMI {
    pub fn reset(&mut self) {
        // When the nRESET signal goes LOW a reset occurs, and the ARM7TDMI core
        //   abandons the executing instruction and continues to increment the address bus as if still
        //   fetching word or halfword instructions. nMREQ and SEQ indicates internal cycles
        //   during this time.

        // When nRESET goes HIGH again, the ARM7TDMI processor:
        // 1. Overwrites R14_svc and SPSR_svc by copying the current values of the PC and
        // CPSR into them. The values of the PC and CPSR are indeterminate.
        let cur_pc = self.regfile.get_register(&self.status, 15).unwrap();
        let cur_cpsr = self.get_cpsr();
        self.regfile.r14_svc = cur_pc;
        self.regfile.spsr_svc = cur_cpsr;

        // 2. Forces M[4:0] to b10011, Supervisor mode, sets the I and F bits, and clears the
        // T-bit in the CPSR.
        let _ = self.set_mode(Status::Supervisor);
        let _ = self.disable_fiq();
        let _ = self.disable_irq();
        let _ = self.enter_arm_mode();

        // 3. Forces the PC to fetch the next instruction from address 0x00.
        self.set_pc(0u32);

        // 4. Reverts to ARM state if necessary and resumes execution.
        // After reset, all register values except the PC and CPSR are indeterminate.
    }

    pub fn print_state(&self) -> String {
        let mut ret_str: String = String::new();
        ret_str.push_str(format!("Current State: {:?}\n", &self.status).as_str());
        ret_str.push_str(
            format!(
                "R0:  {:08x}\t",
                self.regfile.get_register(&self.status, 0u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R8:  {:08x}\n",
                self.regfile.get_register(&self.status, 8u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R1:  {:08x}\t",
                self.regfile.get_register(&self.status, 1u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R9:  {:08x}\n",
                self.regfile.get_register(&self.status, 9u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R2:  {:08x}\t",
                self.regfile.get_register(&self.status, 2u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R10: {:08x}\n",
                self.regfile.get_register(&self.status, 10u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R3:  {:08x}\t",
                self.regfile.get_register(&self.status, 3u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R11: {:08x}\n",
                self.regfile.get_register(&self.status, 11u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R4:  {:08x}\t",
                self.regfile.get_register(&self.status, 4u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R12: {:08x}\n",
                self.regfile.get_register(&self.status, 12u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R5:  {:08x}\t",
                self.regfile.get_register(&self.status, 5u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R13: {:08x}\n",
                self.regfile.get_register(&self.status, 13u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R6:  {:08x}\t",
                self.regfile.get_register(&self.status, 6u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R14: {:08x}\n",
                self.regfile.get_register(&self.status, 14u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R7:  {:08x}\t",
                self.regfile.get_register(&self.status, 7u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str(
            format!(
                "R15: {:08x}\n",
                self.regfile.get_register(&self.status, 15u8).unwrap()
            )
            .as_str(),
        );
        ret_str.push_str("\n");
        ret_str.push_str(self.regfile.print_cpsr_state().as_str());
        return ret_str;
    }

    pub fn get_cpsr(&self) -> u32 {
        return self.regfile.cpsr;
    }

    pub fn set_mode(&mut self, status: Status) -> Result<(), &'static str> {
        self.status = status;
        self.regfile.set_mode(&self.status)
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
        self.regfile.r15_pc = new_pc
    }
}
