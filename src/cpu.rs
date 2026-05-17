use crate::{instructions::Instruction, ram::Ram, register::{Register8, Register16}, transition::{State, transition}};

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
    alu_register: Register8,
    instruction_register: Register8,
    state: State,
    instruction: Instruction
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu { 
            ram: Ram::new(),
            accumulator: Register8::new(),
            index_x: Register8::new(),
            index_y: Register8::new(),
            program_counter: Register16::new(),
            stack_pointer: Register8::new(),
            status_register: Register8::new(),
            memory_data_register: Register8::new(),
            memory_address_register: Register16::new(),
            alu_register: Register8::new(),
            instruction_register: Register8::new(),
            state: State::ReadOpcode,
            instruction: Instruction::Nop
        }
    }

    pub fn cycle(&mut self) {
        let alt = match self.state {
            State::ReadOpcode => self.read_opcode(),
            State::Implied => self.implied(),
            State::Immediate => self.immediate(),
            State::OneOperandStart => self.one_operand(),
            // State::TwoOperandStart => self.two_operand(),
            State::ReadStart => self.start_read(),
            State::ReadWriteStart => self.start_read_write(),
            State::ReadWriteExecute => self.read_write_execute(),
            State::ReadWriteCommit => self.read_write_commit(),
            State::WriteStart => self.start_write(),
            State::ZeroPageIndexedX => self.zero_page_indexed_x(),
            State::ZeroPageIndexedY => self.zero_page_indexed_y(),
            // State::IndexedIndirectStart => self.indexed_indirect_pointer(),
            // State::IndexedIndirectLowByte => self.indexed_indirect_low_byte(),
            // State::IndexedIndirectHighByte => self.indexed_indirect_high_byte(),
            // State::IndirectIndexedStart => self.indirect_indexed_pointer(),
            // State::IndirectIndexedLowByte => self.indirect_indexed_low_byte(),
            // State::IndirectIndexedHighByte => self.indirect_indexed_high_byte(),
            // State::FixHighByte => self.fix_high_byte(),
            // State::JumpStart => self.jump_low_byte(),
            // State::JumpHighByte => self.jump_high_byte(),
            // State::AbslouteStart => self.absloute_address_low_byte(),
            // State::AbslouteAddressHighByte => self.absloute_address_high_byte(),
            // State::AbslouteIndexedXStart => self.absloute_indexed_x_low_byte(),
            // State::AbslouteIndexedXHighByte => self.absloute_indexed_x_high_byte(), 
            // State::AbslouteIndexedYStart => self.absloute_indexed_y_low_byte(),
            // State::AbslouteIndexedYHighByte => self.absloute_indexed_y_high_byte(),
            // State::AbslouteIndirectStart => self.absloute_indirect_pointer_low_byte(),
            // State::AbslouteIndirectHighByte => self.absloute_indirect_pointer_high_byte(),
            // State::AbslouteIndirectLowByteActual => self.absloute_indirect_low_byte(),
            // State::AbslouteIndirectHighByteActual => self.absloute_indirect_high_byte(),
            // State::Relative => self.relative_pointer(),
            // State::RelativeLowByte => self.relative_low_byte(),
            // State::RelativeHighByte => self.relative_high_byte()
            _ => panic!("State not implented!")
        };

        self.state = match transition(&self.state, self.instruction_register.read(), alt) {
            Some(state) => state,
            None => panic!("Invalid state {:?} {} {}", self.state, self.instruction_register.read(), alt)
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

    fn read_from_effective_address(&mut self) -> u8 {
        return self.read(self.memory_address_register.read_low(), self.memory_address_register.read_high());
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        // TODO: implement address mapping
        self.ram.write(low_byte, high_byte, value);
    }

    fn write_to_effective_address(&mut self, value: u8) {
        self.write(self.memory_address_register.read_low(), self.memory_address_register.read_high(), value);
    }

    // do states

    fn read_opcode(&mut self) -> bool {
        let opcode = self.read_from_program_counter();
        self.instruction = Instruction::from_opcode(opcode);

        self.instruction_register.write(opcode);
        self.program_counter.increment();
        return false;
    }

    fn implied(&mut self) -> bool  {
        match self.instruction {
            Instruction::Inx => self.increment_index_x(),
            Instruction::Iny => self.increment_index_y(),
            Instruction::Dex => self.decrement_index_x(),
            Instruction::Dey => self.decrement_index_y(),
            Instruction::Asl => self.arithmetic_shift_left_accumulator(),
            Instruction::Rol => self.rotate_left_accumulator(),
            Instruction::Lsr => self.logical_shift_right_accumulator(),
            Instruction::Ror => self.rotate_right_accumulator(),
            Instruction::Txa => self.trasnfer_index_x_to_accumulator(),
            Instruction::Tya => self.trasnfer_index_y_to_accumulator(),
            Instruction::Tax => self.transfer_accumulator_to_index_x(),
            Instruction::Tay => self.transfer_accumulator_to_index_y(),
            Instruction::Tsx => self.transfer_stack_pointer_to_index_x(),
            Instruction::Txs => self.trasnfer_index_x_to_stack_pointer(),
            Instruction::Clc => self.clear_carry(),
            Instruction::Clv => self.clear_overflow(),
            Instruction::Cli => self.clear_interrupt_disable(),
            Instruction::Cld => self.clear_decimal(),
            Instruction::Sec => self.set_carry(),
            Instruction::Sei => self.set_interrupt_disable(),
            Instruction::Sed => self.set_decimal(),
            Instruction::Nop => (),
            _ => panic!("Instruction {:?} doesn't have implied addressing mode", self.instruction)
        }
        return false;
    }

    fn immediate(&mut self) -> bool {
        let value = self.read_from_program_counter();

        match self.instruction {
            Instruction::Nop => (), // note: unofficial
            Instruction::Lda => self.load_accumulator(value),
            Instruction::Ldx => self.load_index_x(value),
            Instruction::Ldy => self.load_index_y(value),
            Instruction::Cmp => self.compare_accumulator(value),
            Instruction::Cpx => self.compare_index_x(value),
            Instruction::Cpy => self.compare_index_y(value),
            Instruction::And => self.bitwise_and(value),
            Instruction::Ora => self.bitwise_or(value),
            Instruction::Eor => self.bitwise_exclusive_or(value),
            Instruction::Adc => self.add_with_carry(value),
            Instruction::Sbc => self.subtract_with_carry(value),
            _ => panic!("Instruction {:?} doesn't have immediate addressing mode", self.instruction)
        }

        self.program_counter.increment();
        return false;
    }

    fn one_operand(&mut self) -> bool {
        let value = self.read_from_program_counter();
        self.memory_address_register.write_low(value);
        self.memory_address_register.write_high(0);
        self.program_counter.increment();
        return false;
    }

    fn start_read(&mut self) -> bool {
        let value = self.read_from_effective_address();
        match self.instruction {
            Instruction::Nop => (), // note: unofficial
            Instruction::Lda => self.load_accumulator(value),
            Instruction::Ldx => self.load_index_x(value),
            Instruction::Ldy => self.load_index_y(value),
            Instruction::Cmp => self.compare_accumulator(value),
            Instruction::Cpx => self.compare_index_x(value),
            Instruction::Cpy => self.compare_index_y(value),
            Instruction::And => self.bitwise_and(value),
            Instruction::Ora => self.bitwise_or(value),
            Instruction::Eor => self.bitwise_exclusive_or(value),
            Instruction::Adc => self.add_with_carry(value),
            Instruction::Sbc => self.subtract_with_carry(value),
            Instruction::Bit => self.bit_test(value),
            _ => panic!("Instruction {:?} is not a read instrcution", self.instruction)
        }

        return false;
    }

    fn start_read_write(&mut self) -> bool {
        self.read_from_effective_address();
        return false;
    }

    fn read_write_execute(&mut self) -> bool {
        let value = self.read_from_effective_address();
        self.write_to_effective_address(value); // fake write

        match self.instruction {
            Instruction::Asl => self.arithmetic_shift_left(value),
            Instruction::Lsr => self.logical_shift_right(value),
            Instruction::Rol => self.rotate_left(value),
            Instruction::Ror => self.rotate_right(value),
            Instruction::Inc => self.increment(value),
            Instruction::Dec => self.decrement(value),
            _ => panic!("Instruction {:?} is not a read-modify-write instruction", self.instruction)
        }

        return false;
    }

    fn read_write_commit(&mut self) -> bool {
        let value = self.alu_register.read();
        self.write_to_effective_address(value);

        return false;
    }
    
    fn start_write(&mut self) -> bool {
        let value = match self.instruction {
            Instruction::Sta => self.accumulator.read(),
            Instruction::Stx => self.index_x.read(),
            Instruction::Sty => self.index_y.read(),
            _ => panic!("Instruction {:?} is not a write instruction", self.instruction)
        };

        self.write_to_effective_address(value);

        return false;
    }

    fn zero_page_indexed_x(&mut self) -> bool {
        let mut effective_address = self.memory_address_register.read_low();
        let index_x = self.index_x.read();
        effective_address = effective_address.wrapping_add(index_x);

        self.memory_address_register.write_low(effective_address);
        return false;
    }

    fn zero_page_indexed_y(&mut self) -> bool {
        let mut effective_address = self.memory_address_register.read_low();
        let index_y = self.index_y.read();
        effective_address = effective_address.wrapping_add(index_y);

        self.memory_address_register.write_low(effective_address);
        return false;
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

    fn arithmetic_shift_left(&mut self, memory: u8) {
        let result = memory << 1;

        self.status_register.write_bit(0, memory >> 7 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.alu_register.write(result);
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

    fn rotate_left(&mut self, memory: u8) {
        let carry = self.status_register.read_bit(0) as u8;
        let result = memory << 1 | carry;

        self.status_register.write_bit(0, memory >> 7 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.alu_register.write(result);
    }

    fn logical_shift_right_accumulator(&mut self) {
        let accumulator = self.accumulator.read();
        let result = accumulator >> 1;

        self.status_register.write_bit(0, accumulator & 1 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.accumulator.write(result);
    }

    fn logical_shift_right(&mut self, memory: u8) {
        let result = memory >> 1;

        self.status_register.write_bit(0, memory & 1 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.alu_register.write(result);
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

    fn rotate_right(&mut self, memory: u8) {
        let carry = self.status_register.read_bit(0) as u8;
        let result = memory >> 1 | carry << 7;

        self.status_register.write_bit(0, memory & 1 == 1);
        self.status_register.write_bit(1, result == 0x00);
        self.status_register.write_bit(7, result >> 7 == 1);

    
        self.alu_register.write(result);
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

    fn decrement(&mut self, memory: u8) {
        let result = memory.wrapping_sub(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.alu_register.write(result);
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

    fn increment(&mut self, memory: u8) {
        let result = memory.wrapping_add(1);

        self.status_register.write_bit(1, result == 0);
        self.status_register.write_bit(7, result >> 7 == 1);

        self.alu_register.write(result);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();

        // write LDA #$F6 to ram
        cpu.ram.write(0x00, 0x00, 0xa9);
        cpu.ram.write(0x01, 0x00, 0xf6);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::Immediate);
        assert_eq!(cpu.program_counter.read(), 0x0001);
        assert_eq!(cpu.accumulator.read(), 0x00);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.accumulator.read(), 0xf6);
        assert!(!cpu.status_register.read_bit(1));
        assert!(cpu.status_register.read_bit(7));
    }

    #[test]
    fn test_ldx_zero_page() {
        let mut cpu = Cpu::new();

        // write LDX $12 to ram, and write #$66 to $12
        cpu.ram.write(0x00, 0x00, 0xa6);
        cpu.ram.write(0x01, 0x00, 0x12);
        cpu.ram.write(0x12, 0x00, 0x66);

        
        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);
        assert_eq!(cpu.accumulator.read(), 0x00);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0012);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.index_x.read(), 0x66);
    }

    #[test]
    fn test_adc_zero_page() {
        let mut cpu = Cpu::new();

        // write ADC $34 to ram, and write #$66 to $34
        cpu.ram.write(0x00, 0x00, 0x65);
        cpu.ram.write(0x01, 0x00, 0x34);
        cpu.ram.write(0x34, 0x00, 0x34);
        cpu.accumulator.write(0x55);
        cpu.status_register.set_bit(0);

        // this should set accumulator to 0x34 + 0x55 + 0x01 = 0x8a
        // so the overflow, negative flags should be set

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0034);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.accumulator.read(), 0x8a);
        assert_eq!(cpu.status_register.read(), 0b11000000)
    }

    #[test]
    fn test_sty_zero_page() {
        let mut cpu = Cpu::new();

        // write STY $77 to ram
        cpu.ram.write(0x00, 0x00, 0x84);
        cpu.ram.write(0x01, 0x00, 0x77);
        cpu.index_y.write(0x4f);

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::WriteStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0077);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.ram.read(0x77, 0x00), 0x4f);
    }

    #[test]
    fn test_eor_zero_page_indexed_x() {
        let mut cpu = Cpu::new();

        // write EOR $9f,x to ram
        cpu.ram.write(0x00, 0x00, 0x55);
        cpu.ram.write(0x01, 0x00, 0x9f);
        cpu.ram.write(0x1b, 0x00, 0xe5);
        cpu.accumulator.write(0x43);
        cpu.index_x.write(0x7c);

        // effective address = 0x9f + 0x7c = 0x1b (note that this wraps around)
        // new value = 0xe5 ^ 0x43 = 0xa6 

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::ZeroPageIndexedX);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x009f);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x001b);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.accumulator.read(), 0xa6);
        assert!(cpu.status_register.read_bit(7));
    }

    #[test]
    fn test_inc_zero_page_indexed_x() {
        let mut cpu = Cpu::new();

        // write INC $37,x to ram
        cpu.ram.write(0x00, 0x00, 0xf6);
        cpu.ram.write(0x01, 0x00, 0x37);
        cpu.ram.write(0x79, 0x00, 0xff);
        cpu.index_x.write(0x42);

        // effective address = 0x37 + 0x42 = 0x79
        // new value = 0xff + 1 = 0x00

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::ZeroPageIndexedX);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0037);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0079);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteExecute);
        assert_eq!(cpu.program_counter.read(), 0x0002);

        // note: i can't test for the extra write
        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteCommit);
        assert_eq!(cpu.status_register.read(), 0b00000010);
        assert_eq!(cpu.ram.read(0x79, 0x00), 0xff); // write not commited yet

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.ram.read(0x79, 0x00), 0x00);
    }
}
