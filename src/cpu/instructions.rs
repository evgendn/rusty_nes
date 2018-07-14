// Explanation of instructions: http://obelisk.me.uk/6502/reference.html
use cpu::cpu;

// Load and Store operations
pub fn lda(cpu: &mut cpu::CPU, address: u8)  {
    println!("LDA performed");
}

// Arithmetic operations
pub fn adc(cpu: &mut cpu::CPU, address: u8) {
    println!("ADC performed");
    let fetched = cpu.ram.read_byte(address as u16);
    let mut result = cpu.a_reg as u16 + fetched as u16;

    if cpu.p_reg.get_carry_flag()  == 1{
        result += 1;
    }

    cpu.p_reg.set_carry_flag(result > 0xff);
    cpu.p_reg.set_zero_flag(result == 0x0);

    let acc = cpu.a_reg;
    cpu.p_reg.set_overflow_flag((acc ^ fetched) & (acc ^ result as u8) & 0x80 != 0);

    cpu.p_reg.set_negative_flag(result & 0x80 == 0x80);
}
