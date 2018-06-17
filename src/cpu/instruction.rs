// Explanation of instructions: http://obelisk.me.uk/6502/reference.html

use super::cpu::CPU;
use super::bus::Bus;

// Load and Store operations
pub fn lda(opcode: u8, bus: &mut Bus) {

}

// Arithmetic operations
pub fn adc(opcode: u8, bus: &mut Bus) {
    let fetched = bus.read(opcode);
    let result = bus.cpu.a_reg as u32 + fetched as u32;

    if cpu.p_reg.get_carry_flag() {
        result += 1;
    }

    bus.cpu.p_reg.set_carry_flag(result > 0xff);
    bus.cpu.p_reg.set_zero_flag(result == 0x0);
    
    let acc = bus.cpu.a_reg();
    bus.cpu.p_reg.set_overflow_flag((acc ^ fetched) & (acc ^ result) & 0x80 != 0);

    bus.cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}

