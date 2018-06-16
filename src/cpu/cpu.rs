use rom;
use utils;

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

enum Status {
    CarryFlag = 0,
    ZeroFlag = 1,
    InterruptFlag = 2,
    DecimalMode = 3,
    Breakpoint = 4,
    OverflowFlag = 6,
    NegativeFlag = 7,
}
struct StatusRegister {
    data: u8,
}

impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            // IRQ disabled
            data: 0x34,
        }
    }

    pub fn get_carry_flag(&mut self) -> bool {
        utils::get_bit(self.data, Status::CarryFlag)
    }

    pub fn get_zero_flag(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::ZeroFlag)
    }

    pub fn get_interrupt_flag(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::InterruptFlag)
    }

    pub fn get_decimal_flag(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::DecimalMode)
    }

    pub fn get_breakpoint(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::Breakpoint)
    }

    pub fn get_overflow_flag(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::OverflowFlag)
    }

    pub fn get_negative_flag(&mut self) -> bool {
        utils::get_bit(sefl.data, Status::NegativeFlag)
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::CarryFlag, value);
    } 

    pub fn set_zero_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::ZeroFlag, value);
    }

    pub fn set_interrupt_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::InterruptFlag, value);
    }

    pub fn set_decimal_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::DecimalMode, value);
    }

    pub fn set_breakpoint(&mut self, value: bool) {
        utils::set_bit(self.data, Status::Breakpoint, value);
    }

    pub fn set_overflow_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::OverflowFlag, value);
    }

    pub fn set_negative_flag(&mut self, value: bool) {
        utils::set_bit(self.data, Status::NegativeFlag, value);
    }
}  

pub struct CPU {
    // Accumulator register
    a_reg: u8,
    // Index registers
    x_reg: u8,
    y_reg: u8,
    // Processor status flag bits
    p_reg: StatusRegister,
    // Stack pointer
    sp_reg: u8,
    // Program counter
    pc_reg: u16,
}

impl CPU {
    // Power up
    pub fn new() -> CPU {
        CPU {
            a_reg: 0x0,
            x_reg: 0x0,
            y_reg; 0x0,
            p_reg: StatusRegister::new(),
            sp_reg: 0xfd,
            pc_reg: 0x0,
        }
    }

    pub fn reset(&self) {
        self.sp_reg -= 3; 
        self.p_reg.set_interrupt_flag(true);
    }
}

