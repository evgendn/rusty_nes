use rom;
use utils;

// Implementation of status register and stuff for it
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

    pub fn set_carry_flag(&self, value: bool) {
        utils::set_bit(self.data, Status::CarryFlag, value);
    } 

    pub fn set_zero_flag(&self, value: bool) {
        utils::set_bit(self.data, Status::ZeroFlag, value);
    }

    pub fn set_interrupt_flag(&self, value: bool) {
        utils::set_bit(self.data, Status::InterruptFlag, value);
    }

    pub fn set_decimal_flag(&self, value: bool) {
        utils::set_bit(self.data, Status::DecimalMode, value);
    }

    pub fn set_breakpoint(&self, value: bool) {
        utils::set_bit(self.data, Status::Breakpoint, value);
    }

    pub fn set_overflow_flag(&self, value: bool) {
        utils::set_bit(self.data, Status::OverflowFlag, value);
    }

    pub fn set_negative_flag(&self, value: bool) {
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





