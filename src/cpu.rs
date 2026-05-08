use crate::{ram::Ram, register::{Register8, Register16}, instructions::{Instruction, InstructionType, AddressMode}};

enum ReadWriteState {
    FetchOpcode,
    FetchImmediate,
    FetchZeroPageAddress,
    FetchAddressLow,
    FetchAddressHigh,
    ReadUseless,
    ReadZeroPage,
    ReadAbsoluteLow,
    ReadAbsoluteHigh,
    ReadRelative,
    ReadIndirectLow,
    ReadIndirectHigh,
    ReadZeroPageIndexedXAddress,
    ReadZeroPageIndexedXEffective,
    ReadZeroPageIndexedYAddress,
    ReadZeroPageIndexedYEffective,
    ReadAbsoluteIndexedXEffective,
    ReadAbsoluteIndexedXOops,
    ReadAbsoluteIndexedYEffective,
    ReadAbsoluteIndexedYOops,
    ReadIndexedIndirectAddress,
    ReadIndexedIndirectEffectiveLow,
    ReadIndexedIndirectEffectiveHigh,
    ReadIndexedIndirectEffective,
    ReadIndirectIndexedEffectiveLow,
    ReadIndirectIndexedEffectiveHigh,
    ReadIndirectIndexedEffective,
    ReadIndirectIndexedOops,
    WriteFakeEffective,
    WriteRealEffective,
}

enum AluState {
    Noop,
    AddIndexXToMarLow,
    AddIndexYToMarLow,
    IncrementMarHigh,
    ExecuteInstruction
}

struct Cpu {
    ram: Ram,
    accumulator: Register8,
    index_x: Register8,
    index_y: Register8,
    program_counter: Register16,
    stack_pointer: Register8,
    status_register: Register8,
    memory_data_register: Register8,
    memory_address_register: Register16,
    instruction_register: Register8,
    read_write_state: ReadWriteState,
    oopsed: bool,
    instruction: Instruction,
    address_mode: AddressMode,
}

impl Cpu {
    // pub fn new() -> Cpu {
    //     return Cpu { 
    //         ram: Ram::new(),
    //         accumulator: Register8::new(), 
    //         index_x: Register8::new(), 
    //         index_y: Register8::new(), 
    //         program_counter: Register8::new(), 
    //         stack_pointer: Register8::new(),
    //         status_register: Register8::new()
    //     }
    // }

    pub fn cycle(&mut self) {
        let mut alu_state = AluState::Noop;

        match self.read_write_state {
            ReadWriteState::FetchOpcode => {
                self.fetch_opcode();
                self.read_write_state = match self.address_mode {
                    AddressMode::Implicit | AddressMode::Accumulator => ReadWriteState::ReadUseless,
                    AddressMode::Immediate => ReadWriteState::FetchImmediate,
                    AddressMode::ZeroPage |
                    AddressMode::ZeroPageIndexedX | AddressMode::ZeroPageIndexedY | 
                    AddressMode::IndexedIndirectX | AddressMode::IndirectIndexedY => ReadWriteState::FetchZeroPageAddress,
                    AddressMode::Relative | AddressMode::Indirect | 
                    AddressMode::Absolute | AddressMode::AbsoluteIndexedX | AddressMode::AbsoluteIndexedY => ReadWriteState::FetchAddressLow
                };
                
                match self.address_mode {
                    AddressMode::Implicit | AddressMode::Accumulator => alu_state = AluState::ExecuteInstruction,
                    _ => ()
                }
            },

            // Things that happen on cycle #2
            ReadWriteState::FetchImmediate => {
                self.fetch_immediate();
                self.read_write_state = ReadWriteState::FetchOpcode;
                alu_state = AluState::ExecuteInstruction;
            },
            ReadWriteState::FetchZeroPageAddress => {
                self.fetch_zero_page();
                self.read_write_state = match self.instruction.get_type() {
                    InstructionType::Read | InstructionType::ReadModifyWrite => ReadWriteState::ReadZeroPage,
                    InstructionType::Write => ReadWriteState::WriteRealEffective,
                    _ => panic!("Zero page address should be either of type read/read-modify-write/write")
                };
            }
            ReadWriteState::FetchAddressLow => {
                self.fetch_address_low();
                self.read_write_state = ReadWriteState::FetchAddressHigh;
            }

            // Things that happen on cycle #3
            // IDFK
            _ => {}
        }

        self.oopsed = false;
        match alu_state {
            AluState::Noop => {},
            AluState::AddIndexXToMarLow => {
                let value = (self.memory_address_register.read_low() as u16) + (self.index_x.read() as u16);
                self.memory_address_register.write_low(value as u8);
                if value > 0x00ff { self.oopsed = true; }
            }, AluState::AddIndexYToMarLow => {
                let value = (self.memory_address_register.read_low() as u16) + (self.index_y.read() as u16);
                self.memory_address_register.write_low(value as u8);
                if value > 0x00ff { self.oopsed = true; }
            }, AluState::IncrementMarHigh => {
                self.memory_address_register.increment();
            }, AluState::ExecuteInstruction => {
                // TODO: IMPLEMENT!!!!!!
            }

        }
    }

    fn read(&mut self, low_byte: u8, high_byte: u8) -> u8 {
        // TODO: implement address mapping
        let value = self.ram.read(low_byte, high_byte);

        self.memory_data_register.write(value);
        return value;
    }

    fn read_from_program_counter(&mut self) -> u8 {
        return self.read(self.program_counter.read_low(), self.program_counter.read_high());
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        // TODO: implement address mapping
        self.ram.write(low_byte, high_byte, value);
    }

    fn write_to_effective_address(&mut self, value: u8) {
        self.write(self.memory_address_register.read_low(), self.memory_address_register.read_high(), value);
    }

    fn fetch_opcode(&mut self) {
        let opcode = self.read_from_program_counter();
        
        self.instruction_register.write(opcode);
        self.instruction = Instruction::from_opcode(opcode);
        self.address_mode = AddressMode::from_opcode(opcode);
    }

    fn fetch_immediate(&mut self) {
        self.read_from_program_counter();
    } 

    fn fetch_zero_page(&mut self) {
        let address = self.read_from_program_counter();

        self.memory_address_register.write_high(0);
        self.memory_address_register.write_low(address);
    }

    fn fetch_address_low(&mut self) {
        let address = self.read_from_program_counter();

        self.memory_address_register.write_low(address);
    }

    // instructions

    fn add_with_carry(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let carry = self.status_register.read_bit(0) as u8;
        
        let result_16 = (accumulator as u16) + (memory as u16) + (carry as u16);
        let result = accumulator.wrapping_add(memory).wrapping_add(carry);

        self.status_register.write_bit(0, result_16 > 0xff);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(6, ((result ^ accumulator) & (result ^ memory)) >> 7 == 1);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.accumulator.write(result);
    }

    fn subtract_with_carry(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let carry = self.status_register.read_bit(0) as u8;
        
        let result_16 = (accumulator as i16) - (memory as i16) - (!carry as i16);
        let result = accumulator.wrapping_add(!memory).wrapping_add(carry);

        self.status_register.write_bit(0, !(result_16 < 0x00));
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(6, ((result ^ accumulator) & (result ^ !memory)) >> 7 == 1);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.accumulator.write(result);
    }

    fn bitwise_and(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let result = accumulator & memory;

        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.accumulator.write(result);
    }

    fn bitwise_or(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let result = accumulator | memory;

        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.accumulator.write(result);
    }

    fn bitwise_exclusive_or(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let result = accumulator ^ memory;

        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.accumulator.write(result);
    }

    fn arithmetic_shift_left_accumulator(&mut self) {
        let accumulator = self.accumulator.read();
        let result = accumulator << 1;

        self.status_register.write_bit(0, accumulator >> 7 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.accumulator.write(result);
    }

    fn rotate_left_accumulator(&mut self) {
        let accumulator = self.accumulator.read();
        let carry = self.status_register.read_bit(0) as u8;
        let result = accumulator << 1 | carry;

        self.status_register.write_bit(0, accumulator >> 7 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.accumulator.write(result);
    }

    fn logical_shift_right_accumulator(&mut self) {
        let accumulator = self.accumulator.read();
        let result = accumulator >> 1;

        self.status_register.write_bit(0, accumulator & 1 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.accumulator.write(result);
    }

    fn rotate_right_accumulator(&mut self) {
        let accumulator = self.accumulator.read();
        let carry = self.status_register.read_bit(0) as u8;
        let result = accumulator >> 1 | carry << 7;

        self.status_register.write_bit(0, accumulator & 1 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.accumulator.write(result);
    }

    fn bit_test(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let result = accumulator & memory;

        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(6, (result >> 6) & 1 == 1);
        self.status_register.write_bit(7, result >> 7 == 1);
    }

    fn clear_carry(&mut self) {
        self.status_register.unset_bit(0);
    }

    fn clear_interrupt_disable(&mut self) {
        self.status_register.unset_bit(2);
    }

    fn clear_decimal(&mut self) {
        self.status_register.unset_bit(3);
    }

    fn clear_overflow(&mut self) {
        self.status_register.unset_bit(6);
    }

    fn set_carry(&mut self) {
        self.status_register.set_bit(0);
    }

    fn set_interrupt_disable(&mut self) {
        self.status_register.set_bit(2);
    }

    fn set_decimal(&mut self) {
        self.status_register.set_bit(3);
    }

    fn compare_accumulator(&mut self, memory: u8) {
        let accumulator = self.accumulator.read();
        let result = accumulator.wrapping_sub(memory);

        self.status_register.write_bit(0, accumulator >= memory);
        self.status_register.write_bit(1, accumulator == memory);
        self.status_register.write_bit(7, result >> 7 == 1);
    }

    fn compare_index_x(&mut self, memory: u8) {
        let index_x = self.index_x.read();
        let result = index_x.wrapping_sub(memory);

        self.status_register.write_bit(0, index_x >= memory);
        self.status_register.write_bit(1, index_x == memory);
        self.status_register.write_bit(7, result >> 7 == 1);
    }

    fn compare_index_y(&mut self, memory: u8) {
        let index_y = self.index_y.read();
        let result = index_y.wrapping_sub(memory);

        self.status_register.write_bit(0, index_y >= memory);
        self.status_register.write_bit(1, index_y == memory);
        self.status_register.write_bit(7, result >> 7 == 1);
    }

    fn decrement_memory(&mut self, memory: u8) {
        let result = memory.wrapping_sub(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.write_to_effective_address(result);
    }

    fn decrement_index_x(&mut self) {
        let result = self.index_x.read().wrapping_sub(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.index_x.write(result);
    }

    fn decrement_index_y(&mut self) {
        let result = self.index_y.read().wrapping_sub(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.index_y.write(result);
    }

    fn increment_memory(&mut self, memory: u8) {
        let result = memory.wrapping_add(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.write_to_effective_address(result);
    }

    fn increment_index_x(&mut self) {
        let result = self.index_x.read().wrapping_add(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.index_x.write(result);
    }

    fn increment_index_y(&mut self) {
        let result = self.index_y.read().wrapping_add(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.index_y.write(result);
    }

    fn load_accumulator(&mut self, memory: u8) {
        self.status_register.write_bit(1, memory == 0);
        self.status_register.write_bit(7, memory >> 7 == 1);

        self.accumulator.write(memory);
    }

    fn load_index_x(&mut self, memory: u8) {
        self.status_register.write_bit(1, memory == 0);
        self.status_register.write_bit(7, memory >> 7 == 1);

        self.index_x.write(memory);
    }

    fn load_index_y(&mut self, memory: u8) {
        self.status_register.write_bit(1, memory == 0);
        self.status_register.write_bit(7, memory >> 7 == 1);

        self.index_y.write(memory);
    }

    fn store_accumulator(&mut self) {
        self.write_to_effective_address(self.accumulator.read());
    }

    fn store_index_x(&mut self) {
        self.write_to_effective_address(self.index_x.read());
    }

    fn store_index_y(&mut self) {
        self.write_to_effective_address(self.index_y.read());
    }

    fn transfer_accumulator_to_index_x(&mut self) {
        let accumulator = self.accumulator.read();
        
        self.status_register.write_bit(1, accumulator == 0);
        self.status_register.write_bit(7, accumulator >> 7 == 1);
        
        self.index_x.write(accumulator);
    }

    fn transfer_accumulator_to_index_y(&mut self) {
        let accumulator = self.accumulator.read();
        
        self.status_register.write_bit(1, accumulator == 0);
        self.status_register.write_bit(7, accumulator >> 7 == 1);
        
        self.index_y.write(accumulator);
    }

    fn transfer_stack_pointer_to_index_x(&mut self) {
        let stack_pointer = self.stack_pointer.read();
        
        self.status_register.write_bit(1, stack_pointer == 0);
        self.status_register.write_bit(7, stack_pointer >> 7 == 1);
        
        self.index_x.write(stack_pointer);
    }

    fn trasnfer_index_x_to_accumulator(&mut self) {
        let index_x = self.index_x.read();

        self.status_register.write_bit(1, index_x == 0);
        self.status_register.write_bit(7, index_x >> 7 == 1);

        self.accumulator.write(index_x);
    }

    fn trasnfer_index_x_to_stack_pointer(&mut self) {
        let index_x = self.index_x.read();

        self.status_register.write_bit(1, index_x == 0);
        self.status_register.write_bit(7, index_x >> 7 == 1);

        self.stack_pointer.write(index_x);
    }

    fn trasnfer_index_y_to_accumulator(&mut self) {
        let index_y = self.index_y.read();

        self.status_register.write_bit(1, index_y == 0);
        self.status_register.write_bit(7, index_y >> 7 == 1);

        self.accumulator.write(index_y);
    }
}