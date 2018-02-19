extern crate lazy_static;

use std::collections::HashMap;

pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

pub enum Label {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

pub struct OpCode {
    pub label: Label,
    pub mode: AddressingMode,
    pub size: u8,
}

lazy_static! {
    static ref MAP: HashMap<u8, OpCode> = {
        let mut opcodes: HashMap<u8, OpCode>  = HashMap::new();

        opcodes.insert(0x69, OpCode{ADC, AddressingMode::Immediate, 2});
        opcodes.insert(0x65, OpCode{ADC, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x75, OpCode{ADC, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x6d, OpCode{ADC, AddressingMode::Absolute, 3});
        opcodes.insert(0x7d, OpCode{ADC, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x79, OpCode{ADC, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0x61, OpCode{ADC, AddressingMode::IndirectX, 2});
        opcodes.insert(0x71, OpCode{ADC, AddressingMode::IndirectY, 2});
        opcodes.insert(0x29, OpCode{AND, AddressingMode::Immediate, 2});
        opcodes.insert(0x25, OpCode{AND, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x35, OpCode{AND, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x2d, OpCode{AND, AddressingMode::Absolute, 3});
        opcodes.insert(0x3d, OpCode{AND, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x39, OpCode{AND, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0x21, OpCode{AND, AddressingMode::IndirectX, 2});
        opcodes.insert(0x31, OpCode{AND, AddressingMode::IndirectY, 2});
        opcodes
    };
}

