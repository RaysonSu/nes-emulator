use crate::{register::{Register8, Register16}, vram::VRam};

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
    ppu_control_register: Register8,
    ppu_mask_register: Register8,
    ppu_status_register: Register8,
    oam_address_register: Register8,
    oam_data_register: Register8,
    ppu_scroll_register: Register16,
    ppu_address_register: Register16,
    ppu_data_register: Register8,
    oam_dma_register: Register8,
    current_vram_address_register: Register16, // note: actually 15 bits
    tempoary_vram_address_register: Register16, // note: actually 15 bits,
    fine_x_scroll_register: Register8, // note: actually 3 bits,
    write_toggle_register: Register8, // note: actually 1 bit
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu { 
            vram: H
            ppu_control_register: Register8::new(),
            ppu_mask_register: Register8::new(), 
            ppu_status_register: Register8::new(), 
            oam_address_register: Register8::new(), 
            oam_data_register: Register8::new(), 
            ppu_scroll_register: Register16::new(), 
            ppu_address_register: Register16::new(), 
            ppu_data_register: Register8::new(), 
            oam_dma_register: Register8::new(), 
            current_vram_address_register: Register16::new(), 
            tempoary_vram_address_register: Register16::new(), 
            fine_x_scroll_register: Register8::new(), 
            write_toggle_register: Register8::new() 
        }
    }

    pub fn write_register(&mut self, register: PpuRegister, value: u8) {
        
    }

    pub fn read_register(&mut self, register: PpuRegister) -> u8 {
        return 0;
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