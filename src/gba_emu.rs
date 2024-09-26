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
    pub fn load_rom(&mut self, rompath: String, rombytes: &Vec<u8>) -> () {
        self.rompath = PathBuf::from(rompath.clone());
        self.rombytes = rombytes.clone();
        self.status_bar = format!(
            "{:04x} | Loading file: \"{}\"",
            self.arm_core.get_cpsr(),
            rompath
        );
    }

    pub fn load_bios_rom(&mut self, rompath: String, rombytes: &Vec<u8>) -> Result<(), &'static str> {
        self.rompath = PathBuf::from(rompath.clone());
        self.biosrombytes = rombytes.clone();
        self.arm_core.load_bios_rom(&self.biosrombytes)
    }

    pub fn get_rom_bytes(&self) -> &Vec<u8> {
        &self.rombytes
    }

    pub fn get_status(&self) -> String {
        self.status_bar.clone()
    }

    pub fn get_core_state(&self) -> String {
        self.arm_core.print_state()
    }
}
