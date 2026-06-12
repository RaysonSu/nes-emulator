pub struct Oam {
    values: [u8; 0x100]
}

impl Oam {
    pub fn new() -> Oam {
        return Oam { values: [0; 0x100] }
    }

    pub fn read(&self, address: u8) -> u8 {
        return self.values[address as usize];
    }

    pub fn write(&mut self, address: u8, value: u8) {
        self.values[address as usize] = value;
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
        return self.values[(address as usize) & 0x1f];
    }

    pub fn write(&mut self, address: u8, value: u8) {
        self.values[(address as usize) & 0x1f] = value;
    }
}
