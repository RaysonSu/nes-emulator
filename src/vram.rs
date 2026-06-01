pub struct VRam {
    values: [u8; 0x1000]
}

impl VRam {
    pub fn new() -> VRam {
        return VRam { values: [0; 0x1000] }
    }

    pub fn read(&self, low_byte: u8, high_byte: u8) -> u8 {
        let mut addr: usize = low_byte as usize;
        addr |= (high_byte as usize) << 8; // pack low and high bytes together
        addr &= 0x7ff; // ensure correct mirroring;
        return self.values[addr];
    }

    pub fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        let mut addr: usize = low_byte as usize; 
        addr |= (high_byte as usize) << 8; // pack low and high bytes together
        addr &= 0x7ff; // ensure correct mirroring;
        
        self.values[addr] = value;
    }
}
