use crate::arm7tdmi::OpMode;
use crate::util;

#[allow(dead_code)] // Unused reg reads
#[derive(Default)]
pub struct RegFile {
    mode: OpMode,
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
    pub fn get_register(&self, idx: u8) -> u32 {
        assert!(idx <= 15);
        match idx {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.r8,
            9 => self.r9,
            10 => self.r10,
            11 => self.r11,
            12 => self.r12,
            13 => match self.mode {
                OpMode::User => self.r13,
                OpMode::Supervisor => self.r13_svc,
                _ => unimplemented!(),
            },
            14 => match self.mode {
                OpMode::User => self.r14,
                OpMode::Supervisor => self.r14_svc,
                _ => unimplemented!(),
            },
            15 => self.r15_pc,
            _ => unimplemented!(),
        }
    }

    pub fn set_register(&mut self, idx: u8, value: u32) {
        assert!(idx <= 17);
        match idx {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.r8 = value,
            9 => self.r9 = value,
            10 => self.r10 = value,
            11 => self.r11 = value,
            12 => self.r12 = value,
            13 => match self.mode {
                OpMode::User => self.r13 = value,
                OpMode::Supervisor => self.r13_svc = value,
                _ => {
                    unimplemented!()
                }
            },
            14 => match self.mode {
                OpMode::User => self.r14 = value,
                OpMode::Supervisor => self.r14_svc = value,
                _ => {
                    unimplemented!()
                }
            },
            15 => self.r15_pc = value,
            16 => self.cpsr = value,
            17 => match self.mode {
                OpMode::User => unimplemented!(), // SPSR not valid for user mode
                OpMode::Supervisor => self.spsr_svc = value,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    pub fn get_cpsr(&self) -> u32 {
        self.cpsr
    }

    pub fn set_cpsr_bits(&mut self, offset: u8, num: u8, bits: u32) -> Result<(), &'static str> {
        self.cpsr = util::set_bits(self.cpsr, offset, num, bits);
        Ok(())
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.r15_pc = new_pc;
    }

    pub fn set_cpsr_mode(&mut self, mode: &OpMode) -> Result<(), &'static str> {
        match mode {
            OpMode::User => {
                self.mode = OpMode::User;
                self.set_cpsr_bits(0, 5, 0b10000)
            }
            OpMode::Supervisor => {
                self.mode = OpMode::Supervisor;
                self.set_cpsr_bits(0, 5, 0b10011)
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn print_cpsr_state(&self) -> String {
        let mut ret_str = String::new();
        ret_str.push_str("N|Z|C|V|---|I|F|T|MODES\n");
        ret_str.push_str(
            format!(
                "{}|{}|{}|{}|---|{}|{}|{}|{:05b}",
                util::get_bits(self.cpsr, 31, 1),
                util::get_bits(self.cpsr, 30, 1),
                util::get_bits(self.cpsr, 29, 1),
                util::get_bits(self.cpsr, 28, 1),
                util::get_bits(self.cpsr, 7, 1),
                util::get_bits(self.cpsr, 6, 1),
                util::get_bits(self.cpsr, 5, 1),
                util::get_bits(self.cpsr, 0, 5)
            )
            .as_str(),
        );
        ret_str
    }
}
