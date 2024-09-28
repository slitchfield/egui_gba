use std::path::PathBuf;

use crate::arm7tdmi::{self, Arm7TDMI};

pub struct Gbaemu {
    rompath: PathBuf,
    rombytes: Vec<u8>,
    biosrombytes: Vec<u8>,
    status_bar: String,

    arm_core: arm7tdmi::Arm7TDMI,
}

impl Default for Gbaemu {
    fn default() -> Self {
        Self {
            rompath: PathBuf::new(),
            rombytes: vec![],
            biosrombytes: vec![],

            status_bar: String::new(),

            arm_core: Arm7TDMI::default(),
        }
    }
}

impl Gbaemu {
    pub fn load_rom(&mut self, rompath: String, rombytes: &[u8]) -> Result<(), &'static str> {
        self.rompath = PathBuf::from(rompath.clone());
        self.rombytes = rombytes.to_owned();
        self.status_bar = format!(
            "{:04x} | Loading rom file: \"{}\"",
            self.arm_core.get_cpsr(),
            rompath
        );
        Ok(())
    }

    pub fn load_bios_rom(&mut self, rompath: String, rombytes: &[u8]) -> Result<(), &'static str> {
        self.rompath = PathBuf::from(rompath.clone());
        self.biosrombytes = rombytes.to_owned();
        self.status_bar = format!(
            "{:04x} | Loading bios file: \"{}\"",
            self.arm_core.get_cpsr(),
            rompath
        );
        self.arm_core.load_bios_rom(&self.biosrombytes)
    }

    pub fn reset(&mut self) {
        println!("Resetting system...");
        self.status_bar = "Resetting system".to_string();
        self.arm_core.reset()
    }

    pub fn tick_clock(&mut self, num_ticks: usize) -> Result<(), &'static str> {
        if num_ticks > 1 {
            unimplemented!()
        } // TODO: Add support for running multiple cycles at once

        if self.biosrombytes.is_empty() {
            self.status_bar = "Skipping clock ticks without bios".to_string();
            Ok(())
        } else {
            self.arm_core.tick_clock(num_ticks)
        }
    }

    pub fn get_status(&self) -> String {
        self.status_bar.clone()
    }

    pub fn get_core_state(&self) -> String {
        self.arm_core.print_state()
    }

    pub fn get_execution_state(&self) -> String {
        self.arm_core.print_exec_state()
    }

    pub fn advance_mem_cursor(&mut self) {
        self.arm_core.memory.advance_mem_cursor()
    }

    pub fn regress_mem_cursor(&mut self) {
        self.arm_core.memory.regress_mem_cursor()
    }
}
