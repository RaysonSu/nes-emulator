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

enum SpriteEvaluationState {
    ReadSpriteYCoordinate,
    WriteSpriteYCoordinate,
    ReadSpriteTile,
    WriteSpriteTile,
    ReadSpriteAttributes,
    WriteSpriteAttributes,
    ReadSpriteXCoordinate,
    WriteSpriteXCoordinate,
    ReadOverflowCheckedValue,
    OverflowCycle(u8),
    ReadFinished,
    WriteFinished
}

#[derive(Clone, Copy)]
struct TileData {
    tile_number: u8,
    attribute_table_byte: u8,
    pattern_table_tile_low: u8,
    pattern_table_tile_high: u8,
}

pub struct Ppu {
    vram: Ram,
    oam: Oam,
    secondary_oam: SecondaryOam,
    pallet_ram: PaletteRam,

    cartridge: Option<Rc<Cartridge>>,

    current_vram_address_register: Register16, // note: actually 15 bits
    temporary_vram_address_register: Register16, // note: actually 15 bits,
    fine_x_scroll_register: Register8, // note: actually 3 bits,
    write_toggle_register: Register8, // note: actually 1 bit

    transfer_t_to_v: bool,

    ppu_control_register: Register8, // note: only bits 2-7 are used
    ppu_mask_register: Register8,
    oam_address_register: Register8,
    ppu_status_register: Register8, // note: only bits 5-7 are used
    io_bus: Register8,

    temporary_tile_data: TileData,
    next_tile_data: TileData,
    current_tile_data: TileData,

    sprite_evaluation_state: SpriteEvaluationState,
    found_sprites_count: u8,

    scanline: u16,
    cycle_count: u16,
    frame: u64,

    initialising: bool
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

        // handle misc flags
        self.handle_flags();

        self.cycle_count += 1;
        if self.cycle_count > 341 {
            self.scanline += 1;
        }

        if self.scanline > 261 {
            self.scanline = 0;
            self.frame += 1;

            if self.frame % 2 == 0 {
                self.cycle_count += 1;
            }
        }

        if self.transfer_t_to_v {
            self.transfer_t_to_v = false;
            self.current_vram_address_register.write(self.temporary_vram_address_register.read());
        }
    }

    pub fn reset(&mut self) {
        todo!("implement reset")
    }

    pub fn power_up(&mut self) {
        todo!("implement powering up")
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
        if self.scanline >= 240 && self.scanline != 261 { // non-visible scanline
            return; 
        } else if self.cycle_count == 0 {
            return; 
        } else if self.cycle_count <= 255 || (self.cycle_count >= 321 && self.cycle_count <= 336) {
            match self.cycle_count % 8 {
                2 => self.fetch_tile_number(), // nametable byte
                4 => self.fetch_attribute_byte(), // attribute table byte
                6 => self.fetch_pattern_table_low_byte(), // pattern table tile low
                0 => { // pattern table tile high
                    self.fetch_pattern_table_high_byte();
                    self.coarse_increment_horizontal_position();
                    self.shift_tile_data();
                }, _ => return
            };
        } else if self.cycle_count == 256 {
            self.fetch_pattern_table_high_byte();
            self.increment_vertical_position();
        } else if self.cycle_count == 257 {
            self.copy_horizontal_position_into_v();
        } else if self.cycle_count == 258 {
            self.fetch_tile_number();
        } else if self.cycle_count <= 265 {
            return;
        } else if self.cycle_count == 266 { 
            self.fetch_tile_number(); 
        } else if self.cycle_count <= 279 {
            return;
        } else if self.cycle_count <= 304 {
            if self.scanline == 261 {
                self.copy_vertical_position_into_v();
            } else {
                return;
            } 
        } else if self.cycle_count == 305 {
            return;
        } else if self.cycle_count == 306 {
            self.fetch_tile_number();
        } else if self.cycle_count <= 320 {
            return;
        } else if self.cycle_count == 338 {
            self.fetch_tile_number();
        } else if self.cycle_count <= 340 {
            return;
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

    fn handle_flags(&mut self) {
        if self.scanline == 241 && self.cycle_count == 1 {
            self.ppu_status_register.set_bit(7);
        }

        if self.scanline == 261 && self.cycle_count == 1 {
            self.ppu_status_register.write(0);
        }
    }
}

// ppu scrolling stuff
impl Ppu {    
    fn increment_vertical_position(&mut self) {
        let mut v = self.current_vram_address_register.read();

        // basically copied from https://www.nesdev.org/wiki/PPU_scrolling#Y_increment
        if v & 0x7000 != 0x7000 {
            v += 0x1000;
        } else {
            v &= 0x0fff;
            let mut y = (v & 0x03e0) >> 5;
            if y == 29 {
                y = 0;
                v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            v = (v & 0x7c1f) | (y << 5);
        }

        self.current_vram_address_register.write(v);
    }

    fn coarse_increment_horizontal_position(&mut self) {
        let mut v = self.current_vram_address_register.read();

        if v & 0x001f == 31 {
            v &= 0x7fe0;
            v ^= 0x0400;
        } else {
            v += 1;
        }

        self.current_vram_address_register.write(v);
    }

    fn fetch_tile_number(&mut self) {
        let v = self.current_vram_address_register.read();
        let address_high_byte = 0x20 | ((v >> 8) & 0x0f) as u8;
        let address_low_byte = v as u8; // v & 0xff 
        
        let tile_number = self.read(address_low_byte, address_high_byte);

        self.temporary_tile_data.tile_number = tile_number;
    }

    fn fetch_attribute_byte(&mut self) {
        let v = self.current_vram_address_register.read();
        let address_high_byte = 0x23 | ((v >> 8) & 0x0c) as u8;
        let address_low_byte = (((v >> 4) & 0x38) | ((v >> 2) & 0x07)) as u8;
        
        let attribute_table_byte = self.read(address_low_byte, address_high_byte);

        self.temporary_tile_data.attribute_table_byte = attribute_table_byte;
    }

    fn get_pattern_table_address(&mut self) -> (u8, u8) {
        let fine_y = (self.current_vram_address_register.read() >> 12) as u8;
        let pattern_table = self.ppu_control_register.read() & 0x10;
        let tile_number_low = self.temporary_tile_data.tile_number & 0xf;
        let tile_number_high = self.temporary_tile_data.tile_number >> 4;

        let address_high_byte = pattern_table | tile_number_high;
        let address_low_byte = (tile_number_low << 4) | fine_y;

        return (address_low_byte, address_high_byte);
    }

    fn fetch_pattern_table_low_byte(&mut self) {
        let (address_low_byte, address_high_byte) = self.get_pattern_table_address();

        let pattern_table_low_byte = self.read(address_low_byte, address_high_byte);

        self.temporary_tile_data.pattern_table_tile_low = pattern_table_low_byte;
    }

    fn fetch_pattern_table_high_byte(&mut self) {
        let (mut address_low_byte, address_high_byte) = self.get_pattern_table_address();
        address_low_byte |= 0x08;

        let pattern_table_high_byte = self.read(address_low_byte, address_high_byte);

        self.temporary_tile_data.pattern_table_tile_high = pattern_table_high_byte;
    }

    fn copy_horizontal_position_into_v(&mut self) {
        let t = self.temporary_vram_address_register.read();
        let mut v = self.current_vram_address_register.read();

        v = (v & 0x7be0) | (t & 0x041f);
        self.current_vram_address_register.write(v);
    }

    fn copy_vertical_position_into_v(&mut self) {
        let t = self.temporary_vram_address_register.read();
        let mut v = self.current_vram_address_register.read();

        v = (v & 0x041f) | (t & 0x7be0);
        self.current_vram_address_register.write(v);
    }

    fn shift_tile_data(&mut self) {
        self.current_tile_data = self.next_tile_data;
        self.next_tile_data = self.temporary_tile_data;
    }
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
            SpriteEvaluationState::OverflowCycle(cycle) => self.process_overflow_cycle(cycle),
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

            return SpriteEvaluationState::OverflowCycle(2);
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
            return SpriteEvaluationState::OverflowCycle(8);
        }
    }

    fn process_overflow_cycle(&mut self, cycle: u8) -> SpriteEvaluationState {
        if cycle % 2 == 1 {
            self.oam_address_register.increment();
        }

        if cycle < 8 {
            return SpriteEvaluationState::OverflowCycle(cycle + 1);
        } else {
            return SpriteEvaluationState::ReadOverflowCheckedValue;
        }
    }
}

// render stuff
impl Ppu {
    fn compute_background_pixel(&mut self) -> u8 {
        let low_plane = self.current_tile_data.pattern_table_tile_low;
        let high_plane = self.current_tile_data.pattern_table_tile_high;

        let fine_x = self.fine_x_scroll_register.read();
        let low_bit = (low_plane >> fine_x) & 1;
        let high_bit = (high_plane >> fine_x) & 1;

        let color_index = (high_bit << 1) | low_bit;

        let attribute_index = 
            if self.current_vram_address_register.read_bit(1) { 2 } else { 0 }
            + if self.current_vram_address_register.read_bit(6) { 4 } else { 0 };
        
        let palette_index = (self.current_tile_data.attribute_table_byte >> attribute_index) & 0x03;

        return (palette_index << 2) | color_index;
    }

    fn compute_sprite_pixel(&mut self) -> (u8, bool) {
        // returns (color, is high priority)

        let current_x = (self.cycle_count - 1) as u8; // we draw x = 0 at cycle 1?
        let current_y = self.scanline as u8;
        for sprite_index in 0..8 {
            let sprite_x = self.secondary_oam.read(sprite_index * 4 + 3);
            
            if !(sprite_x <= current_x && current_x <= sprite_x.saturating_add(7)) { continue; }
            let sprite_y = self.secondary_oam.read(sprite_index * 4);
            let sprite_tile = self.secondary_oam.read(sprite_index * 4 + 1);
            let sprite_attribute = self.secondary_oam.read(sprite_index * 4 + 1);

            let internal_x = 
                if sprite_attribute >> 5 & 1 == 0 { current_x - sprite_x }
                else {8 - (current_x - sprite_x)} as u8;
            
            let sprite_height = if self.ppu_control_register.read_bit(5) { 16 } else { 8 };

            let internal_x = 
                if sprite_attribute >> 5 & 1 == 0 { current_x - sprite_x }
                else {8 - (current_x - sprite_x)} as u8;
            
        }

        return (0, false);
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
        if self.initialising { return; }

        let mut t = self.temporary_vram_address_register.read();
        t = t & 0xf3ff | (value as u16 & 0x03) << 10;

        self.temporary_vram_address_register.write(t);
        
        let mut ppu_control = self.ppu_control_register.read();
        ppu_control = ppu_control & 0x03 | value & 0xfc;

        self.ppu_control_register.write(ppu_control);

    }

    fn write_ppu_mask(&mut self, value: u8) {
        if self.initialising { return; }

        self.ppu_mask_register.write(value);
    }

    fn write_ppu_scroll(&mut self, value: u8) {
        if self.initialising { return; }

        if !self.write_toggle_register.read_bit(0) {
            let mut t = self.temporary_vram_address_register.read();
            t = t & 0x7fe0 | (value as u16 >> 3);

            self.temporary_vram_address_register.write(t);

            let x = value & 0x07;
            self.fine_x_scroll_register.write(x);

            self.write_toggle_register.write(1);
        } else {
            let mut t = self.temporary_vram_address_register.read();
            t = t & 0x0c1f | (value as u16 & 0x03) << 12 | (value as u16 & 0xfc) << 2;

            self.temporary_vram_address_register.write(t);

            self.write_toggle_register.write(0);
        }
    }

    fn write_ppu_addr(&mut self, value: u8) {
        if self.initialising { return; }

        if !self.write_toggle_register.read_bit(0) {
            self.temporary_vram_address_register.write_high(value);
            self.temporary_vram_address_register.unset_bit(14);
            self.temporary_vram_address_register.unset_bit(15);
            self.write_toggle_register.write(1);
        } else {
            self.temporary_vram_address_register.write_low(value);
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
        let mut io_bus = self.io_bus.read();
        
        io_bus = (io_bus & 0x1f) | (status & 0xe0);
        self.io_bus.write(io_bus);
        self.ppu_status_register.unset_bit(7);
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
