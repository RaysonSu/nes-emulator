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

pub enum SpriteEvaluationState {
    ReadSpriteYCoordinate,
    WriteSpriteYCoordinate,
    ReadSpriteTile,
    WriteSpriteTile,
    ReadSpriteAttributes,
    WriteSpriteAttributes,
    ReadSpriteXCoordinate,
    WriteSpriteXCoordinate,
    ReadOverflowCheckedValue,
    OverflowSecondCycle,
    OverflowThirdCycle,
    OverflowFourthCycle,
    OverflowFifthCycle,
    OverflowSixthCycle,
    OverflowSeventhCycle,
    OverflowEighthCycle,
    ReadFinished,
    WriteFinished
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

    transfer_t_to_v: bool,

    ppu_control_register: Register8, // note: only bits 2-7 are used
    ppu_mask_register: Register8,
    oam_address_register: Register8,
    ppu_status_register: Register8, // note: only bits 5-7 are used
    io_bus: Register8,

    nametable_byte: u8,
    attribute_table_byte: u8,
    pattern_table_tile_low: u8,
    pattern_table_tile_high: u8,

    sprite_evaluation_state: SpriteEvaluationState,
    found_sprites_count: u8,

    scanline: u16,
    cycle_count: u16,
    frame: u64
}

// exposed functions
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

        if self.transfer_t_to_v {
            self.transfer_t_to_v = false;
            self.current_vram_address_register.write(self.tempoary_vram_address_register.read());
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
            PpuRegister::PpuData => self.write_ppu_data(value), 
            PpuRegister::OamDma => todo!()
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
            PpuRegister::PpuData => self.read_ppu_data(),
            PpuRegister::OamDma => todo!()
        }

        return self.io_bus.read();
    }

    pub fn connect_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.cartridge = Some(cartridge)
    }
}

// make ppu do shit!
impl Ppu {
    fn prepare_render(&mut self) {
        if self.scanline >= 240 { return; } // non-visible scanline
        if self.cycle_count == 0 { return; }
        else if self.cycle_count <= 336 {
            let low_byte = todo!("figure out which address point to?");
            let high_byte = todo!("figure out which address point to?");

            let value = self.read(low_byte, high_byte);

            
            match self.cycle_count % 8 {
                2 => { self.nametable_byte = value }, // nametable byte
                4 => { self.attribute_table_byte = value }, // attribute table byte
                6 => { self.pattern_table_tile_low = value }, // pattern table tile low
                0 => { self.pattern_table_tile_high = value }, // pattern table tile high
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
        if self.cycle_count == 0 { 
            todo!(); 
        } else if self.cycle_count == 1 { 
            self.oam.make_unreadable();
        } else if self.cycle_count <= 64 {
            if self.cycle_count % 2 == 0 { 
                self.oam.write((self.cycle_count / 2 - 1) as u8, 0xff); 
            }
        } else if self.cycle_count == 65 { 
            self.oam.make_readable();
            self.sprite_evaluation_state = SpriteEvaluationState::ReadSpriteYCoordinate;
            self.oam_address_register.write(0);
            self.found_sprites_count = 0;

            self.cycle_sprite_evaluation();
        } else if self.cycle_count <= 256 { 
            self.cycle_sprite_evaluation(); 
        } else if self.cycle_count <= 340 { 
            return; // does a bunch of reading?
        } else {
            panic!("Cycle count should never be above 340.");
        }
    }

    fn draw_to_screen(&mut self) {
        todo!("actually implement drawing to screen")
    }
}

// tile fetching (during rendering)
impl Ppu {
    
}

// sprite eval stuff
impl Ppu {
    fn cycle_sprite_evaluation(&mut self) {
        self.sprite_evaluation_state = match self.sprite_evaluation_state {
            SpriteEvaluationState::ReadSpriteYCoordinate => SpriteEvaluationState::WriteSpriteYCoordinate,
            SpriteEvaluationState::WriteSpriteYCoordinate => self.write_sprite_y_coodinate(),
            SpriteEvaluationState::ReadSpriteTile => SpriteEvaluationState::WriteSpriteTile,
            SpriteEvaluationState::WriteSpriteTile => self.write_sprite_tile(),
            SpriteEvaluationState::ReadSpriteAttributes => SpriteEvaluationState::WriteSpriteAttributes,
            SpriteEvaluationState::WriteSpriteAttributes => self.write_sprite_attributes(),
            SpriteEvaluationState::ReadSpriteXCoordinate => SpriteEvaluationState::WriteSpriteXCoordinate,
            SpriteEvaluationState::WriteSpriteXCoordinate => self.write_sprite_x_coordinate(),
            SpriteEvaluationState::ReadOverflowCheckedValue => self.process_overflow(),
            SpriteEvaluationState::OverflowSecondCycle => SpriteEvaluationState::OverflowThirdCycle,
            SpriteEvaluationState::OverflowThirdCycle => self.process_overflow_third_cycle(),
            SpriteEvaluationState::OverflowFourthCycle => SpriteEvaluationState::OverflowFifthCycle,
            SpriteEvaluationState::OverflowFifthCycle => self.process_overflow_fifth_cycle(),
            SpriteEvaluationState::OverflowSixthCycle => SpriteEvaluationState::OverflowSeventhCycle,
            SpriteEvaluationState::OverflowSeventhCycle => self.process_overflow_seventh_cycle(),
            SpriteEvaluationState::OverflowEighthCycle => SpriteEvaluationState::ReadOverflowCheckedValue,
            SpriteEvaluationState::ReadFinished => SpriteEvaluationState::WriteFinished,
            SpriteEvaluationState::WriteFinished => self.write_finished()
        }
    }

    fn is_y_coordinate_in_range(&self, y_coordinate: u8) -> bool {
        // TODO: check this
        let sprite_height = if self.ppu_control_register.read_bit(5) { 16 } else { 8 };

        let lower_bound = y_coordinate as u16;
        let upper_bound = y_coordinate as u16 + sprite_height;

        return lower_bound <= self.scanline && self.scanline < upper_bound; 
    }

    fn write_sprite_y_coodinate(&mut self) -> SpriteEvaluationState {
        let oam_address = self.oam_address_register.read();
        let y_coordinate = self.oam.read(oam_address);
        if self.found_sprites_count < 8 {
            self.secondary_oam.write(self.found_sprites_count * 4, y_coordinate);
        }

        if self.is_y_coordinate_in_range(y_coordinate) {
            self.oam_address_register.increment();
            return SpriteEvaluationState::ReadSpriteTile; // in range, we can now exit
        }

        self.oam_address_register.write(oam_address.wrapping_add(4));
        return self.go_to_next_sprite();    
    }

    fn write_sprite_tile(&mut self) -> SpriteEvaluationState {
        let oam_address = self.oam_address_register.read();
        let tile = self.oam.read(oam_address);
        if self.found_sprites_count < 8 {
            self.secondary_oam.write(self.found_sprites_count * 4 + 1, tile);
        }

        self.oam_address_register.increment();
        return SpriteEvaluationState::ReadSpriteAttributes;
    }

    fn write_sprite_attributes(&mut self) -> SpriteEvaluationState {
        let oam_address = self.oam_address_register.read();
        let attributes = self.oam.read(oam_address);
        if self.found_sprites_count < 8 {
            self.secondary_oam.write(self.found_sprites_count * 4 + 2, attributes);
        }

        self.oam_address_register.increment();
        return SpriteEvaluationState::ReadSpriteXCoordinate;
    }

    fn write_sprite_x_coordinate(&mut self) -> SpriteEvaluationState {
        let oam_address = self.oam_address_register.read();
        let x_coordinate = self.oam.read(oam_address);
        if self.found_sprites_count < 8 {
            self.secondary_oam.write(self.found_sprites_count * 4 + 3, x_coordinate);
        }

        self.oam_address_register.increment();
        return self.go_to_next_sprite();
    }

    fn write_finished(&mut self) -> SpriteEvaluationState {
        let oam_address = self.oam_address_register.read();
        let garbage = self.oam.read(oam_address);
        if self.found_sprites_count < 8 {
            self.secondary_oam.write(self.found_sprites_count * 4, garbage);
        }

        self.oam_address_register.write(oam_address.wrapping_add(4));

        return SpriteEvaluationState::ReadFinished;
    }

    fn go_to_next_sprite(&mut self) -> SpriteEvaluationState {        
        if self.oam_address_register.read() == 0 { // wrapping
            return SpriteEvaluationState::ReadFinished;
        }

        if self.found_sprites_count < 8 { // we're fine, no overflow yet!
            return SpriteEvaluationState::ReadSpriteYCoordinate;
        }

        return SpriteEvaluationState::ReadOverflowCheckedValue;
    }

    fn process_overflow(&mut self) -> SpriteEvaluationState {
        let mut oam_address = self.oam_address_register.read();
        let value = self.oam.read(oam_address);
        
        if self.is_y_coordinate_in_range(value) {
            self.ppu_status_register.set_bit(5); // set sprite overflow flag
            
            self.oam_address_register.increment();

            return SpriteEvaluationState::OverflowSecondCycle;
        }
        
        // note: this is a ppu bug, see https://www.nesdev.org/wiki/PPU_sprite_evaluation
        if oam_address & 0x03 == 0x03 {
            oam_address &= 0xfc;
            oam_address = oam_address.wrapping_add(4);
        }  else {
            oam_address = oam_address.wrapping_add(5);
        }

        self.oam_address_register.write(oam_address);

        if oam_address < 0x04 {
            return SpriteEvaluationState::WriteFinished;
        } else {
            return SpriteEvaluationState::OverflowEighthCycle;
        }
    }

    fn process_overflow_third_cycle(&mut self) -> SpriteEvaluationState {
        self.oam_address_register.increment();
        return SpriteEvaluationState::OverflowFourthCycle;
    }

    fn process_overflow_fifth_cycle(&mut self) -> SpriteEvaluationState {
        self.oam_address_register.increment();
        return SpriteEvaluationState::OverflowSixthCycle;
    }

    fn process_overflow_seventh_cycle(&mut self) -> SpriteEvaluationState {
        self.oam_address_register.increment();
        return SpriteEvaluationState::OverflowEighthCycle;
    }
}

// read/write (internal)
impl Ppu {
    fn read(&mut self, low_byte: u8, high_byte: u8) -> u8 {
        let value;
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

// read/write memory mapped registers
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

            self.transfer_t_to_v = true;
        }
    }

    fn write_ppu_data(&mut self, value: u8) {
        self.write(self.current_vram_address_register.read_low(), self.current_vram_address_register.read_high(), value);
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

    fn read_ppu_data(&mut self) {
        let data = self.read(self.current_vram_address_register.read_low(), self.current_vram_address_register.read_high());
        
        self.io_bus.write(data);
    }

    fn read_oam_data(&mut self) {
        let address = self.oam_address_register.read();
        
        self.io_bus.write(self.oam.read(address));
    }
}
