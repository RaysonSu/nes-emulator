use std::io::Read;

use crate::{ram::Ram, register::Register8};

struct Cpu {
    ram: Ram,
    accumulator: Register8,
    index_x: Register8,
    index_y: Register8,
    program_counter: Register8,
    stack_pointer: Register8,
    status_register: Register8,
    state: CpuState,
    opcode: u8,
    operands: Vec<u8>
}

enum CpuState {
    Fetch,
    Decode,
    Execute
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu { 
            ram: Ram::new(),
            accumulator: Register8::new(), 
            index_x: Register8::new(), 
            index_y: Register8::new(), 
            program_counter: Register8::new(), 
            stack_pointer: Register8::new(),
            status_register: Register8::new()
        }
    }

    pub fn cycle(&mut self) {
        match self.state {
            CpuState::Fetch => self.fetch(),
            CpuState::Decode => (),
            CpuState::Execute => ()
        }
    }

    fn fetch(&mut self) {

    }
}