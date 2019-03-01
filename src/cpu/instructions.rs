// Explanation of instructions: http://obelisk.me.uk/6502/reference.html
use cpu::cpu;
use utils::get_bit;

// --------------- Load and Store operations ---------------
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

// --------------- Register Transfers ---------------
pub fn tax(cpu: &mut cpu::CPU, address: u8) {
    cpu.x_reg = cpu.a_reg;
    let x = cpu.x_reg;

    cpu.p_reg.set_zero_flag(x == 0x0);
    cpu.p_reg.set_negative_flag(x & 0x80 == 0x80);
}

pub fn tay(cpu: &mut cpu::CPU, address: u8) {
    cpu.y_reg = cpu.a_reg;
    let y = cpu.y_reg;

    cpu.p_reg.set_zero_flag(y == 0x0);
    cpu.p_reg.set_negative_flag(y & 0x80 == 0x80);
}

pub fn txa(cpu: &mut cpu::CPU, address: u8) {
    cpu.a_reg = cpu.x_reg;
    let accum = cpu.a_reg;

    cpu.p_reg.set_zero_flag(accum == 0x0);
    cpu.p_reg.set_negative_flag(accum & 0x80 == 0x80);
}

pub fn tya(cpu: &mut cpu::CPU, address: u8) {
    cpu.a_reg = cpu.y_reg;
    let accum = cpu.a_reg;

    cpu.p_reg.set_zero_flag(accum == 0x0);
    cpu.p_reg.set_negative_flag(accum & 0x80 == 0x80);
}

// --------------- Stack Operations ---------------
pub fn tsx(cpu: &mut cpu::CPU, address: u8) {
    cpu.x_reg = cpu.sp_reg;
    let x = cpu.x_reg;

    cpu.p_reg.set_zero_flag(x == 0x0);
    cpu.p_reg.set_negative_flag(x & 0x80 == 0x80);
}

pub fn txs(cpu: &mut cpu::CPU, address: u8) {
    cpu.sp_reg = cpu.x_reg;
}

pub fn pha(cpu: &mut cpu::CPU, address: u8) {
    let accum = cpu.a_reg;
    cpu.push(accum);
}

pub fn php(cpu: &mut cpu::CPU, address: u8) {
    let status = cpu.p_reg.data;
    cpu.push(status);
}

pub fn pla(cpu: &mut cpu::CPU, address: u8) {
    cpu.a_reg = cpu.pull();
    let accum = cpu.a_reg;

    cpu.p_reg.set_zero_flag(accum == 0x0);
    cpu.p_reg.set_negative_flag(accum & 0x80 == 0x80);
}

pub fn plp(cpu: &mut cpu::CPU, address: u8) {
    cpu.p_reg.data = cpu.pull(); 
}

// --------------- Logical --------------- 
pub fn and(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let accum = cpu.a_reg & memory; 
    cpu.a_reg = accum;

    cpu.p_reg.set_zero_flag(accum == 0x0);
    cpu.p_reg.set_negative_flag(accum & 0x80 == 0x80);
}

pub fn eor(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let accum = cpu.a_reg ^ memory;
    cpu.a_reg = accum;

    cpu.p_reg.set_zero_flag(accum == 0x0);
    cpu.p_reg.set_negative_flag(accum & 0x80 == 0x80);
}

pub fn nop(cpu: &mut cpu::CPU, address: u8) {
    // Empty instruction. It's just increment program counter.
}

pub fn bit(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let accum = cpu.a_reg;
    let m6 = memory & 0x40; 
    let m7 = memory & 0x80;

    cpu.p_reg.set_zero_flag(accum & memory == 0x0);
    cpu.p_reg.set_overflow_flag(m6 == 0x40);
    cpu.p_reg.set_negative_flag(m7 == 0x80);
}

// --------------- Arithmetic operations ---------------
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

pub fn cpx(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let x = cpu.x_reg;
    let result = x - memory;

    cpu.p_reg.set_carry_flag(x >= memory);
    cpu.p_reg.set_zero_flag(x == memory);
    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}

pub fn cpy(cpu: &mut cpu::CPU, address: u8) {
    let memory = cpu.ram.read_byte(address as u16);
    let y = cpu.y_reg;
    let result = y - memory;

    cpu.p_reg.set_carry_flag(y >= memory);
    cpu.p_reg.set_zero_flag(y == memory);
    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}
