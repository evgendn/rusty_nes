// Explanation of instructions: http://obelisk.me.uk/6502/reference.html

use super::cpu::CPU;
use super::bus::Bus;

pub fn adc(opcode: u8, cpu: &mut CPU, bus: &mut Bus) {
    let fetched = bus.read(opcode);
    let result = cpu.a_reg as u32 + fetched as u32;

    if cpu.p_reg.get_carry_flag() {
        result += 1;
    }
}
