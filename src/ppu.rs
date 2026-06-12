use std::rc::Rc;
use crate::{cartridge::Cartridge, ram::Ram, register::{Register8, Register16}, vram::{Oam, PaletteRam, SecondaryOam}};

#[derive(Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical
}

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
    secondary_oam: SecondaryOam,
    pallet_ram: PaletteRam,

    cartridge: Option<Rc<Cartridge>>,

    current_vram_address_register: Register16, // note: actually 15 bits
    tempoary_vram_address_register: Register16, // note: actually 15 bits,
    fine_x_scroll_register: Register8, // note: actually 3 bits,
    write_toggle_register: Register8, // note: actually 1 bit

    ppu_control_register: Register8, // note: only bits 2-7 are used
    ppu_mask_register: Register8,
    oam_address_register: Register8,
    ppu_status_register: Register8, // note: only bits 5-7 are used
    io_bus: Register8,

    nametable_byte: u8,
    attribute_table_byte: u8,
    pattern_table_tile_low: u8,
    pattern_table_tile_high: u8,
    // secondary_oam_open_slot: u8,

    scanline: u16,
    cycle_count: u16,
    frame: u64
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

    pub fn cycle(&mut self) {
        // prepare render
        self.prepare_render();
        
        // update secondary oam
        self.update_secondary_oam();
        
        // actually draw stuff
        self.draw_to_screen();

        self.cycle_count += 1;
        if self.cycle_count > 341 {
            self.scanline += 1;
        }

        if self.scanline > 261 {
            self.scanline = 0;
            self.frame += 1;
        }
    }

    pub fn write_register(&mut self, register: PpuRegister, value: u8) {
        match register {
            PpuRegister::PpuCtrl => self.write_ppu_control(value),
            PpuRegister::PpuMask => self.write_ppu_mask(value),
            PpuRegister::PpuStatus => (),
            PpuRegister::OamAddr => self.write_oam_addr(value),
            PpuRegister::OamData => self.write_oam_data(value),
            PpuRegister::PpuScroll => self.write_ppu_scroll(value),
            PpuRegister::PpuAddr => self.write_ppu_addr(value),
            PpuRegister::PpuData | PpuRegister::OamDma => todo!()
        }

        self.io_bus.write(value);
    }

    pub fn read_register(&mut self, register: PpuRegister) -> u8 {
        match register {
            PpuRegister::PpuCtrl => (),
            PpuRegister::PpuMask => (),
            PpuRegister::PpuStatus => self.read_ppu_status(),
            PpuRegister::OamAddr => (),
            PpuRegister::OamData => self.read_oam_data(),
            PpuRegister::PpuScroll => (),
            PpuRegister::PpuAddr => (),
            PpuRegister::PpuData | PpuRegister::OamDma => todo!()
        }

        return self.io_bus.read();
    }

    pub fn connect_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.cartridge = Some(cartridge)
    }
}

impl Ppu {
    fn prepare_render(&mut self) {
        if self.scanline >= 240 { return; } // non-visible scanline
        if self.cycle_count == 0 { return; }
        else if self.cycle_count <= 336 {
            match self.cycle_count % 8 {
                2 => todo!(), // nametable byte
                4 => todo!(), // attribute table byte
                6 => todo!(), // pattern table tile low
                0 => todo!(), // pattern table tile high
                _ => (),
            };
        } else if self.cycle_count <= 340 {
            todo!();
        } else {
            panic!("Cycle count should never be above 340.");
        }
    }

    fn update_secondary_oam(&mut self) {
        if self.scanline >= 240 { return; } // non-visible scanline
        if self.cycle_count == 0 { todo!(); }
        else if self.cycle_count == 1 { // not actually true, but oh well
            for address in 0..64 {
                self.secondary_oam.write(address, 0xff);
            }
            // self.secondary_oam_open_slot = 0;
        } else if self.cycle_count <= 64 { return; }
        else if self.cycle_count == 65 { // also not actually true, but also oh well 
            let mut sprite_number = 0;
            let mut oam_byte = 0;
            let mut rendered_sprites = 0;
            let sprite_height = if self.ppu_control_register.read_bit(5) { 16 } else { 8 };

            while sprite_number < 64 {
                let y_coordinate = self.oam.read(sprite_number * 4) as u16;
                
                if y_coordinate <= self.scanline && self.scanline < y_coordinate + sprite_height {
                    self.secondary_oam.write(rendered_sprites * 4, self.oam.read(sprite_number * 4));
                    self.secondary_oam.write(rendered_sprites * 4 + 1, self.oam.read(sprite_number * 4 + 1));
                    self.secondary_oam.write(rendered_sprites * 4 + 2, self.oam.read(sprite_number * 4 + 2));
                    self.secondary_oam.write(rendered_sprites * 4 + 3, self.oam.read(sprite_number * 4 + 3));
                }
            }
        }
    }
}

impl Ppu {
    fn read(&mut self, low_byte: u8, high_byte: u8) -> u8 {
        let mut value = 0;
        let cartridge = self.cartridge.as_ref().expect("cartridge should be connected to ppu before ppu can read");

        if high_byte < 0x20 {
            value = cartridge.read_ppu_mapped(low_byte, high_byte);
        } else if high_byte < 0x3f {
            let fixed_high_byte = match cartridge.get_mirroring() {
                Mirroring::Horizontal => high_byte & 0x03 | (high_byte >> 1) & 0x04, 
                Mirroring::Vertical => high_byte & 0x07
            };

            value = self.vram.read(low_byte, fixed_high_byte);
        } else if high_byte == 0x3f {
            value = self.pallet_ram.read(low_byte);
        } else {
            panic!("Cannot read from ppu memory address ${:x}", ((high_byte as u16) << 8) | (low_byte as u16));
        }

        return value;
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        let cartridge = self.cartridge.as_ref().expect("cartridge should be connected to ppu before ppu can read");

        if high_byte < 0x20 {
            // write to rom = no effect
            return;
        } else if high_byte < 0x3f {
            let fixed_high_byte = match cartridge.get_mirroring() {
                Mirroring::Horizontal => high_byte & 0x03 | (high_byte >> 1) & 0x04, 
                Mirroring::Vertical => high_byte & 0x07
            };

            self.vram.write(low_byte, fixed_high_byte, value);
        } else if high_byte == 0x3f {
            self.pallet_ram.write(low_byte, value);
        } else {
            panic!("Cannot write to ppu memory address ${:x}", ((high_byte as u16) << 8) | (low_byte as u16));
        }
    }
}

impl Ppu {
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

    fn read_ppu_status(&mut self) {
        let status = self.ppu_status_register.read();
        self.io_bus.write_bit(5, (status >> 5) & 1 == 1);
        self.io_bus.write_bit(6, (status >> 6) & 1 == 1);
        self.io_bus.write_bit(7, (status >> 7) & 1 == 1);
    }

    fn read_oam_data(&mut self) {
        let address = self.oam_address_register.read();

        self.io_bus.write(self.oam.read(address));
    }
}
