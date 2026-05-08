pub struct Register8 {
    value: u8
}

pub struct Register16 {
    value: u16
}

impl Register8 {
    pub fn new() -> Register8 {
        return Register8 { value: 0 };
    }

    pub fn read(&self) -> u8 {
        return self.value;
    }

    pub fn read_bit(&self, bit: u8) -> bool {
        return (self.value >> bit) & 1 == 1;
    }

    pub fn write(&mut self, value: u8) {
        self.value = value;
    }

    pub fn write_bit(&mut self, bit: u8, value: bool) {
        self.unset_bit(bit);
        self.value |= (value as u8) << bit;
    }

    pub fn set_bit(&mut self, bit: u8) {
        self.value |= 1 << bit;
    }

    pub fn unset_bit(&mut self, bit: u8) {
        self.value &= !(1 << bit);
    }

    pub fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }
}

impl Register16 {
    pub fn new() -> Register16 {
        return Register16 { value: 0 };
    }

    pub fn read(&self) -> u16 {
        return self.value;
    }

    pub fn read_bit(&self, bit: u8) -> bool {
        return (self.value >> bit) & 1 == 1;
    }

    pub fn read_low(&self) -> u8 {
        return self.value as u8;
    }

    pub fn read_high(&self) -> u8 {
        return (self.value >> 8) as u8;
    }

    pub fn write(&mut self, value: u16) {
        self.value = value;
    }

    pub fn write_low(&mut self, value: u8) {
        self.value = self.value & 0xff00 | (value as u16);
    }

    pub fn write_high(&mut self, value: u8) {
        self.value = self.value & 0xff | (value as u16) << 8;
    }

    pub fn write_bit(&mut self, bit: u8, value: bool) {
        self.unset_bit(bit);
        self.value |= (value as u16) << bit;
    }

    pub fn set_bit(&mut self, bit: u8) {
        self.value |= 1 << bit;
    }

    pub fn unset_bit(&mut self, bit: u8) {
        self.value &= !(1 << bit);
    }

    pub fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    pub fn increment_low(&mut self) {
        self.value = (self.value & 0xff00) + (self.value.wrapping_add(1) & 0x00ff);
    }

    pub fn increment_high(&mut self) {
        self.value = self.value.wrapping_add(0x0100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_8() {
        let mut reg = Register8::new();

        assert_eq!(reg.read(), 0x00);

        reg.write(0x42);
        assert_eq!(reg.read(), 0x42);

        reg.set_bit(5);
        assert_eq!(reg.read(), 0x62);

        reg.set_bit(5);
        assert_eq!(reg.read(), 0x62);

        reg.unset_bit(6);
        assert_eq!(reg.read(), 0x22);
        
        reg.unset_bit(6);
        assert_eq!(reg.read(), 0x22);

        reg.write_bit(7, true);
        assert_eq!(reg.read(), 0xa2);
        assert_eq!(reg.read_bit(7), true);

        reg.write_bit(7, false);
        assert_eq!(reg.read(), 0x22);
        assert_eq!(reg.read_bit(7), false);

        reg.increment();
        assert_eq!(reg.read(), 0x23);

        reg.write(0xff);
        reg.increment();
        assert_eq!(reg.read(), 0x00);
    }

    #[test]
    fn test_register_16() {
        let mut reg = Register16::new();

        assert_eq!(reg.read(), 0x0000);

        reg.write(0x1242);
        assert_eq!(reg.read(), 0x1242);

        reg.set_bit(14);
        assert_eq!(reg.read(), 0x5242);

        reg.set_bit(14);
        assert_eq!(reg.read(), 0x5242);

        reg.unset_bit(1);
        assert_eq!(reg.read(), 0x5240);
        
        reg.unset_bit(1);
        assert_eq!(reg.read(), 0x5240);

        assert_eq!(reg.read_low(), 0x40);
        assert_eq!(reg.read_high(), 0x52);
        
        reg.write_low(0x37);
        assert_eq!(reg.read(), 0x5237);

        reg.write_high(0x44);
        assert_eq!(reg.read(), 0x4437);

        reg.increment();
        assert_eq!(reg.read(), 0x4438);

        reg.write(0xffff);
        reg.increment();
        assert_eq!(reg.read(), 0x0000);

        reg.write(0x4243);
        reg.increment_low();
        assert_eq!(reg.read(), 0x4244);
    
        reg.write(0x42ff);
        reg.increment_low();
        assert_eq!(reg.read(), 0x4200);
    
        reg.write(0x1234);
        reg.increment_high();
        assert_eq!(reg.read(), 0x1334);
        
        reg.write(0xff32);
        reg.increment_high();
        assert_eq!(reg.read(), 0x0032);

        reg.write_bit(12, true);
        assert_eq!(reg.read(), 0x0800);
        assert_eq!(reg.read_bit(12), true);

        reg.write_bit(12, false);
        assert_eq!(reg.read(), 0x0000);
        assert_eq!(reg.read_bit(12), false);
    }
}