pub struct Cartridge {
    program_ram: [u8; 0x2000],
    program_rom: [u8; 0x8000],
    character_rom: [u8; 0x2000],
}

impl Cartridge {
    pub fn read_cpu_mapped(&self, low_byte: u8, high_byte: u8) -> u8 {
        let addr = ((high_byte as usize) << 8) + (low_byte as usize);

        if addr < 0x6000 {
            panic!("Cannot read from cartridge at memory address ${:x}", addr);
        } else if addr < 0x8000 {
            return self.program_ram[addr - 0x6000];
        } else {
            return self.program_rom[addr - 0x8000];
        }
    }

    pub fn write_cpu_mapped(&self, low_byte: u8, high_byte: u8, value: u8) {
        let addr = ((high_byte as usize) << 8) + (low_byte as usize);

        if addr < 0x6000 {
            panic!("Cannot write to cartridge at memory address ${:x}", addr);
        } else if addr < 0x8000 {
            self.program_ram[addr - 0x6000] = value;
        }
    }

    pub fn read_ppu_mapped(&self, low_byte: u8, high_byte: u8) -> u8 {
        let addr = ((high_byte as usize) << 8) + (low_byte as usize);

        if addr >= 0x2000  {
            panic!("Cannot read from cartridge at ppu memory address ${:x}", addr);
        } else {
            return self.program_ram[addr];
        }
    }
}