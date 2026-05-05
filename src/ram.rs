pub struct Ram {
    values: [u8; 0x800]
}

impl Ram {
    pub fn new() -> Ram {
        return Ram { values: [0; 0x800] }
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ram() {
        let mut ram = Ram::new();
        
        // test ram startup state
        assert_eq!(ram.read(0x12, 0x34), 0);
        assert_eq!(ram.read(0x34, 0x56), 0);

        // test writing to ram
        ram.write(0x12, 0x02, 0x37);
        assert_eq!(ram.read(0x12, 0x02), 0x37);
        
        // test read from ram mirroring
        assert_eq!(ram.read(0x12, 0x0a), 0x37);
        assert_eq!(ram.read(0x12, 0x12), 0x37);
        assert_eq!(ram.read(0x12, 0x1a), 0x37);

        // test write to ram mirroring
        ram.write(0x13, 0x09, 0x42);
        assert_eq!(ram.read(0x13, 0x01), 0x42);

        ram.write(0x14, 0x11, 0x43);
        assert_eq!(ram.read(0x14, 0x01), 0x43);

        ram.write(0x15, 0x19, 0x44);
        assert_eq!(ram.read(0x15, 0x01), 0x44);

        // test over writing values from ram
        ram.write(0x15, 0x09, 0x01);
        assert_eq!(ram.read(0x15, 0x01), 0x01);
    }
}