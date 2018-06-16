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
        opcodes.insert(0x0a, OpCode{ASL, AddressingMode::Accumulator, 1});
        opcodes.insert(0x06, OpCode{ASL, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x16, OpCode{ASL, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x0e, OpCode{ASL, AddressingMode::Absolute, 3});
        opcodes.insert(0x1e, OpCode{ASL, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x90, OpCode{BCC, AddressingMode::Relative, 2});
        opcodes.insert(0xb0, OpCode{BCS, AddressingMode::Relative, 2});
        opcodes.insert(0xf0, OpCode{BEQ, AddressingMode::Relative, 2});
        opcodes.insert(0x24, OpCode{BIT, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x2c, OpCode{BIT, AddressingMode::Absolute, 3});
        opcodes.insert(0x30, OpCode{BMI, AddressingMode::Relative, 2});
        opcodes.insert(0xd0, OpCode{BNE, AddressingMode::Relative, 2});
        opcodes.insert(0x10, OpCode{BPL, AddressingMode::Relative, 2});
        opcodes.insert(0x00, OpCode{BRK, AddressingMode::Implicit, 1});
        opcodes.insert(0x50, OpCode{BVC, AddressingMode::Relative, 2});
        opcodes.insert(0x70, OpCode{BVS, AddressingMode::Relative, 2});
        opcodes.insert(0x18, OpCode{CLC, AddressingMode::Implicit, 1});
        opcodes.insert(0xd8, OpCode{CLD, AddressingMode::Implicit, 1});
        opcodes.insert(0x58, OpCode{CLI, AddressingMode::Implicit, 1});
        opcodes.insert(0xb8, OpCode{CLV, AddressingMode::Implicit, 1});
        opcodes.insert(0xc9, OpCode{CMP, AddressingMode::Immediate, 2});
        opcodes.insert(0xc5, OpCode{CMP, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xd5, OpCode{CMP, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xcd, OpCode{CMP, AddressingMode::Absolute, 3});
        opcodes.insert(0xdd, OpCode{CMP, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xd9, OpCode{CMP, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0xc1, OpCode{CMP, AddressingMode::IndirectX, 2});
        opcodes.insert(0xd1, OpCode{CMP, AddressingMode::IndirectY, 2});
        opcodes.insert(0xe0, OpCode{CPX, AddressingMode::Immediate, 2});
        opcodes.insert(0xe4, OpCode{CPX, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xec, OpCode{CPX, AddressingMode::Absolute, 2});
        opcodes.insert(0xc0, OpCode{CPY, AddressingMode::Immediate, 2});
        opcodes.insert(0xc4, OpCode{CPY, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xcc, OpCode{CPY, AddressingMode::Absolute, 3});
        opcodes.insert(0xc6, OpCode{DEC, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xd6, OpCode{DEC, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xce, OpCode{DEC, AddressingMode::Absolute, 3});
        opcodes.insert(0xde, OpCode{DEC, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xca, OpCode{DEX, AddressingMode::Implicit, 1});
        opcodes.insert(0x88, OpCode{DEY, AddressingMode::Implicit, 1});
        opcodes.insert(0x49, OpCode{EOR, AddressingMode::Immediate, 2});
        opcodes.insert(0x45, OpCode{EOR, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x55, OpCode{EOR, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x4d, OpCode{EOR, AddressingMode::Absolute, 3});
        opcodes.insert(0x5d, OpCode{EOR, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x59, OpCode{EOR, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0x41, OpCode{EOR, AddressingMode::IndirectX, 2});
        opcodes.insert(0x51, OpCode{EOR, AddressingMode::IndirectY, 2});
        opcodes.insert(0xe6, OpCode{INC, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xf6, OpCode{INC, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xee, OpCode{INC, AddressingMode::Absolute, 3});
        opcodes.insert(0xfe, OpCode{INC, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xe8, OpCode{INX, AddressingMode::Implicit, 1});
        opcodes.insert(0xc8, OpCode{INY, AddressingMode::Implicit, 1})
        opcodes.insert(0x4c, OpCode{JMP, AddressingMode::Absolute, 3});
        opcodes.insert(0x6c, OpCode{JMP, AddressingMode::Indirect, 3});
        opcodes.insert(0x20, OpCode{JSR, AddressingMode::Absolute, 3});
        opcodes.insert(0xa9, OpCode{LDA, AddressingMode::Immediate, 2});
        opcodes.insert(0xa5, OpCode{LDA, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xb5, OpCode{LDA, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xad, OpCode{LDA, AddressingMode::Absolute, 3});
        opcodes.insert(0xbd, OpCode{LDA, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xb9, OpCode{LDA, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0xa1, OpCode{LDA, AddressingMode::IndirectX, 2});
        opcodes.insert(0xb1, OpCode{LDA, AddressingMode::IndirectY, 2});
        opcodes.insert(0xa2, OpCode{LDX, AddressingMode::Immediate, 2});
        opcodes.insert(0xa6, OpCode{LDX, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xb6, OpCode{LDX, AddressingMode::ZeroPageY, 2});
        opcodes.insert(0xae, OpCode{LDX, AddressingMode::Absolute, 3});
        opcodes.insert(0xbe, OpCode{LDX, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0xa0, OpCode{LDY, AddressingMode::Immediate, 2});
        opcodes.insert(0xa4, OpCode{LDY, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xb4, OpCode{LDY, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xac, OpCode{LDY, AddressingMode::Absolute, 3});
        opcodes.insert(0xbc, OpCode{LDY, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x4a, OpCode{LSR, AddressingMode::Accumulator, 1});
        opcodes.insert(0x46, OpCode{LSR, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x56, OpCode{LSR, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x4e, OpCode{LSR, AddressingMode::Absolute, 3});
        opcodes.insert(0x5e, OpCode{LSR, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xea, OpCode{NOP, AddressingMode::Implicit, 1});
        opcodes.insert(0x09, OpCode{ORA, AddressingMode::Immediate, 2});
        opcodes.insert(0x05, OpCode{ORA, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x15, OpCode{ORA, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x0d, OpCode{ORA, AddressingMode::Absolute, 3});
        opcodes.insert(0x1d, OpCode{ORA, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x19, OpCode{ORA, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0x01, OpCode{ORA, AddressingMode::IndirectX, 2});
        opcodes.insert(0x11, OpCode{ORA, AddressingMode::IndirectY, 2});
        opcodes.insert(0x48, OpCode{PHA, AddressingMode::Implicit, 1});
        opcodes.insert(0x08, OpCode{PHP, AddressingMode::Implicit, 1});
        opcodes.insert(0x68, OpCode{PLA, AddressingMode::Implicit, 1});
        opcodes.insert(0x28, OpCode{PLP, AddressingMode::Implicit, 1});
        opcodes.insert(0x2a, OpCode{ROL, AddressingMode::Accumulator, 1});
        opcodes.insert(0x26, OpCode{ROL, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x36, OpCode{ROL, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x2e, OpCode{ROL, AddressingMode::Absolute, 3});
        opcodes.insert(0x3e, OpCode{ROL, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x6a, OpCode{ROR, AddressingMode::Accumulator, 1});
        opcodes.insert(0x66, OpCode{ROR, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x76, OpCode{ROR, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x6e, OpCode{ROR, AddressingMode::Absolute, 3});
        opcodes.insert(0x7e, OpCode{ROR, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x40, OpCode{RTI, AddressingMode::Implicit, 1});
        opcodes.insert(0x60, OpCode{RTS, AddressingMode::Implicit, 1});
        opcodes.insert(0xe9, OpCode{SBC, AddressingMode::Immediate, 2});
        opcodes.insert(0xe5, OpCode{SBC, AddressingMode::ZeroPage, 2});
        opcodes.insert(0xf5, OpCode{SBC, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0xed, OpCode{SBC, AddressingMode::Absolute, 3});
        opcodes.insert(0xfd, OpCode{SBC, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0xf9, OpCode{SBC, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0xe1, OpCode{SBC, AddressingMode::IndirectX, 2});
        opcodes.insert(0xf1, OpCode{SBC, AddressingMode::IndirectY, 2});
        opcodes.insert(0x38, OpCode{SEC, AddressingMode::Implicit, 1});
        opcodes.insert(0xf8, OpCode{SED, AddressingMode::Implicit, 1});
        opcodes.insert(0x78, OpCode{SEI, AddressingMode::Implicit, 1});
        opcodes.insert(0x85, OpCode{STA, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x95, OpCode{STA, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x8d, OpCode{STA, AddressingMode::Absolute, 3});
        opcodes.insert(0x9d, OpCode{STA, AddressingMode::AbsoluteX, 3});
        opcodes.insert(0x99, OpCode{STA, AddressingMode::AbsoluteY, 3});
        opcodes.insert(0x81, OpCode{STA, AddressingMode::IndirectX, 2});
        opcodes.insert(0x91, OpCode{STA, AddressingMode::IndirectY, 2});
        opcodes.insert(0x86, OpCode{STX, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x96, OpCode{STX, AddressingMode::ZeroPageY, 2});
        opcodes.insert(0x8e, OpCode{STX, AddressingMode::Absolute, 3});
        opcodes.insert(0x84, OpCode{STY, AddressingMode::ZeroPage, 2});
        opcodes.insert(0x94, OpCode{STY, AddressingMode::ZeroPageX, 2});
        opcodes.insert(0x8c, OpCode{STY, AddressingMode::Absolute, 3});
        opcodes.insert(0xaa, OpCode{TAX, AddressingMode::Implicit, 1});
        opcodes.insert(0xa8, OpCode{TAY, AddressingMode::Implicit, 1});
        opcodes.insert(0xba, OpCode{TSX, AddressingMode::Implicit, 1});
        opcodes.insert(0x8a, OpCode{TXA, AddressingMode::Implicit, 1});
        opcodes.insert(0x9a, OpCode{TXS, AddressingMode::Implicit, 1});
        opcodes.insert(0x98, OpCode{TYA, AddressingMode::Implicit, 1});
        opcodes
    };
}

