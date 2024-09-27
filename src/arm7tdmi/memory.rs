pub struct Memory {
    print_cursor: usize,
    bios_rom: [u8; 16384],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            print_cursor: 0usize,
            bios_rom: [0u8; 16384],
        }
    }
}

impl Memory {
    pub fn load_bios_rom(&mut self, bios_bytes: &Vec<u8>) -> Result<(), &'static str> {
        if bios_bytes.len() != 16384usize {
            return Err("Bios image incorrect size: Expected 16384");
        }

        println!("Loading bios rom");
        self.bios_rom.clone_from_slice(bios_bytes.as_slice());
        Ok(())
    }

    pub fn get_byte(&self, address: usize) -> u8 {
        // TODO: pull from different memory regions based on address?
        self.bios_rom[address]
    }

    pub fn get_halfword(&self, address: usize) -> u16 {
        // TODO: pull from different memory regions
        let address = address & (!1usize); // Mask off lowest bit to ensure alignment
        ((self.bios_rom[address + 1] as u16) << 8) | (self.bios_rom[address] as u16)
    }

    pub fn get_word(&self, address: usize) -> u32 {
        // TODO: pull from different memory regions
        let address = address & (!3usize); // Mask off lowest two bits to ensure alignment
        ((self.bios_rom[address + 3] as u32) << 24)
            | ((self.bios_rom[address + 2] as u32) << 16)
            | ((self.bios_rom[address + 1] as u32) << 8)
            | (self.bios_rom[address] as u32)
    }

    pub fn advance_mem_cursor(&mut self) {
        self.print_cursor = self.print_cursor.saturating_add(8);
    }

    pub fn regress_mem_cursor(&mut self) {
        self.print_cursor = self.print_cursor.saturating_sub(8);
    }

    pub fn print_memory(&self, num_bytes: usize) -> String {
        let mut ret_str: String = String::new();
        let num_lines = num_bytes / 8;
        ret_str.push_str("             07 06 05 04 03 02 01 00\n");
        ret_str.push_str("-----------|-------------------------|\n");

        for line_offset in 0..num_lines {
            ret_str.push_str(
                format!(
                    "{:#010x} | {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} |\n",
                    line_offset * 8 + self.print_cursor,
                    self.get_byte(line_offset * 8 + self.print_cursor + 7),
                    self.get_byte(line_offset * 8 + self.print_cursor + 6),
                    self.get_byte(line_offset * 8 + self.print_cursor + 5),
                    self.get_byte(line_offset * 8 + self.print_cursor + 4),
                    self.get_byte(line_offset * 8 + self.print_cursor + 3),
                    self.get_byte(line_offset * 8 + self.print_cursor + 2),
                    self.get_byte(line_offset * 8 + self.print_cursor + 1),
                    self.get_byte(line_offset * 8 + self.print_cursor + 0),
                )
                .as_str(),
            );
        }

        ret_str.push_str("-----------|-------------------------|\n");

        ret_str
    }
}
