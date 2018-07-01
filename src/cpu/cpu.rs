use rom;
use utils;
use nes::NES;
use cpu::opcode::*;
use cpu::instruction::*;

#[derive(Debug)]
pub struct CPU {
    // Accumulator register
    pub a_reg: u8,
    // Index registers
    pub x_reg: u8,
    pub y_reg: u8,
    // Processor status flag bits
    pub p_reg: StatusRegister,
    // Stack pointer
    pub sp_reg: u8,
    // Program counter
    pub pc_reg: u16,
}

// 7  bit  0
// ---- ----
// NVss DIZC
// |||| ||||
// |||| |||+- Carry: 1 if last addition or shift resulted in a carry, or if
// |||| |||     last subtraction resulted in no borrow
// |||| ||+-- Zero: 1 if last operation resulted in a 0 value
// |||| |+--- Interrupt: Interrupt inhibit
// |||| |       (0: /IRQ and /NMI get through; 1: only /NMI gets through)
// |||| +---- Decimal: 1 to make ADC and SBC use binary-coded decimal arithmetic
// ||||         (ignored on second-source 6502 like that in the NES)
// ||++------ s: No effect, used by the stack copy, see note below
// |+-------- Overflow: 1 if last ADC or SBC resulted in signed overflow,
// |            or D6 from last BIT
// +--------- Negative: Set to bit 7 of the last operation

#[derive(Debug)]
enum Status {
    CarryFlag = 0,
    ZeroFlag = 1,
    InterruptFlag = 2,
    DecimalMode = 3,
    Breakpoint = 4,
    OverflowFlag = 6,
    NegativeFlag = 7,
}

#[derive(Debug)]
pub struct StatusRegister {
    data: u8,
}

impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            // IRQ disabled
            data: 0x34,
        }
    }

    pub fn get_carry_flag(&self) -> u8 {
        utils::get_bit(&self.data, Status::CarryFlag as u8)
    }

    pub fn get_zero_flag(&self) -> u8 {
        utils::get_bit(&self.data, Status::ZeroFlag as u8)
    }

    pub fn get_interrupt_flag(&self) -> u8 {
        utils::get_bit(&self.data, Status::InterruptFlag as u8)
    }

    pub fn get_decimal_flag(&self) -> u8 {
        utils::get_bit(&self.data, Status::DecimalMode as u8)
    }

    pub fn get_breakpoint(&self) -> u8 {
        utils::get_bit(&self.data, Status::Breakpoint as u8)
    }

    pub fn get_overflow_flag(&self) -> u8 {
        utils::get_bit(&self.data, Status::OverflowFlag as u8)
    }

    pub fn get_negative_flag(&mut self) -> u8 {
        utils::get_bit(&self.data, Status::NegativeFlag as u8)
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::CarryFlag as u8, value);
    } 

    pub fn set_zero_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::ZeroFlag as u8, value);
    }

    pub fn set_interrupt_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::InterruptFlag as u8, value);
    }

    pub fn set_decimal_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::DecimalMode as u8, value);
    }

    pub fn set_breakpoint(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::Breakpoint as u8, value);
    }

    pub fn set_overflow_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::OverflowFlag as u8, value);
    }

    pub fn set_negative_flag(&mut self, value: bool) {
        utils::set_bit(&mut self.data, Status::NegativeFlag as u8, value);
    }
} 

impl CPU {
    // Power up
    pub fn new() -> CPU {
        CPU {
            a_reg: 0x0,
            x_reg: 0x0,
            y_reg: 0x0,
            p_reg: StatusRegister::new(),
            sp_reg: 0xfd,
            pc_reg: 0x0,
        }
    }

    pub fn reset(&mut self) {
        self.sp_reg -= 3; 
        self.p_reg.set_interrupt_flag(true);
    }

    pub fn tick(&mut self, nes: &mut NES) {
        let opcodes= &MAP;
        let address = self.pc_reg as u8;
        let opcode = opcodes.get(&address).unwrap();
        println!("{:?}", opcode);
        let fetched_address = self.chose_addressing_mode(nes, opcode);
        self.execute_instruction(nes, fetched_address, opcode);
    }

    pub fn chose_addressing_mode(&mut self, nes: &mut NES, opcode: &OpCode) -> u16 {
        match opcode.mode {
            AddressingMode::Implicit => 0x0000,
            AddressingMode::Accumulator => 0x0000,
            AddressingMode::Immediate => self.fetch_immidiate(nes) as u16,
            AddressingMode::ZeroPage => self.fetch_zero_page(nes),
            AddressingMode::ZeroPageX => self.fetch_zero_page_x(nes),
            AddressingMode::ZeroPageY => self.fetch_zero_page_y(nes),
            AddressingMode::Absolute => self.fetch_absolute(nes),
            AddressingMode::AbsoluteX => self.fetch_absolute_x(nes),
            AddressingMode::AbsoluteY => self.fetch_absolute_y(nes),
            AddressingMode::Relative => self.fetch_relative(nes) as u16,
            AddressingMode::IndirectX => self.fetch_indirect_x(nes),
            AddressingMode::IndirectY => self.fetch_indirect_y(nes),
            _ => 0x0
        } 
    }

    pub fn fetch_immidiate(&mut self, nes: &mut NES) -> u8 {
        let address = nes.wram.read_byte(self.pc_reg);
        self.pc_reg += 1;
        address
    }

    pub fn fetch_zero_page(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_byte(self.pc_reg);
        self.pc_reg += 1;
        address as u16
    }

    pub fn fetch_zero_page_x(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_byte(self.pc_reg);
        self.pc_reg += 1;
        (address as u16 + self.x_reg as u16) & 0xff as u16
    }

    pub fn fetch_zero_page_y(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_byte(self.pc_reg);
        self.pc_reg += 1;
        (address as u16 + self.y_reg as u16) & 0xff as u16
    }

    pub fn fetch_relative(&mut self, nes: &mut NES) -> u8 {
        let address = nes.wram.read_byte(self.pc_reg);
        self.pc_reg += 1;
        if address > 0x80 {
            address + self.pc_reg as u8
        } else {
            address + (0x100 - self.pc_reg) as u8
        }
    }

    pub fn fetch_absolute(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_word(self.pc_reg);
        self.pc_reg += 2;
        address & 0xffff
    }

    pub fn fetch_absolute_x(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_word(self.pc_reg);
        self.pc_reg += 2;
        (address + self.x_reg as u16) & 0xffff
    }
    
    pub fn fetch_absolute_y(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_word(self.pc_reg);
        self.pc_reg += 2;
        (address + self.y_reg as u16) & 0xffff
    }

    pub fn fetch_indirect_x(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_byte(self.pc_reg);
        let tmp = ((address + self.x_reg) & 0xff) as u16;
        let result = nes.wram.read_word(tmp);
        self.pc_reg += 1;

        (nes.wram.read_word(result) + nes.wram.read_word(result + 1) * 0x100)
    }

    pub fn fetch_indirect_y(&mut self, nes: &mut NES) -> u16 {
        let address = nes.wram.read_word(self.pc_reg);
        let base = ((nes.wram.read_word(address) + nes.wram.read_word(address + 1)) & 0x00ff) * 0x100;
        self.pc_reg += 1;

        (base as u16 + self.y_reg as u16) & 0xffff
    }

    pub fn execute_instruction(&self, nes: &mut NES, address: u16, opcode: &OpCode) {
        match opcode.label {
            Label::ADC => adc(nes, address as u8),
            Label::LDA => lda(nes, address as u8),
            _ => println!("Other instructions")
        }
    }
}

