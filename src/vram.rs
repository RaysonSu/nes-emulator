pub struct Oam {
    values: [u8; 0x100],
    readable: bool
}

impl Oam {
    pub fn new() -> Oam {
        return Oam { 
            values: [0; 0x100],
            readable: true
        }
    }

    pub fn read(&self, address: u8) -> u8 {
        if self.readable {
            return self.values[address as usize];
        } else {
            return 0xff;
        }
    }

    pub fn write(&mut self, address: u8, value: u8) {
        self.values[address as usize] = value;
    }

    pub fn make_readable(&mut self) {
        self.readable = true;
    }

    pub fn make_unreadable(&mut self) {
        self.readable = false;
    }
}

pub struct SecondaryOam {
    values: [u8; 0x20]
}

impl SecondaryOam {
    pub fn new() -> SecondaryOam {
        return SecondaryOam { values: [0; 0x20] }
    }

    pub fn read(&self, address: u8) -> u8 {
        return self.values[address as usize];
    }

    pub fn write(&mut self, address: u8, value: u8) {
        self.values[address as usize] = value;
    }
}

pub struct PaletteRam {
    values: [u8; 0x20]
}

impl PaletteRam {
    pub fn new() -> PaletteRam {
        return PaletteRam { values: [0; 0x20] }
    }

    pub fn read(&self, address: u8) -> u8 {
        // ensure reading 0x3f00 is the same as 0x3f10
        let mask = ((address & 0x0f != 0x00) as usize) << 4;

        return self.values[(address as usize) & 0x1f & mask];
    }

    pub fn write(&mut self, address: u8, value: u8) {
        let mask = ((address & 0x0f != 0x00) as usize) << 4;

        self.values[(address as usize) & 0x1f & mask] = value;
    }
}
