// Explanation of instructions: http://obelisk.me.uk/6502/reference.html
use cpu::cpu;

// Load and Store operations
pub fn lda(cpu: &mut cpu::CPU, address: u8)  {
    println!("LDA performed");
    let memory = cpu.ram.read_byte(address as u16);
    cpu.a_reg = memory;

    if cpu.a_reg == 0x0 {
        cpu.p_reg.set_zero_flag(false);
    }
}

// Arithmetic operations
pub fn adc(cpu: &mut cpu::CPU, address: u8) {
    println!("ADC performed");
    let memory = cpu.ram.read_byte(address as u16);
    let carry = cpu.p_reg.get_carry_flag() as u8;
    let accum = cpu.a_reg;
    let result = accum as u16 + memory as u16 + carry as u16;
    cpu.a_reg = result as u8;

    cpu.p_reg.set_carry_flag(result > 0xff);
    cpu.p_reg.set_zero_flag(result == 0x0);

    let acc = cpu.a_reg;
    cpu.p_reg.set_overflow_flag((acc ^ memory) & (acc ^ result as u8) & 0x80 != 0);

    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}
