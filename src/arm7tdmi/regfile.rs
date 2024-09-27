use crate::arm7tdmi::OpMode;
use crate::util;

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
    pub r14_svc: u32, // TODO: Revisit design to change directly setting regs
    pub spsr_svc: u32,
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
    pub fn get_register(&self, mode: &OpMode, idx: u8) -> Option<u32> {
        assert!(idx <= 15);
        match mode {
            OpMode::User => match idx {
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
            OpMode::Supervisor => match idx {
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
            OpMode::User => self.set_cpsr_bits(0, 5, 0b10000),
            OpMode::Supervisor => self.set_cpsr_bits(0, 5, 0b10011),
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
