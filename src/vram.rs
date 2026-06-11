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
