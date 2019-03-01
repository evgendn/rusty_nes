// Explanation of instructions: http://obelisk.me.uk/6502/reference.html
use cpu::cpu;
use utils::get_bit;

// Load and Store operations
pub fn lda(cpu: &mut cpu::CPU, address: u8)  {
    println!("LDA performed");
    let memory = cpu.ram.read_byte(address as u16);
    cpu.a_reg = memory;

    if cpu.a_reg == 0x0 {
        cpu.p_reg.set_zero_flag(false);
    }

    if get_bit(&cpu.a_reg, 7) == 1 {
        cpu.p_reg.set_negative_flag(true);
    }
}

pub fn ldx(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    cpu.x_reg = memory;

    if cpu.x_reg == 0x0 {
        cpu.p_reg.set_zero_flag(false);
    }

    if get_bit(&cpu.x_reg, 7) == 1 {
        cpu.p_reg.set_negative_flag(true);
    } 
}

pub fn ldy(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    cpu.y_reg = memory;

    if cpu.y_reg == 0x0 {
        cpu.p_reg.set_zero_flag(false);
    }

    if get_bit(&cpu.y_reg, 7) == 1 {
        cpu.p_reg.set_negative_flag(true);
    } 
}

pub fn sta(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    cpu.a_reg = memory;
}

pub fn stx(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    cpu.x_reg = memory;
}

pub fn sty(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    cpu.y_reg = memory;
}
// Arithmetic operations
pub fn adc(cpu: &mut cpu::CPU, address: u8) {
    println!("ADC performed");
    let memory = cpu.ram.read_byte(address as u16);
    let carry = cpu.p_reg.get_carry_flag() as u8;
    let accum = cpu.a_reg;
    cpu.a_reg = accum + memory + carry;

    let result = accum as u16 + memory as u16 + carry as u16;
    cpu.p_reg.set_carry_flag(result > 0xff);
    cpu.p_reg.set_zero_flag(result == 0x0);
    cpu.p_reg.set_overflow_flag((accum ^ memory) & (accum ^ result as u8) & 0x80 != 0);
    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}

pub fn sbc(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let carry = 1u8 - cpu.p_reg.get_carry_flag();
    let accum = cpu.a_reg;
    cpu.a_reg = accum + memory + carry;
    
    let result = accum as u16 + memory as u16 + carry as u16;
    cpu.p_reg.set_carry_flag(result > 0xff);
    cpu.p_reg.set_zero_flag(result == 0x0);
    cpu.p_reg.set_overflow_flag((accum ^ memory) & (accum ^ result as u8) & 0x80 != 0);
    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}

pub fn cpm(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let accum = cpu.a_reg;
    let result = accum - memory;

    cpu.p_reg.set_carry_flag(accum >= memory);
    cpu.p_reg.set_zero_flag(accum == memory);
    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}
