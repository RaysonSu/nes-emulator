struct Ram {
    values: [u8; 0x800]
}

impl Ram {
    fn new() -> Ram {
        return Ram { values: [0; 0x800] }
    }

    fn read(&self, low_byte: u8, high_byte: u8) -> u8 {
        let mut addr: usize = low_byte as usize;
        addr |= (high_byte as usize) << 8; // pack low and high bytes together
        addr &= 0x7ff; // ensure correct mirroring;
        return self.values[addr];
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        let mut addr: usize = low_byte as usize; 
        addr |= (high_byte as usize) << 8; // pack low and high bytes together
        addr &= 0x7ff; // ensure correct mirroring;
        
        self.values[addr] = value;
    }
}