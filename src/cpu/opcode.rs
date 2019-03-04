use std::collections::HashMap;

#[derive(Debug)]
pub struct OpCode {
    pub label: Label,
    pub mode: AddressingMode,
    pub size: u8,
    pub cycles: u8,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

lazy_static! {
    pub static ref MAP: HashMap<u8, OpCode> = {
        let mut opcodes: HashMap<u8, OpCode>  = HashMap::new();

        opcodes.insert(0x69, OpCode{label: Label::ADC, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0x65, OpCode{label: Label::ADC, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x75, OpCode{label: Label::ADC, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x6d, OpCode{label: Label::ADC, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x7d, OpCode{label: Label::ADC, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0x79, OpCode{label: Label::ADC, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0x61, OpCode{label: Label::ADC, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0x71, OpCode{label: Label::ADC, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0x29, OpCode{label: Label::AND, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0x25, OpCode{label: Label::AND, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x35, OpCode{label: Label::AND, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x2d, OpCode{label: Label::AND, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x3d, OpCode{label: Label::AND, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0x39, OpCode{label: Label::AND, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0x21, OpCode{label: Label::AND, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0x31, OpCode{label: Label::AND, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0x0a, OpCode{label: Label::ASL, mode: AddressingMode::Accumulator, size: 1, cycles: 2});
        opcodes.insert(0x06, OpCode{label: Label::ASL, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0x16, OpCode{label: Label::ASL, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0x0e, OpCode{label: Label::ASL, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0x1e, OpCode{label: Label::ASL, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0x90, OpCode{label: Label::BCC, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0xb0, OpCode{label: Label::BCS, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0xf0, OpCode{label: Label::BEQ, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0x24, OpCode{label: Label::BIT, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x2c, OpCode{label: Label::BIT, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x30, OpCode{label: Label::BMI, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0xd0, OpCode{label: Label::BNE, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0x10, OpCode{label: Label::BPL, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0x00, OpCode{label: Label::BRK, mode: AddressingMode::Implicit, size: 1, cycles: 7});
        opcodes.insert(0x50, OpCode{label: Label::BVC, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0x70, OpCode{label: Label::BVS, mode: AddressingMode::Relative, size: 2, cycles: 2});
        opcodes.insert(0x18, OpCode{label: Label::CLC, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xd8, OpCode{label: Label::CLD, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x58, OpCode{label: Label::CLI, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xb8, OpCode{label: Label::CLV, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xc9, OpCode{label: Label::CMP, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xc5, OpCode{label: Label::CMP, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xd5, OpCode{label: Label::CMP, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0xcd, OpCode{label: Label::CMP, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xdd, OpCode{label: Label::CMP, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0xd9, OpCode{label: Label::CMP, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0xc1, OpCode{label: Label::CMP, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0xd1, OpCode{label: Label::CMP, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0xe0, OpCode{label: Label::CPX, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xe4, OpCode{label: Label::CPX, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xec, OpCode{label: Label::CPX, mode: AddressingMode::Absolute, size: 2, cycles: 4});
        opcodes.insert(0xc0, OpCode{label: Label::CPY, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xc4, OpCode{label: Label::CPY, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xcc, OpCode{label: Label::CPY, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xc6, OpCode{label: Label::DEC, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0xd6, OpCode{label: Label::DEC, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0xce, OpCode{label: Label::DEC, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0xde, OpCode{label: Label::DEC, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0xca, OpCode{label: Label::DEX, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x88, OpCode{label: Label::DEY, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x49, OpCode{label: Label::EOR, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0x45, OpCode{label: Label::EOR, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x55, OpCode{label: Label::EOR, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x4d, OpCode{label: Label::EOR, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x5d, OpCode{label: Label::EOR, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0x59, OpCode{label: Label::EOR, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0x41, OpCode{label: Label::EOR, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0x51, OpCode{label: Label::EOR, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0xe6, OpCode{label: Label::INC, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0xf6, OpCode{label: Label::INC, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0xee, OpCode{label: Label::INC, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0xfe, OpCode{label: Label::INC, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0xe8, OpCode{label: Label::INX, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xc8, OpCode{label: Label::INY, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x4c, OpCode{label: Label::JMP, mode: AddressingMode::Absolute, size: 3, cycles: 3});
        opcodes.insert(0x6c, OpCode{label: Label::JMP, mode: AddressingMode::Indirect, size: 3, cycles: 5});
        opcodes.insert(0x20, OpCode{label: Label::JSR, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0xa9, OpCode{label: Label::LDA, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xa5, OpCode{label: Label::LDA, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xb5, OpCode{label: Label::LDA, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0xad, OpCode{label: Label::LDA, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xbd, OpCode{label: Label::LDA, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0xb9, OpCode{label: Label::LDA, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0xa1, OpCode{label: Label::LDA, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0xb1, OpCode{label: Label::LDA, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0xa2, OpCode{label: Label::LDX, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xa6, OpCode{label: Label::LDX, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xb6, OpCode{label: Label::LDX, mode: AddressingMode::ZeroPageY, size: 2, cycles: 4});
        opcodes.insert(0xae, OpCode{label: Label::LDX, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xbe, OpCode{label: Label::LDX, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0xa0, OpCode{label: Label::LDY, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xa4, OpCode{label: Label::LDY, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xb4, OpCode{label: Label::LDY, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0xac, OpCode{label: Label::LDY, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xbc, OpCode{label: Label::LDY, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0x4a, OpCode{label: Label::LSR, mode: AddressingMode::Accumulator, size: 1, cycles: 2});
        opcodes.insert(0x46, OpCode{label: Label::LSR, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0x56, OpCode{label: Label::LSR, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0x4e, OpCode{label: Label::LSR, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0x5e, OpCode{label: Label::LSR, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0xea, OpCode{label: Label::NOP, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x09, OpCode{label: Label::ORA, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0x05, OpCode{label: Label::ORA, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x15, OpCode{label: Label::ORA, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x0d, OpCode{label: Label::ORA, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x1d, OpCode{label: Label::ORA, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0x19, OpCode{label: Label::ORA, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0x01, OpCode{label: Label::ORA, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0x11, OpCode{label: Label::ORA, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0x48, OpCode{label: Label::PHA, mode: AddressingMode::Implicit, size: 1, cycles: 3});
        opcodes.insert(0x08, OpCode{label: Label::PHP, mode: AddressingMode::Implicit, size: 1, cycles: 3});
        opcodes.insert(0x68, OpCode{label: Label::PLA, mode: AddressingMode::Implicit, size: 1, cycles: 4});
        opcodes.insert(0x28, OpCode{label: Label::PLP, mode: AddressingMode::Implicit, size: 1, cycles: 4});
        opcodes.insert(0x2a, OpCode{label: Label::ROL, mode: AddressingMode::Accumulator, size: 1, cycles: 2});
        opcodes.insert(0x26, OpCode{label: Label::ROL, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0x36, OpCode{label: Label::ROL, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0x2e, OpCode{label: Label::ROL, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0x3e, OpCode{label: Label::ROL, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0x6a, OpCode{label: Label::ROR, mode: AddressingMode::Accumulator, size: 1, cycles: 2});
        opcodes.insert(0x66, OpCode{label: Label::ROR, mode: AddressingMode::ZeroPage, size: 2, cycles: 5});
        opcodes.insert(0x76, OpCode{label: Label::ROR, mode: AddressingMode::ZeroPageX, size: 2, cycles: 6});
        opcodes.insert(0x6e, OpCode{label: Label::ROR, mode: AddressingMode::Absolute, size: 3, cycles: 6});
        opcodes.insert(0x7e, OpCode{label: Label::ROR, mode: AddressingMode::AbsoluteX, size: 3, cycles: 7});
        opcodes.insert(0x40, OpCode{label: Label::RTI, mode: AddressingMode::Implicit, size: 1, cycles: 6});
        opcodes.insert(0x60, OpCode{label: Label::RTS, mode: AddressingMode::Implicit, size: 1, cycles: 6});
        opcodes.insert(0xe9, OpCode{label: Label::SBC, mode: AddressingMode::Immediate, size: 2, cycles: 2});
        opcodes.insert(0xe5, OpCode{label: Label::SBC, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0xf5, OpCode{label: Label::SBC, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0xed, OpCode{label: Label::SBC, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xfd, OpCode{label: Label::SBC, mode: AddressingMode::AbsoluteX, size: 3, cycles: 4});
        opcodes.insert(0xf9, OpCode{label: Label::SBC, mode: AddressingMode::AbsoluteY, size: 3, cycles: 4});
        opcodes.insert(0xe1, OpCode{label: Label::SBC, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0xf1, OpCode{label: Label::SBC, mode: AddressingMode::IndirectY, size: 2, cycles: 5});
        opcodes.insert(0x38, OpCode{label: Label::SEC, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xf8, OpCode{label: Label::SED, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x78, OpCode{label: Label::SEI, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x85, OpCode{label: Label::STA, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x95, OpCode{label: Label::STA, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x8d, OpCode{label: Label::STA, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x9d, OpCode{label: Label::STA, mode: AddressingMode::AbsoluteX, size: 3, cycles: 5});
        opcodes.insert(0x99, OpCode{label: Label::STA, mode: AddressingMode::AbsoluteY, size: 3, cycles: 5});
        opcodes.insert(0x81, OpCode{label: Label::STA, mode: AddressingMode::IndirectX, size: 2, cycles: 6});
        opcodes.insert(0x91, OpCode{label: Label::STA, mode: AddressingMode::IndirectY, size: 2, cycles: 6});
        opcodes.insert(0x86, OpCode{label: Label::STX, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x96, OpCode{label: Label::STX, mode: AddressingMode::ZeroPageY, size: 2, cycles: 4});
        opcodes.insert(0x8e, OpCode{label: Label::STX, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0x84, OpCode{label: Label::STY, mode: AddressingMode::ZeroPage, size: 2, cycles: 3});
        opcodes.insert(0x94, OpCode{label: Label::STY, mode: AddressingMode::ZeroPageX, size: 2, cycles: 4});
        opcodes.insert(0x8c, OpCode{label: Label::STY, mode: AddressingMode::Absolute, size: 3, cycles: 4});
        opcodes.insert(0xaa, OpCode{label: Label::TAX, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xa8, OpCode{label: Label::TAY, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0xba, OpCode{label: Label::TSX, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x8a, OpCode{label: Label::TXA, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x9a, OpCode{label: Label::TXS, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes.insert(0x98, OpCode{label: Label::TYA, mode: AddressingMode::Implicit, size: 1, cycles: 2});
        opcodes
    };
}
