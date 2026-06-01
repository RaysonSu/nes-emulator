use crate::{instructions::Instruction, ram::Ram, register::{Register8, Register16}, transition::{State, transition}, ppu::Ppu};

pub struct Cpu {
    ram: Ram,
    ppu: Option<Box<Ppu>>,

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
    instruction: Instruction,
    oops_amount: i8
}
// core methods
impl Cpu {
    pub fn new() -> Cpu {
        let mut res = Cpu { 
            ram: Ram::new(),
            ppu: None,

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
            instruction: Instruction::Nop,
            oops_amount: 0
        };

        res.status_register.set_bit(5);
        return res;
    }

    pub fn connect_ppu(&mut self, ppu: Box<Ppu>) {
        self.ppu = Some(ppu);
    }

    pub fn cycle(&mut self) {
        let alt = match self.state {
            State::ReadOpcode => self.read_opcode(),
            State::Implied => self.implied(),
            State::Immediate => self.immediate(),
            State::OneOperandStart => self.one_operand(),
            State::TwoOperandStart => self.two_operand(),
            State::ReadStart => self.start_read(),
            State::ReadWriteStart => self.start_read_write(),
            State::ReadWriteExecute => self.read_write_execute(),
            State::ReadWriteCommit => self.read_write_commit(),
            State::WriteStart => self.start_write(),
            State::ZeroPageIndexedX => self.zero_page_indexed_x(),
            State::ZeroPageIndexedY => self.zero_page_indexed_y(),
            State::IndexedIndirectStart => self.indexed_indirect_pointer(),
            State::IndexedIndirectLowByte => self.indexed_indirect_low_byte(),
            State::IndexedIndirectHighByte => self.indexed_indirect_high_byte(),
            State::IndirectIndexedLowByte => self.indirect_indexed_low_byte(),
            State::IndirectIndexedHighByte => self.indirect_indexed_high_byte(),
            State::FixHighByte => self.fix_high_byte(),
            State::JumpHighByte => self.jump_high_byte(),
            State::AbsoluteAddressHighByte => self.absolute_address_high_byte(),
            State::AbsoluteIndexedXHighByte => self.absolute_indexed_x_high_byte(), 
            State::AbsoluteIndexedYHighByte => self.absolute_indexed_y_high_byte(),
            State::AbsoluteIndirectHighByte => self.absolute_indirect_pointer_high_byte(),
            State::AbsoluteIndirectLowByteActual => self.absolute_indirect_low_byte(),
            State::AbsoluteIndirectHighByteActual => self.absolute_indirect_high_byte(),
            State::Relative => self.relative_pointer(),
            State::RelativeLowByte => self.relative_low_byte(),
            State::RelativeHighByte => self.relative_high_byte(),
            State::DummyRead => self.dummy_read(),
            State::DummyReadAndIncrementPC => self.dummy_read_and_increment_pc(),
            State::PushPCHighByte => self.push_pch(),
            State::PushPCLowByte => self.push_pcl(),
            State::PushStatusRegisteWithBFlag => self.push_sr_with_b(),
            State::FetchPCLowByte => self.fetch_pcl(),
            State::FetchPCHighByte => self.fetch_pch(),
            State::IncrementSP => self.increment_sp(),
            State::PullStatusRegisterAndIncrementSP => self.pull_sr_and_increment_sp(),
            State::PullPCLowByte => self.pull_pcl(),
            State::PullPCHighByte => self.pull_pch(),
            State::IncrementPC => self.increment_pc(),
            State::PushAccumulator => self.push_accumulator(),
            State::PushStatusRegister => self.push_sr(),
            State::PullAccumulator => self.pull_accumulator(),
            State::PullStatusRegister => self.pull_sr(),
            State::FetchSubroutineLowByte => self.fetch_subroutine_low_byte(),
            State::JSRMystery => self.do_nothing(),
            State::FetchSubroutineHighByte => self.fetch_subroutine_high_byte(),
            // _ => panic!("State not implented!")
        };

        self.state = match transition(&self.state, self.instruction_register.read(), alt) {
            Some(state) => state,
            None => panic!("Invalid state {:?} {} {}", self.state, self.instruction_register.read(), alt)
        }

    }

    fn read(&mut self, low_byte: u8, high_byte: u8) -> u8 {
        // TODO: implement address mapping
        let mut value = self.memory_data_register.read();
        
        if high_byte < 0x20 {
            value = self.ram.read(low_byte, high_byte);
        } else if high_byte < 0x40 {
            // do shit.
        }

        self.memory_data_register.write(value);
        return value;
    }

    fn read_from_program_counter(&mut self) -> u8 {
        return self.read(self.program_counter.read_low(), self.program_counter.read_high());
    }

    fn read_from_memory_address_register(&mut self) -> u8 {
        return self.read(self.memory_address_register.read_low(), self.memory_address_register.read_high());
    }
    
    fn read_from_stack_pointer(&mut self) -> u8 {
        let stack_pointer = self.stack_pointer.read();
        return self.ram.read(stack_pointer, 0x01);
    }

    fn write(&mut self, low_byte: u8, high_byte: u8, value: u8) {
        // TODO: implement address mapping
        self.ram.write(low_byte, high_byte, value);
    }

    fn write_to_memory_address_register(&mut self, value: u8) {
        self.write(self.memory_address_register.read_low(), self.memory_address_register.read_high(), value);
    }

    fn write_to_stack_pointer(&mut self, value: u8) {
        let stack_pointer = self.stack_pointer.read();
        self.ram.write(stack_pointer, 0x01, value);
    }
}

// do states
impl Cpu {
    fn read_opcode(&mut self) -> bool {
        let opcode = self.read_from_program_counter();
        self.instruction = Instruction::from_opcode(opcode);

        self.instruction_register.write(opcode);
        self.program_counter.increment();

        self.oops_amount = 0; // TODO: fix this shitty bodge
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
            Instruction::Tya => self.transfer_index_y_to_accumulator(),
            Instruction::Tax => self.transfer_accumulator_to_index_x(),
            Instruction::Tay => self.transfer_accumulator_to_index_y(),
            Instruction::Tsx => self.transfer_stack_pointer_to_index_x(),
            Instruction::Txs => self.transfer_index_x_to_stack_pointer(),
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

    fn two_operand(&mut self) -> bool {
        let value = self.read_from_program_counter();
        self.memory_address_register.write_low(value);
        self.program_counter.increment();

        return false;
    }

    fn start_read(&mut self) -> bool {
        let value = self.read_from_memory_address_register();
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
        self.read_from_memory_address_register();
        return false;
    }

    fn read_write_execute(&mut self) -> bool {
        let value = self.read_from_memory_address_register();
        self.write_to_memory_address_register(value); // fake write

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
        self.write_to_memory_address_register(value);

        return false;
    }
    
    fn start_write(&mut self) -> bool {
        let value = match self.instruction {
            Instruction::Sta => self.accumulator.read(),
            Instruction::Stx => self.index_x.read(),
            Instruction::Sty => self.index_y.read(),
            _ => panic!("Instruction {:?} is not a write instruction", self.instruction)
        };

        self.write_to_memory_address_register(value);

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

    fn indexed_indirect_pointer(&mut self) -> bool {
        let mut effective_address = self.memory_address_register.read_low();
        let index_x = self.index_x.read();
        effective_address = effective_address.wrapping_add(index_x);

        self.memory_address_register.write_low(effective_address);

        return false;
    }

    fn indexed_indirect_low_byte(&mut self) -> bool {
        self.read_from_memory_address_register();
        let ptr = self.memory_address_register.read_low();
        self.memory_address_register.write_low(ptr.wrapping_add(1));

        return false;
    }

    fn indexed_indirect_high_byte(&mut self) -> bool {
        let low_byte = self.memory_data_register.read();
        let high_byte = self.read_from_memory_address_register();

        self.memory_address_register.write_low(low_byte);
        self.memory_address_register.write_high(high_byte);

        return false;
    }

    fn indirect_indexed_low_byte(&mut self) -> bool {
        self.read_from_memory_address_register();
        let ptr = self.memory_address_register.read_low();
        self.memory_address_register.write_low(ptr.wrapping_add(1));

        return false;
    }

    fn indirect_indexed_high_byte(&mut self) -> bool {
        let low_byte = self.memory_data_register.read();
        let high_byte = self.read_from_memory_address_register();

        let oopsed = (low_byte as u16) + (self.index_y.read() as u16) >= 0x100;
        let new_low_byte = low_byte.wrapping_add(self.index_y.read());

        self.memory_address_register.write_low(new_low_byte);
        self.memory_address_register.write_high(high_byte);

        if oopsed { self.oops_amount = 1 }

        return oopsed;
    }

    fn fix_high_byte(&mut self) -> bool {
        let high_byte = self.memory_address_register.read_high();
        self.memory_address_register.write_high(high_byte.wrapping_add_signed(self.oops_amount));

        return false;
    }

    fn jump_high_byte(&mut self) -> bool {
        let low_byte = self.memory_address_register.read_low();
        let high_byte = self.read_from_program_counter();

        self.program_counter.write_low(low_byte);
        self.program_counter.write_high(high_byte);

        return false;
    }

    fn absolute_address_high_byte(&mut self) -> bool {
        let high_byte = self.read_from_program_counter();
        self.memory_address_register.write_high(high_byte);
        self.program_counter.increment();

        return false;
    }

    fn absolute_indexed_x_high_byte(&mut self) -> bool {
        let low_byte = self.memory_data_register.read();
        let high_byte = self.read_from_program_counter();
        let index_x = self.index_x.read();

        let oopsed = (low_byte as u16) + (index_x as u16) >= 0x100;
        let new_low_byte = low_byte.wrapping_add(index_x);

        self.memory_address_register.write_low(new_low_byte);
        self.memory_address_register.write_high(high_byte);

        if oopsed { self.oops_amount = 1 };

        self.program_counter.increment();
        return oopsed;
    }

    fn absolute_indexed_y_high_byte(&mut self) -> bool {
        let low_byte = self.memory_data_register.read();
        let high_byte = self.read_from_program_counter();
        let index_y = self.index_y.read();

        let oopsed = (low_byte as u16) + (index_y as u16) >= 0x100;
        let new_low_byte = low_byte.wrapping_add(index_y);

        self.memory_address_register.write_low(new_low_byte);
        self.memory_address_register.write_high(high_byte);

        if oopsed { self.oops_amount = 1 };

        self.program_counter.increment();
        return oopsed;
    }
    
    fn absolute_indirect_pointer_high_byte(&mut self) -> bool {
        let high_byte = self.read_from_program_counter();
        self.memory_address_register.write_high(high_byte);
        self.program_counter.increment();

        return false;
    }

    fn absolute_indirect_low_byte(&mut self) -> bool {
        self.read_from_memory_address_register();
        
        let low_byte = self.memory_address_register.read_low();
        self.memory_address_register.write_low(low_byte.wrapping_add(1)); // high byte is not handled - this is a bug in the original cpu!

        return false;
    }

    fn absolute_indirect_high_byte(&mut self) -> bool {
        let low_byte = self.memory_data_register.read();
        let high_byte = self.read_from_memory_address_register();

        self.program_counter.write_low(low_byte);
        self.program_counter.write_high(high_byte);

        return false;
    }  

    fn relative_pointer(&mut self) -> bool {
        self.read_from_program_counter();
        
        let branch = match self.instruction {
            Instruction::Bcs => self.status_register.read_bit(0),
            Instruction::Bcc => !self.status_register.read_bit(0),
            Instruction::Beq => self.status_register.read_bit(1),
            Instruction::Bne => !self.status_register.read_bit(1),
            Instruction::Bvs => self.status_register.read_bit(6),
            Instruction::Bvc => !self.status_register.read_bit(6),
            Instruction::Bmi => self.status_register.read_bit(7),
            Instruction::Bpl => !self.status_register.read_bit(7),
            _ => panic!("Instruction {:?} is not a branch instrcution", self.instruction)
        };

        self.program_counter.increment();

        return branch
    }

    fn relative_low_byte(&mut self) -> bool {
        let branch_amount = self.memory_data_register.read().cast_signed();
        let program_counter_low_byte = self.program_counter.read_low();
        
        let new_low_byte = (program_counter_low_byte as i16) + (branch_amount as i16);
        let mut alt = false;
        
        if new_low_byte < 0x00 {
            self.oops_amount = -1;
            alt = true;
        } else if new_low_byte >= 0x100 {
            self.oops_amount = 1;
            alt = true;
        }

        self.program_counter.write_low(program_counter_low_byte.wrapping_add_signed(branch_amount));

        return alt;
    }

    fn relative_high_byte(&mut self) -> bool {
        let high_byte = self.program_counter.read_high();
        self.program_counter.write_high(high_byte.wrapping_add_signed(self.oops_amount));

        return false;
    }

    fn dummy_read(&mut self) -> bool {
        self.read_from_program_counter();
    
        return false;
    }

    fn dummy_read_and_increment_pc(&mut self) -> bool {
        self.read_from_program_counter();
        self.program_counter.increment();

        return false;
    }

    fn push_pch(&mut self) -> bool {
        let program_counter_high_byte = self.program_counter.read_high();
        self.write_to_stack_pointer(program_counter_high_byte);
        self.stack_pointer.decrement();

        return false;
    }

    fn push_pcl(&mut self) -> bool {
        let program_counter_low_byte = self.program_counter.read_low();
        self.write_to_stack_pointer(program_counter_low_byte);
        self.stack_pointer.decrement();

        return false;
    }

    fn push_sr_with_b(&mut self) -> bool {
        let status_register = self.status_register.read() | 0x30;
        
        self.write_to_stack_pointer(status_register);
        self.stack_pointer.decrement();

        return false;
    }

    fn fetch_pcl(&mut self) -> bool {
        let program_counter_low_byte = self.read(0xfe, 0xff);
        self.program_counter.write_low(program_counter_low_byte);
        
        return false;
    }

    fn fetch_pch(&mut self) -> bool {
        let program_counter_high_byte = self.read(0xff, 0xff);
        self.program_counter.write_high(program_counter_high_byte);

        return false;
    }

    fn increment_sp(&mut self) -> bool {
        self.stack_pointer.increment();

        return false;
    }

    fn pull_sr_and_increment_sp(&mut self) -> bool {
        let status_register = self.read_from_stack_pointer() & 0xcf;
        
        self.status_register.write(status_register);
        self.stack_pointer.increment();

        return false;
    }

    fn pull_pcl(&mut self) -> bool {
        let program_counter_high_byte = self.read_from_stack_pointer();
        self.program_counter.write_high(program_counter_high_byte);
        self.stack_pointer.increment();

        return false;
    }

    fn pull_pch(&mut self) -> bool {
        let program_counter_low_byte = self.read_from_stack_pointer();
        self.program_counter.write_low(program_counter_low_byte);
        self.stack_pointer.increment();

        return false;
    }

    fn increment_pc(&mut self) -> bool {
        self.program_counter.increment();

        return false;
    }

    fn push_accumulator(&mut self) -> bool {
        let accumulator = self.accumulator.read();
        self.write_to_stack_pointer(accumulator);
        self.stack_pointer.decrement();

        return false;
    }

    fn push_sr(&mut self) -> bool {
        let status_register = self.status_register.read() | 0x30;
        self.write_to_stack_pointer(status_register);
        self.stack_pointer.decrement();

        return false;
    }

    fn pull_accumulator(&mut self) -> bool {
        let accumulator  = self.read_from_stack_pointer();
        self.accumulator.write(accumulator);
        self.stack_pointer.increment();

        return false;
    }

    fn pull_sr(&mut self) -> bool {
        let status_register = self.read_from_stack_pointer() & 0xcf;
        self.status_register.write(status_register);

        return false;
    }

    fn fetch_subroutine_low_byte(&mut self) -> bool {
        let low_byte = self.read_from_program_counter();
        self.memory_address_register.write_low(low_byte);
        self.program_counter.increment();

        return false;
    }

    fn fetch_subroutine_high_byte(&mut self) -> bool {
        let high_byte = self.read_from_program_counter();
        let low_byte = self.memory_address_register.read_low();

        self.program_counter.write_low(low_byte);
        self.program_counter.write_high(high_byte);

        return false;
    }

    fn do_nothing(&mut self) -> bool {
        return false;
    }
}

// instructions
impl Cpu {
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

    fn transfer_index_x_to_stack_pointer(&mut self) {
        let index_x = self.index_x.read();

        self.status_register.write_bit(1, index_x == 0);
        self.status_register.write_bit(7, index_x >> 7 == 1);

        self.stack_pointer.write(index_x);
    }

    fn transfer_index_y_to_accumulator(&mut self) {
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
        assert_eq!(cpu.status_register.read(), 0b11100000)
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
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.status_register.read(), 0b00100010);
        assert_eq!(cpu.ram.read(0x79, 0x00), 0xff); // write not commited yet

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.ram.read(0x79, 0x00), 0x00);
    }

    #[test]
    fn test_sta_indexed_indirect() {
        let mut cpu = Cpu::new();

        // write STA ($23,x) to ram
        cpu.ram.write(0x00, 0x00, 0x81);
        cpu.ram.write(0x01, 0x00, 0x79);
        cpu.ram.write(0x35, 0x00, 0x12);
        cpu.ram.write(0x36, 0x00, 0x06);
        cpu.index_x.write(0xbc);
        cpu.accumulator.write(0xa3);

        // pointer = 0x79 + 0xbc = 0x35
        // effective address = 0x0612 ($36 $35)

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::IndexedIndirectStart);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read_low(), 0x79);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndexedIndirectLowByte);
        assert_eq!(cpu.memory_address_register.read(), 0x0035);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndexedIndirectHighByte);
        assert_eq!(cpu.memory_data_register.read(), 0x12);

        cpu.cycle();
        assert_eq!(cpu.state, State::WriteStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0612);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.ram.read(0x12, 0x06), 0xa3)
    }

    #[test]
    fn test_jmp_absolute_and_indirect() {
        let mut cpu = Cpu::new();

        // write JMP $0623 to ram
        cpu.ram.write(0x00, 0x00, 0x4c);
        cpu.ram.write(0x01, 0x00, 0x23);
        cpu.ram.write(0x02, 0x00, 0x06);
        
        // write a second JMP ($04ff) that crosses page boundaries
        // this should jump to $0288
        cpu.ram.write(0x23, 0x06, 0x6c);
        cpu.ram.write(0x24, 0x06, 0xff);
        cpu.ram.write(0x25, 0x06, 0x04);
        cpu.ram.write(0xff, 0x04, 0x88);
        cpu.ram.write(0x00, 0x04, 0x02);

        // first instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::JumpHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read_low(), 0x23);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0623);

        // second instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0624);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndirectHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0625);
        assert_eq!(cpu.memory_address_register.read_low(), 0xff);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndirectLowByteActual);
        assert_eq!(cpu.memory_address_register.read(), 0x04ff);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndirectHighByteActual);
        assert_eq!(cpu.memory_data_register.read(), 0x88);
        assert_eq!(cpu.memory_address_register.read(), 0x0400);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0288);
    }

    #[test]
    fn test_bvs() {
        let mut cpu = Cpu::new();

        // write BVS $10 (that will not be taken)
        cpu.ram.write(0x00, 0x00, 0x70);
        cpu.ram.write(0x01, 0x00, 0x10);
        
        // write BVS $7f (that will be taken)
        cpu.ram.write(0x02, 0x00, 0x70);
        cpu.ram.write(0x03, 0x00, 0x7f);

        // write BVS $7f (that crosses page boundary)
        cpu.ram.write(0x83, 0x00, 0x70);
        cpu.ram.write(0x84, 0x00, 0x7f);

        // write BVS $80 (that crosses page boundary backwards)
        cpu.ram.write(0x04, 0x01, 0x70);
        cpu.ram.write(0x05, 0x01, 0x80);

        // so pc should be $0086 after this
        
        // the BVS $10 instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::Relative);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        // branch not taken, so continue to next instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);

        // fake the overflow flag being set
        cpu.status_register.set_bit(6);
        
        // the (first) BVS $7f instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::Relative);
        assert_eq!(cpu.program_counter.read(), 0x0003);

        cpu.cycle();
        assert_eq!(cpu.state, State::RelativeLowByte);
        assert_eq!(cpu.program_counter.read(), 0x0004);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0083);

        // the (second) BVS $7f instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::Relative);
        assert_eq!(cpu.program_counter.read(), 0x0084);

        cpu.cycle();
        assert_eq!(cpu.state, State::RelativeLowByte);
        assert_eq!(cpu.program_counter.read(), 0x0085);

        // cpu fucked up
        cpu.cycle();
        assert_eq!(cpu.state, State::RelativeHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0004);
        
        // fix it
        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0104);

        // the BVS $80 instruction
        cpu.cycle();
        assert_eq!(cpu.state, State::Relative);
        assert_eq!(cpu.program_counter.read(), 0x0105);

        cpu.cycle();
        assert_eq!(cpu.state, State::RelativeLowByte);
        assert_eq!(cpu.program_counter.read(), 0x0106);

        cpu.cycle();
        assert_eq!(cpu.state, State::RelativeHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0186);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0086);
    }

    #[test]
    fn test_flag_setting() {
        let mut cpu = Cpu::new();
        
        cpu.ram.write(0x00, 0x00, 0x38); // SEC
        cpu.ram.write(0x01, 0x00, 0xf8); // SED
        cpu.ram.write(0x02, 0x00, 0x78); // SEI
        cpu.ram.write(0x03, 0x00, 0x18); // CLC
        cpu.ram.write(0x04, 0x00, 0xd8); // CLD
        cpu.ram.write(0x05, 0x00, 0x58); // CLI
        cpu.ram.write(0x06, 0x00, 0xb8); // CLV

        // fake a "set overflow" instruction
        cpu.status_register.set_bit(6);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0001);
        assert_eq!(cpu.status_register.read(), 0b01100001);

        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0002);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.status_register.read(), 0b01101001);

        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0003);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.status_register.read(), 0b01101101);

        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0004);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0004);
        assert_eq!(cpu.status_register.read(), 0b01101100);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0005);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0005);
        assert_eq!(cpu.status_register.read(), 0b01100100);

        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0006);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0006);
        assert_eq!(cpu.status_register.read(), 0b01100000);

        cpu.cycle();
        assert_eq!(cpu.state, State::Implied);
        assert_eq!(cpu.program_counter.read(), 0x0007);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0007);
        assert_eq!(cpu.status_register.read(), 0b00100000);
    }

    #[test]
    fn test_stx_zero_page_indexed_y() {
        let mut cpu = Cpu::new();
        
        // STX ($13,y)
        cpu.ram.write(0x00, 0x00, 0x96);
        cpu.ram.write(0x01, 0x00, 0x43);
        cpu.index_x.write(0x99);
        cpu.index_y.write(0xff);
        
        // effective address = 0x43 + 0xff = 0x42

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::ZeroPageIndexedY);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0043);

        cpu.cycle();
        assert_eq!(cpu.state, State::WriteStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0042);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.ram.read(0x42, 0x00), 0x99);
    }

    #[test]
    fn test_asl_absolute() {
        let mut cpu = Cpu::new();
        
        // ASL $0123
        cpu.ram.write(0x00, 0x00, 0x0e);
        cpu.ram.write(0x01, 0x00, 0x23);
        cpu.ram.write(0x02, 0x00, 0x01);
        cpu.ram.write(0x23, 0x01, 0xe2);

        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteAddressHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0023);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteStart);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.memory_address_register.read(), 0x0123);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteExecute);
        assert_eq!(cpu.memory_data_register.read(), 0xe2);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadWriteCommit);
        assert_eq!(cpu.alu_register.read(), 0xc4);
        assert_eq!(cpu.status_register.read(), 0b10100001);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.ram.read(0x23, 0x01), 0xc4);
    }

    #[test]
    fn test_and_indirect_indexed() {
        let mut cpu = Cpu::new();

        // AND ($12),y => AND $0102,y => AND $0103
        cpu.ram.write(0x00, 0x00, 0x31);
        cpu.ram.write(0x01, 0x00, 0x12);
        cpu.ram.write(0x12, 0x00, 0x02);
        cpu.ram.write(0x13, 0x00, 0x01);
        cpu.ram.write(0x03, 0x01, 0xf3);

        // AND ($14),y => AND $01ff,y => AND $0200
        cpu.ram.write(0x02, 0x00, 0x31);
        cpu.ram.write(0x03, 0x00, 0x14);
        cpu.ram.write(0x14, 0x00, 0xff);
        cpu.ram.write(0x15, 0x00, 0x01);
        cpu.ram.write(0x00, 0x02, 0x2c);

        cpu.accumulator.write(0x8f);
        cpu.index_y.write(0x01);
        
        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndirectIndexedLowByte);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read(), 0x0012);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndirectIndexedHighByte);
        assert_eq!(cpu.memory_address_register.read(), 0x0013);
        assert_eq!(cpu.memory_data_register.read(), 0x02);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0103);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.accumulator.read(), 0x83);
        assert_eq!(cpu.status_register.read(), 0b10100000);

        cpu.cycle();
        assert_eq!(cpu.state, State::OneOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0003);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndirectIndexedLowByte);
        assert_eq!(cpu.program_counter.read(), 0x0004);
        assert_eq!(cpu.memory_address_register.read(), 0x0014);

        cpu.cycle();
        assert_eq!(cpu.state, State::IndirectIndexedHighByte);
        assert_eq!(cpu.memory_address_register.read(), 0x0015);
        assert_eq!(cpu.memory_data_register.read(), 0xff);

        cpu.cycle();
        assert_eq!(cpu.state, State::FixHighByte);
        assert_eq!(cpu.memory_address_register.read(), 0x0100);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0200);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0004);
        assert_eq!(cpu.accumulator.read(), 0x00);
        assert_eq!(cpu.status_register.read(), 0b00100010);
    }

    #[test]
    fn test_cmp_absolute_indexed_x() {
        let mut cpu = Cpu::new();

        // CMP $0123,x => CMP $01a3
        cpu.ram.write(0x00, 0x00, 0xdd);
        cpu.ram.write(0x01, 0x00, 0x23);
        cpu.ram.write(0x02, 0x00, 0x01);
        cpu.ram.write(0xa3, 0x01, 0x33);

        // CMP $0180,x => CMP $0200
        cpu.ram.write(0x03, 0x00, 0xdd);
        cpu.ram.write(0x04, 0x00, 0x80);
        cpu.ram.write(0x05, 0x00, 0x01);
        cpu.ram.write(0x00, 0x02, 0x34);

        cpu.index_x.write(0x80);
        cpu.accumulator.write(0x33);

        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndexedXHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read_low(), 0x23);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.memory_address_register.read(), 0x01a3);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.status_register.read(), 0b00100011);

        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0004);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndexedXHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0005);
        assert_eq!(cpu.memory_address_register.read_low(), 0x80);

        cpu.cycle();
        assert_eq!(cpu.state, State::FixHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0006);
        assert_eq!(cpu.memory_address_register.read(), 0x0100);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0200);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0006);
        assert_eq!(cpu.status_register.read(), 0b10100000);
    }

    #[test]
    fn test_ora_absolute_indexed_y() {
        let mut cpu = Cpu::new();

        // ORA $0123,x => ORA $01a3
        cpu.ram.write(0x00, 0x00, 0x19);
        cpu.ram.write(0x01, 0x00, 0x23);
        cpu.ram.write(0x02, 0x00, 0x01);
        cpu.ram.write(0xa3, 0x01, 0x04);

        // ORA $0180,x => ORA $0200
        cpu.ram.write(0x03, 0x00, 0x19);
        cpu.ram.write(0x04, 0x00, 0x80);
        cpu.ram.write(0x05, 0x00, 0x01);
        cpu.ram.write(0x00, 0x02, 0x83);

        cpu.index_y.write(0x80);
        cpu.accumulator.write(0x23);

        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0001);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndexedYHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0002);
        assert_eq!(cpu.memory_address_register.read_low(), 0x23);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.memory_address_register.read(), 0x01a3);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0003);
        assert_eq!(cpu.accumulator.read(), 0x27);
        assert_eq!(cpu.status_register.read(), 0b00100000);

        cpu.cycle();
        assert_eq!(cpu.state, State::TwoOperandStart);
        assert_eq!(cpu.program_counter.read(), 0x0004);

        cpu.cycle();
        assert_eq!(cpu.state, State::AbsoluteIndexedYHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0005);
        assert_eq!(cpu.memory_address_register.read_low(), 0x80);

        cpu.cycle();
        assert_eq!(cpu.state, State::FixHighByte);
        assert_eq!(cpu.program_counter.read(), 0x0006);
        assert_eq!(cpu.memory_address_register.read(), 0x0100);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadStart);
        assert_eq!(cpu.memory_address_register.read(), 0x0200);

        cpu.cycle();
        assert_eq!(cpu.state, State::ReadOpcode);
        assert_eq!(cpu.program_counter.read(), 0x0006);
        assert_eq!(cpu.accumulator.read(), 0xa7);
        assert_eq!(cpu.status_register.read(), 0b10100000);
    }
}
