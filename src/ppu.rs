use crate::{register::{Register8, Register16}, ram::Ram, vram::Oam};

pub enum PpuRegister {
    PpuCtrl,
    PpuMask,
    PpuStatus,
    OamAddr,
    OamData,
    PpuScroll,
    PpuAddr,
    PpuData,
    OamDma
}
pub struct Ppu {
    vram: Ram,
    oam: Oam,

    current_vram_address_register: Register16, // note: actually 15 bits
    tempoary_vram_address_register: Register16, // note: actually 15 bits,
    fine_x_scroll_register: Register8, // note: actually 3 bits,
    write_toggle_register: Register8, // note: actually 1 bit

    ppu_control_register: Register8, // note: only bits 2-7 are used
    ppu_mask_register: Register8,
    oam_address_register: Register8,
    io_bus: Register8
}

impl Ppu {
    // pub fn new() -> Self {
    //     return Ppu { 
    //         vram: Ram::new(),
    //         current_vram_address_register: Register16::new(), 
    //         tempoary_vram_address_register: Register16::new(), 
    //         fine_x_scroll_register: Register8::new(), 
    //         write_toggle_register: Register8::new(),
    //         io_bus: Register8::new()
    //     }
    // }

    pub fn write_register(&mut self, register: PpuRegister, value: u8) {
        match register {
            PpuRegister::PpuCtrl => self.write_ppu_control(value),
            PpuRegister::PpuMask => self.write_ppu_mask(value),
            PpuRegister::PpuStatus => (),
            PpuRegister::OamAddr => self.write_oam_addr(value),
            PpuRegister::OamData => self.write_oam_data(value),
            PpuRegister::PpuScroll => self.write_ppu_scroll(value),
            PpuRegister::PpuAddr => self.write_ppu_addr(value),
            
        }

        self.io_bus.write(value);
    }

    fn write_ppu_control(&mut self, value: u8) {
        self.tempoary_vram_address_register.write_bit(10, value & 1 == 1);
        self.tempoary_vram_address_register.write_bit(11, (value >> 1) & 1 == 1);

        self.ppu_control_register.write_bit(2, (value >> 2) & 1 == 1);
        self.ppu_control_register.write_bit(3, (value >> 3) & 1 == 1);
        self.ppu_control_register.write_bit(4, (value >> 4) & 1 == 1);
        self.ppu_control_register.write_bit(5, (value >> 5) & 1 == 1);
        self.ppu_control_register.write_bit(6, (value >> 6) & 1 == 1);
        self.ppu_control_register.write_bit(7, (value >> 7) & 1 == 1);
    }

    fn write_ppu_mask(&mut self, value: u8) {
        self.ppu_mask_register.write(value);
    }

    fn write_ppu_scroll(&mut self, value: u8) {
        if !self.write_toggle_register.read_bit(0) {
            self.tempoary_vram_address_register.write_bit(0, (value >> 3) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(1, (value >> 4) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(2, (value >> 5) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(3, (value >> 6) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(4, (value >> 7) & 1 == 1);
            self.fine_x_scroll_register.write_bit(0, value & 1 == 1);
            self.fine_x_scroll_register.write_bit(1, (value >> 1) & 1 == 1);
            self.fine_x_scroll_register.write_bit(2, (value >> 2) & 1 == 1);
            self.write_toggle_register.write(1);
        } else {
            self.tempoary_vram_address_register.write_bit(5, (value >> 3) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(6, (value >> 4) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(7, (value >> 5) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(8, (value >> 6) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(9, (value >> 7) & 1 == 1);

            self.tempoary_vram_address_register.write_bit(12, value & 1 == 1);
            self.tempoary_vram_address_register.write_bit(13, (value >> 1) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(14, (value >> 2) & 1 == 1);
            self.write_toggle_register.write(0);
        }
    }

    

    fn write_ppu_addr(&mut self, value: u8) {
        if !self.write_toggle_register.read_bit(0) {
            self.tempoary_vram_address_register.write_bit(8, value & 1 == 1);
            self.tempoary_vram_address_register.write_bit(9, (value >> 1) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(10, (value >> 2) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(11, (value >> 3) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(12, (value >> 4) & 1 == 1);
            self.tempoary_vram_address_register.write_bit(13, (value >> 5) & 1 == 1);
            self.tempoary_vram_address_register.unset_bit(14);
            self.write_toggle_register.write(1);
        } else {
            self.tempoary_vram_address_register.write_low(value);
            self.write_toggle_register.write(0);

            // mystery transfer t to v?
        }
    }

    fn write_oam_addr(&mut self, value: u8) {
        self.oam_address_register.write(value);
    }

    fn write_oam_data(&mut self, value: u8) {
        self.oam.write(self.oam_address_register.read(), value);
        self.oam_address_register.increment();
    }

    pub fn read_register(&mut self, register: PpuRegister) -> u8 {
        return self.io_bus.read();
    }

    fn read_ppu_status(&mut self) {

    }

    pub fn cycle() {

    }

    fn read(&mut self, low_byte: u8, high_byte: u8) -> u8 {
        let mut value = 0;

        // do some magic shit
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        // do some more magic shit
    }
}