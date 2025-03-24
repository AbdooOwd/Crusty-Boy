use crate::registers::{Reg, Reg16};


#[derive(PartialEq, Debug, Clone)]
pub enum JumpTypes {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    ADD(Reg),
    ADC(Reg),
    ADDHL(Reg16),

    SUB(Reg),
    SBC(Reg),

    AND(Reg),
    OR(Reg),
    XOR(Reg),
    CP(Reg),

    INC(Reg),
    DEC(Reg),

    CCF,
    SCF,

    RRA,
    RLA,
    RRCA,
    RLCA,

    CPL,
    BIT(Reg, u8),
    RESET(Reg, u8),
    SET(Reg, u8),
    SRL(Reg),

    RR(Reg),
    RL(Reg),
    RRC(Reg),
    RLC(Reg),

    SRA(Reg),
    SLA(Reg),
    SWAP(Reg),

    JP(JumpTypes),
    JR(JumpTypes),
    CALL(JumpTypes),
    RET(JumpTypes),

    LD(Reg, Reg),

    PUSH(Reg16),
    POP(Reg16),

    HALT,
    NOP,


    IDK,    // DEBUG PURPOSES! CALLED WHEN IDFK WHAT'S HAPPENING
    EXIT,   // DEBUG PURPQSRPSS
}

impl Instruction {


    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(Reg::B)),
            0x01 => Some(Instruction::RLC(Reg::C)),
            0x02 => Some(Instruction::RLC(Reg::D)),
            0x03 => Some(Instruction::RLC(Reg::E)),
            0x04 => Some(Instruction::RLC(Reg::H)),
            0x05 => Some(Instruction::RLC(Reg::L)),
            0x06 => Some(Instruction::RLC(Reg::HL)),
            0x07 => Some(Instruction::RLC(Reg::A)),

            0x08 => Some(Instruction::RRC(Reg::B)),
            0x09 => Some(Instruction::RRC(Reg::C)),
            0x0a => Some(Instruction::RRC(Reg::D)),
            0x0b => Some(Instruction::RRC(Reg::E)),
            0x0c => Some(Instruction::RRC(Reg::H)),
            0x0d => Some(Instruction::RRC(Reg::L)),
            0x0e => Some(Instruction::RRC(Reg::HL)),
            0x0f => Some(Instruction::RRC(Reg::A)),

            0x10 => Some(Instruction::RL(Reg::B)),
            0x11 => Some(Instruction::RL(Reg::C)),
            0x12 => Some(Instruction::RL(Reg::D)),
            0x13 => Some(Instruction::RL(Reg::E)),
            0x14 => Some(Instruction::RL(Reg::H)),
            0x15 => Some(Instruction::RL(Reg::L)),
            0x16 => Some(Instruction::RL(Reg::HL)),
            0x17 => Some(Instruction::RL(Reg::A)),

            0x18 => Some(Instruction::RR(Reg::B)),
            0x19 => Some(Instruction::RR(Reg::C)),
            0x1a => Some(Instruction::RR(Reg::D)),
            0x1b => Some(Instruction::RR(Reg::E)),
            0x1c => Some(Instruction::RR(Reg::H)),
            0x1d => Some(Instruction::RR(Reg::L)),
            0x1e => Some(Instruction::RR(Reg::HL)),
            0x1f => Some(Instruction::RR(Reg::A)),

            0x20 => Some(Instruction::SLA(Reg::B)),
            0x21 => Some(Instruction::SLA(Reg::C)),
            0x22 => Some(Instruction::SLA(Reg::D)),
            0x23 => Some(Instruction::SLA(Reg::E)),
            0x24 => Some(Instruction::SLA(Reg::H)),
            0x25 => Some(Instruction::SLA(Reg::L)),
            0x26 => Some(Instruction::SLA(Reg::HL)),
            0x27 => Some(Instruction::SLA(Reg::A)),

            0x28 => Some(Instruction::SRA(Reg::B)),
            0x29 => Some(Instruction::SRA(Reg::C)),
            0x2a => Some(Instruction::SRA(Reg::D)),
            0x2b => Some(Instruction::SRA(Reg::E)),
            0x2c => Some(Instruction::SRA(Reg::H)),
            0x2d => Some(Instruction::SRA(Reg::L)),
            0x2e => Some(Instruction::SRA(Reg::HL)),
            0x2f => Some(Instruction::SRA(Reg::A)),

            0x30 => Some(Instruction::SWAP(Reg::B)),
            0x31 => Some(Instruction::SWAP(Reg::C)),
            0x32 => Some(Instruction::SWAP(Reg::D)),
            0x33 => Some(Instruction::SWAP(Reg::E)),
            0x34 => Some(Instruction::SWAP(Reg::H)),
            0x35 => Some(Instruction::SWAP(Reg::L)),
            0x36 => Some(Instruction::SWAP(Reg::HL)),
            0x37 => Some(Instruction::SWAP(Reg::A)),

            0x38 => Some(Instruction::SRL(Reg::B)),
            0x39 => Some(Instruction::SRL(Reg::C)),
            0x3a => Some(Instruction::SRL(Reg::D)),
            0x3b => Some(Instruction::SRL(Reg::E)),
            0x3c => Some(Instruction::SRL(Reg::H)),
            0x3d => Some(Instruction::SRL(Reg::L)),
            0x3e => Some(Instruction::SRL(Reg::HL)),
            0x3f => Some(Instruction::SRL(Reg::A)),

            0x40 => Some(Instruction::BIT(Reg::B, 0)),
            0x41 => Some(Instruction::BIT(Reg::C, 0)),
            0x42 => Some(Instruction::BIT(Reg::D, 0)),
            0x43 => Some(Instruction::BIT(Reg::E, 0)),
            0x44 => Some(Instruction::BIT(Reg::H, 0)),
            0x45 => Some(Instruction::BIT(Reg::L, 0)),
            0x46 => Some(Instruction::BIT(Reg::HL, 0)),
            0x47 => Some(Instruction::BIT(Reg::A, 0)),

            0x48 => Some(Instruction::BIT(Reg::B, 1)),
            0x49 => Some(Instruction::BIT(Reg::C, 1)),
            0x4a => Some(Instruction::BIT(Reg::D, 1)),
            0x4b => Some(Instruction::BIT(Reg::E, 1)),
            0x4c => Some(Instruction::BIT(Reg::H, 1)),
            0x4d => Some(Instruction::BIT(Reg::L, 1)),
            0x4e => Some(Instruction::BIT(Reg::HL, 1)),
            0x4f => Some(Instruction::BIT(Reg::A, 1)),

            0x50 => Some(Instruction::BIT(Reg::B, 2)),
            0x51 => Some(Instruction::BIT(Reg::C, 2)),
            0x52 => Some(Instruction::BIT(Reg::D, 2)),
            0x53 => Some(Instruction::BIT(Reg::E, 2)),
            0x54 => Some(Instruction::BIT(Reg::H, 2)),
            0x55 => Some(Instruction::BIT(Reg::L, 2)),
            0x56 => Some(Instruction::BIT(Reg::HL, 2)),
            0x57 => Some(Instruction::BIT(Reg::A, 2)),

            0x58 => Some(Instruction::BIT(Reg::B, 3)),
            0x59 => Some(Instruction::BIT(Reg::C, 3)),
            0x5a => Some(Instruction::BIT(Reg::D, 3)),
            0x5b => Some(Instruction::BIT(Reg::E, 3)),
            0x5c => Some(Instruction::BIT(Reg::H, 3)),
            0x5d => Some(Instruction::BIT(Reg::L, 3)),
            0x5e => Some(Instruction::BIT(Reg::HL, 3)),
            0x5f => Some(Instruction::BIT(Reg::A, 3)),

            0x60 => Some(Instruction::BIT(Reg::B, 4)),
            0x61 => Some(Instruction::BIT(Reg::C, 4)),
            0x62 => Some(Instruction::BIT(Reg::D, 4)),
            0x63 => Some(Instruction::BIT(Reg::E, 4)),
            0x64 => Some(Instruction::BIT(Reg::H, 4)),
            0x65 => Some(Instruction::BIT(Reg::L, 4)),
            0x66 => Some(Instruction::BIT(Reg::HL, 4)),
            0x67 => Some(Instruction::BIT(Reg::A, 4)),

            0x68 => Some(Instruction::BIT(Reg::B, 5)),
            0x69 => Some(Instruction::BIT(Reg::C, 5)),
            0x6a => Some(Instruction::BIT(Reg::D, 5)),
            0x6b => Some(Instruction::BIT(Reg::E, 5)),
            0x6c => Some(Instruction::BIT(Reg::H, 5)),
            0x6d => Some(Instruction::BIT(Reg::L, 5)),
            0x6e => Some(Instruction::BIT(Reg::HL, 5)),
            0x6f => Some(Instruction::BIT(Reg::A, 5)),

            0x70 => Some(Instruction::BIT(Reg::B, 6)),
            0x71 => Some(Instruction::BIT(Reg::C, 6)),
            0x72 => Some(Instruction::BIT(Reg::D, 6)),
            0x73 => Some(Instruction::BIT(Reg::E, 6)),
            0x74 => Some(Instruction::BIT(Reg::H, 6)),
            0x75 => Some(Instruction::BIT(Reg::L, 6)),
            0x76 => Some(Instruction::BIT(Reg::HL, 6)),
            0x77 => Some(Instruction::BIT(Reg::A, 6)),

            0x78 => Some(Instruction::BIT(Reg::B, 7)),
            0x79 => Some(Instruction::BIT(Reg::C, 7)),
            0x7a => Some(Instruction::BIT(Reg::D, 7)),
            0x7b => Some(Instruction::BIT(Reg::E, 7)),
            0x7c => Some(Instruction::BIT(Reg::H, 7)),
            0x7d => Some(Instruction::BIT(Reg::L, 7)),
            0x7e => Some(Instruction::BIT(Reg::HL, 7)),
            0x7f => Some(Instruction::BIT(Reg::A, 7)),

            0x80 => Some(Instruction::RESET(Reg::B, 0)),
            0x81 => Some(Instruction::RESET(Reg::C, 0)),
            0x82 => Some(Instruction::RESET(Reg::D, 0)),
            0x83 => Some(Instruction::RESET(Reg::E, 0)),
            0x84 => Some(Instruction::RESET(Reg::H, 0)),
            0x85 => Some(Instruction::RESET(Reg::L, 0)),
            0x86 => Some(Instruction::RESET(Reg::HL, 0)),
            0x87 => Some(Instruction::RESET(Reg::A, 0)),

            0x88 => Some(Instruction::RESET(Reg::B, 1)),
            0x89 => Some(Instruction::RESET(Reg::C, 1)),
            0x8a => Some(Instruction::RESET(Reg::D, 1)),
            0x8b => Some(Instruction::RESET(Reg::E, 1)),
            0x8c => Some(Instruction::RESET(Reg::H, 1)),
            0x8d => Some(Instruction::RESET(Reg::L, 1)),
            0x8e => Some(Instruction::RESET(Reg::HL, 1)),
            0x8f => Some(Instruction::RESET(Reg::A, 1)),

            0x90 => Some(Instruction::RESET(Reg::B, 2)),
            0x91 => Some(Instruction::RESET(Reg::C, 2)),
            0x92 => Some(Instruction::RESET(Reg::D, 2)),
            0x93 => Some(Instruction::RESET(Reg::E, 2)),
            0x94 => Some(Instruction::RESET(Reg::H, 2)),
            0x95 => Some(Instruction::RESET(Reg::L, 2)),
            0x96 => Some(Instruction::RESET(Reg::HL, 2)),
            0x97 => Some(Instruction::RESET(Reg::A, 2)),

            0x98 => Some(Instruction::RESET(Reg::B, 3)),
            0x99 => Some(Instruction::RESET(Reg::C, 3)),
            0x9a => Some(Instruction::RESET(Reg::D, 3)),
            0x9b => Some(Instruction::RESET(Reg::E, 3)),
            0x9c => Some(Instruction::RESET(Reg::H, 3)),
            0x9d => Some(Instruction::RESET(Reg::L, 3)),
            0x9e => Some(Instruction::RESET(Reg::HL, 3)),
            0x9f => Some(Instruction::RESET(Reg::A, 3)),

            0xa0 => Some(Instruction::RESET(Reg::B, 4)),
            0xa1 => Some(Instruction::RESET(Reg::C, 4)),
            0xa2 => Some(Instruction::RESET(Reg::D, 4)),
            0xa3 => Some(Instruction::RESET(Reg::E, 4)),
            0xa4 => Some(Instruction::RESET(Reg::H, 4)),
            0xa5 => Some(Instruction::RESET(Reg::L, 4)),
            0xa6 => Some(Instruction::RESET(Reg::HL, 4)),
            0xa7 => Some(Instruction::RESET(Reg::A, 4)),

            0xa8 => Some(Instruction::RESET(Reg::B, 5)),
            0xa9 => Some(Instruction::RESET(Reg::C, 5)),
            0xaa => Some(Instruction::RESET(Reg::D, 5)),
            0xab => Some(Instruction::RESET(Reg::E, 5)),
            0xac => Some(Instruction::RESET(Reg::H, 5)),
            0xad => Some(Instruction::RESET(Reg::L, 5)),
            0xae => Some(Instruction::RESET(Reg::HL, 5)),
            0xaf => Some(Instruction::RESET(Reg::A, 5)),

            0xb0 => Some(Instruction::RESET(Reg::B, 6)),
            0xb1 => Some(Instruction::RESET(Reg::C, 6)),
            0xb2 => Some(Instruction::RESET(Reg::D, 6)),
            0xb3 => Some(Instruction::RESET(Reg::E, 6)),
            0xb4 => Some(Instruction::RESET(Reg::H, 6)),
            0xb5 => Some(Instruction::RESET(Reg::L, 6)),
            0xb6 => Some(Instruction::RESET(Reg::HL, 6)),
            0xb7 => Some(Instruction::RESET(Reg::A, 6)),

            0xb8 => Some(Instruction::RESET(Reg::B, 7)),
            0xb9 => Some(Instruction::RESET(Reg::C, 7)),
            0xba => Some(Instruction::RESET(Reg::D, 7)),
            0xbb => Some(Instruction::RESET(Reg::E, 7)),
            0xbc => Some(Instruction::RESET(Reg::H, 7)),
            0xbd => Some(Instruction::RESET(Reg::L, 7)),
            0xbe => Some(Instruction::RESET(Reg::HL, 7)),
            0xbf => Some(Instruction::RESET(Reg::A, 7)),

            0xc0 => Some(Instruction::SET(Reg::B, 0)),
            0xc1 => Some(Instruction::SET(Reg::C, 0)),
            0xc2 => Some(Instruction::SET(Reg::D, 0)),
            0xc3 => Some(Instruction::SET(Reg::E, 0)),
            0xc4 => Some(Instruction::SET(Reg::H, 0)),
            0xc5 => Some(Instruction::SET(Reg::L, 0)),
            0xc6 => Some(Instruction::SET(Reg::HL, 0)),
            0xc7 => Some(Instruction::SET(Reg::A, 0)),

            0xc8 => Some(Instruction::SET(Reg::B, 1)),
            0xc9 => Some(Instruction::SET(Reg::C, 1)),
            0xca => Some(Instruction::SET(Reg::D, 1)),
            0xcb => Some(Instruction::SET(Reg::E, 1)),
            0xcc => Some(Instruction::SET(Reg::H, 1)),
            0xcd => Some(Instruction::SET(Reg::L, 1)),
            0xce => Some(Instruction::SET(Reg::HL, 1)),
            0xcf => Some(Instruction::SET(Reg::A, 1)),

            0xd0 => Some(Instruction::SET(Reg::B, 2)),
            0xd1 => Some(Instruction::SET(Reg::C, 2)),
            0xd2 => Some(Instruction::SET(Reg::D, 2)),
            0xd3 => Some(Instruction::SET(Reg::E, 2)),
            0xd4 => Some(Instruction::SET(Reg::H, 2)),
            0xd5 => Some(Instruction::SET(Reg::L, 2)),
            0xd6 => Some(Instruction::SET(Reg::HL, 2)),
            0xd7 => Some(Instruction::SET(Reg::A, 2)),

            0xd8 => Some(Instruction::SET(Reg::B, 3)),
            0xd9 => Some(Instruction::SET(Reg::C, 3)),
            0xda => Some(Instruction::SET(Reg::D, 3)),
            0xdb => Some(Instruction::SET(Reg::E, 3)),
            0xdc => Some(Instruction::SET(Reg::H, 3)),
            0xdd => Some(Instruction::SET(Reg::L, 3)),
            0xde => Some(Instruction::SET(Reg::HL, 3)),
            0xdf => Some(Instruction::SET(Reg::A, 3)),

            0xe0 => Some(Instruction::SET(Reg::B, 4)),
            0xe1 => Some(Instruction::SET(Reg::C, 4)),
            0xe2 => Some(Instruction::SET(Reg::D, 4)),
            0xe3 => Some(Instruction::SET(Reg::E, 4)),
            0xe4 => Some(Instruction::SET(Reg::H, 4)),
            0xe5 => Some(Instruction::SET(Reg::L, 4)),
            0xe6 => Some(Instruction::SET(Reg::HL, 4)),
            0xe7 => Some(Instruction::SET(Reg::A, 4)),

            0xe8 => Some(Instruction::SET(Reg::B, 5)),
            0xe9 => Some(Instruction::SET(Reg::C, 5)),
            0xea => Some(Instruction::SET(Reg::D, 5)),
            0xeb => Some(Instruction::SET(Reg::E, 5)),
            0xec => Some(Instruction::SET(Reg::H, 5)),
            0xed => Some(Instruction::SET(Reg::L, 5)),
            0xee => Some(Instruction::SET(Reg::HL, 5)),
            0xef => Some(Instruction::SET(Reg::A, 5)),

            0xf0 => Some(Instruction::SET(Reg::B, 6)),
            0xf1 => Some(Instruction::SET(Reg::C, 6)),
            0xf2 => Some(Instruction::SET(Reg::D, 6)),
            0xf3 => Some(Instruction::SET(Reg::E, 6)),
            0xf4 => Some(Instruction::SET(Reg::H, 6)),
            0xf5 => Some(Instruction::SET(Reg::L, 6)),
            0xf6 => Some(Instruction::SET(Reg::HL, 6)),
            0xf7 => Some(Instruction::SET(Reg::A, 6)),

            0xf8 => Some(Instruction::SET(Reg::B, 7)),
            0xf9 => Some(Instruction::SET(Reg::C, 7)),
            0xfa => Some(Instruction::SET(Reg::D, 7)),
            0xfb => Some(Instruction::SET(Reg::E, 7)),
            0xfc => Some(Instruction::SET(Reg::H, 7)),
            0xfd => Some(Instruction::SET(Reg::L, 7)),
            0xfe => Some(Instruction::SET(Reg::HL, 7)),
            0xff => Some(Instruction::SET(Reg::A, 7)),
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),

            0x3c => Some(Instruction::INC(Reg::A)),
            0x04 => Some(Instruction::INC(Reg::B)),
            0x14 => Some(Instruction::INC(Reg::D)),
            0x24 => Some(Instruction::INC(Reg::H)),

            0x0c => Some(Instruction::INC(Reg::C)),
            0x1c => Some(Instruction::INC(Reg::E)),
            0x2c => Some(Instruction::INC(Reg::L)),

            0x34 => Some(Instruction::INC(Reg::HL)),
            0x03 => Some(Instruction::INC(Reg::BC)),
            0x13 => Some(Instruction::INC(Reg::DE)),
            0x23 => Some(Instruction::INC(Reg::HL)),
            0x33 => Some(Instruction::INC(Reg::SP)),


            0x3d => Some(Instruction::DEC(Reg::A)),
            0x05 => Some(Instruction::DEC(Reg::B)),
            0x0d => Some(Instruction::DEC(Reg::C)),
            0x15 => Some(Instruction::DEC(Reg::D)),

            0x1d => Some(Instruction::DEC(Reg::E)),
            0x25 => Some(Instruction::DEC(Reg::H)),
            0x2d => Some(Instruction::DEC(Reg::L)),

            0x35 => Some(Instruction::DEC(Reg::HL)),
            0x0b => Some(Instruction::DEC(Reg::BC)),
            0x1b => Some(Instruction::DEC(Reg::DE)),
            0x2b => Some(Instruction::DEC(Reg::HL)),
            0x3b => Some(Instruction::DEC(Reg::SP)),


            0x87 => Some(Instruction::ADD(Reg::A)),
            0x80 => Some(Instruction::ADD(Reg::B)),
            0x81 => Some(Instruction::ADD(Reg::C)),
            0x82 => Some(Instruction::ADD(Reg::D)),

            0x83 => Some(Instruction::ADD(Reg::E)),
            0x84 => Some(Instruction::ADD(Reg::H)),
            0x85 => Some(Instruction::ADD(Reg::L)),
            0x86 => Some(Instruction::ADD(Reg::HL)),
            0xC6 => Some(Instruction::ADD(Reg::D8)),


            0x09 => Some(Instruction::ADDHL(Reg16::BC)),
            0x19 => Some(Instruction::ADDHL(Reg16::DE)),
            0x29 => Some(Instruction::ADDHL(Reg16::HL)),
            0x39 => Some(Instruction::ADDHL(Reg16::SP)),


            0x8F => Some(Instruction::ADC(Reg::A)),
            0x88 => Some(Instruction::ADC(Reg::B)),
            0x89 => Some(Instruction::ADC(Reg::C)),
            0x8a => Some(Instruction::ADC(Reg::D)),

            0x8b => Some(Instruction::ADC(Reg::E)),
            0x8c => Some(Instruction::ADC(Reg::H)),
            0x8d => Some(Instruction::ADC(Reg::L)),
            0x8e => Some(Instruction::ADC(Reg::HL)),
            // 0xce => Some(Instruction::ADC(Reg::D8)),


            0x97 => Some(Instruction::SUB(Reg::A)),
            0x90 => Some(Instruction::SUB(Reg::B)),
            0x91 => Some(Instruction::SUB(Reg::C)),
            0x92 => Some(Instruction::SUB(Reg::D)),

            0x93 => Some(Instruction::SUB(Reg::E)),
            0x94 => Some(Instruction::SUB(Reg::H)),
            0x95 => Some(Instruction::SUB(Reg::L)),
            0x96 => Some(Instruction::SUB(Reg::HL)),
            // 0xd6 => Some(Instruction::SUB(Reg::D8)),


            0x9f => Some(Instruction::SBC(Reg::A)),
            0x98 => Some(Instruction::SBC(Reg::B)),
            0x99 => Some(Instruction::SBC(Reg::C)),
            0x9a => Some(Instruction::SBC(Reg::D)),

            0x9b => Some(Instruction::SBC(Reg::E)),
            0x9c => Some(Instruction::SBC(Reg::H)),
            0x9d => Some(Instruction::SBC(Reg::L)),
            0x9e => Some(Instruction::SBC(Reg::HL)),
            // 0xde => Some(Instruction::SBC(Reg::D8)),


            0xa7 => Some(Instruction::AND(Reg::A)),
            0xa0 => Some(Instruction::AND(Reg::B)),
            0xa1 => Some(Instruction::AND(Reg::C)),
            0xa2 => Some(Instruction::AND(Reg::D)),

            0xa3 => Some(Instruction::AND(Reg::E)),
            0xa4 => Some(Instruction::AND(Reg::H)),
            0xa5 => Some(Instruction::AND(Reg::L)),
            0xa6 => Some(Instruction::AND(Reg::HL)),
            // 0xe6 => Some(Instruction::AND(Reg::D8)),


            0xb7 => Some(Instruction::OR(Reg::A)),
            0xb0 => Some(Instruction::OR(Reg::B)),
            0xb1 => Some(Instruction::OR(Reg::C)),
            0xb2 => Some(Instruction::OR(Reg::D)),

            0xb3 => Some(Instruction::OR(Reg::E)),
            0xb4 => Some(Instruction::OR(Reg::H)),
            0xb5 => Some(Instruction::OR(Reg::L)),
            0xb6 => Some(Instruction::OR(Reg::HL)),
            // 0xf6 => Some(Instruction::OR(Reg::D8)),


            0xaf => Some(Instruction::XOR(Reg::A)),
            0xa8 => Some(Instruction::XOR(Reg::B)),
            0xa9 => Some(Instruction::XOR(Reg::C)),
            0xaa => Some(Instruction::XOR(Reg::D)),

            0xab => Some(Instruction::XOR(Reg::E)),
            0xac => Some(Instruction::XOR(Reg::H)),
            0xad => Some(Instruction::XOR(Reg::L)),
            0xae => Some(Instruction::XOR(Reg::HL)),
            // 0xee => Some(Instruction::XOR(Reg::D8)),


            0xbf => Some(Instruction::CP(Reg::A)),
            0xb8 => Some(Instruction::CP(Reg::B)),
            0xb9 => Some(Instruction::CP(Reg::C)),
            0xba => Some(Instruction::CP(Reg::D)),

            0xbb => Some(Instruction::CP(Reg::E)),
            0xbc => Some(Instruction::CP(Reg::H)),
            0xbd => Some(Instruction::CP(Reg::L)),
            0xbe => Some(Instruction::CP(Reg::HL)),
            // 0xfe => Some(Instruction::CP(Reg::D8)),


            // 0xe8 => Some(Instruction::ADDSP),

            0x3f => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x1f => Some(Instruction::RRA),
            0x17 => Some(Instruction::RLA),
            0x0f => Some(Instruction::RRCA),
            0x07 => Some(Instruction::RLCA),
            0x2f => Some(Instruction::CPL),


            // Jumps
            0xC3 => Some(Instruction::JP(JumpTypes::Always)),
            0xCA => Some(Instruction::JP(JumpTypes::Zero)),
            0xC2 => Some(Instruction::JP(JumpTypes::NotZero)),
            0xDA => Some(Instruction::JP(JumpTypes::Carry)),
            0xD2 => Some(Instruction::JP(JumpTypes::NotCarry)),


            // Relative Jump
            0x18 => Some(Instruction::JR(JumpTypes::Always)),
            0x28 => Some(Instruction::JR(JumpTypes::Zero)),
            0x20 => Some(Instruction::JR(JumpTypes::NotZero)),
            0x38 => Some(Instruction::JR(JumpTypes::Carry)),
            0x30 => Some(Instruction::JR(JumpTypes::NotCarry)),
            


            0xCD => Some(Instruction::CALL(JumpTypes::Always)),
            0xCC => Some(Instruction::CALL(JumpTypes::Zero)),
            0xC4 => Some(Instruction::CALL(JumpTypes::NotZero)),
            0xDC => Some(Instruction::CALL(JumpTypes::Carry)),
            0xD4 => Some(Instruction::CALL(JumpTypes::NotCarry)),

            
            0xC9 => Some(Instruction::RET(JumpTypes::Always)),
            0xC8 => Some(Instruction::RET(JumpTypes::Zero)),
            0xC0 => Some(Instruction::RET(JumpTypes::NotZero)),
            0xD8 => Some(Instruction::RET(JumpTypes::Carry)),
            0xD0 => Some(Instruction::RET(JumpTypes::NotCarry)),



            // LOADS
            0x06 => Some(Instruction::LD(Reg::B, Reg::D8)),
            0x16 => Some(Instruction::LD(Reg::D, Reg::D8)),
            0x26 => Some(Instruction::LD(Reg::H, Reg::D8)),
            0x36 => Some(Instruction::LD(Reg::HL, Reg::D8)),

            0x0E => Some(Instruction::LD(Reg::C, Reg::D8)),
            0x1E => Some(Instruction::LD(Reg::E, Reg::D8)),
            0x2E => Some(Instruction::LD(Reg::L, Reg::D8)),
            0x3E => Some(Instruction::LD(Reg::A, Reg::D8)),

            0x40 => Some(Instruction::LD(Reg::B, Reg::B)),
            0x41 => Some(Instruction::LD(Reg::B, Reg::C)),
            0x42 => Some(Instruction::LD(Reg::B, Reg::D)),
            0x43 => Some(Instruction::LD(Reg::B, Reg::E)),

            0x44 => Some(Instruction::LD(Reg::B, Reg::H)),
            0x45 => Some(Instruction::LD(Reg::B, Reg::L)),
            0x46 => Some(Instruction::LD(Reg::B, Reg::HL)),
            0x47 => Some(Instruction::LD(Reg::B, Reg::A)),

            0x48 => Some(Instruction::LD(Reg::C, Reg::B)),
            0x49 => Some(Instruction::LD(Reg::C, Reg::C)),
            0x4A => Some(Instruction::LD(Reg::C, Reg::D)),
            0x4B => Some(Instruction::LD(Reg::C, Reg::E)),

            0x4C => Some(Instruction::LD(Reg::C, Reg::H)),
            0x4D => Some(Instruction::LD(Reg::C, Reg::L)),
            0x4E => Some(Instruction::LD(Reg::C, Reg::HL)),
            0x4F => Some(Instruction::LD(Reg::C, Reg::A)),

            0x50 => Some(Instruction::LD(Reg::D, Reg::B)),
            0x51 => Some(Instruction::LD(Reg::D, Reg::C)),
            0x52 => Some(Instruction::LD(Reg::D, Reg::D)),
            0x53 => Some(Instruction::LD(Reg::D, Reg::E)),

            0x54 => Some(Instruction::LD(Reg::D, Reg::H)),
            0x55 => Some(Instruction::LD(Reg::D, Reg::H)),
            0x56 => Some(Instruction::LD(Reg::D, Reg::H)),
            0x57 => Some(Instruction::LD(Reg::D, Reg::A)),

            0x58 => Some(Instruction::LD(Reg::E, Reg::B)),
            0x59 => Some(Instruction::LD(Reg::E, Reg::C)),
            0x5A => Some(Instruction::LD(Reg::E, Reg::D)),
            0x5B => Some(Instruction::LD(Reg::E, Reg::E)),

            0x5C => Some(Instruction::LD(Reg::E, Reg::H)),
            0x5D => Some(Instruction::LD(Reg::E, Reg::L)),
            0x5E => Some(Instruction::LD(Reg::E, Reg::HL)),
            0x5F => Some(Instruction::LD(Reg::E, Reg::A)),

            0x60 => Some(Instruction::LD(Reg::H, Reg::B)),
            0x61 => Some(Instruction::LD(Reg::H, Reg::C)),
            0x62 => Some(Instruction::LD(Reg::H, Reg::D)),
            0x63 => Some(Instruction::LD(Reg::H, Reg::E)),

            0x64 => Some(Instruction::LD(Reg::H, Reg::H)),
            0x65 => Some(Instruction::LD(Reg::H, Reg::L)),
            0x66 => Some(Instruction::LD(Reg::H, Reg::HL)),
            0x67 => Some(Instruction::LD(Reg::H, Reg::A)),

            0x68 => Some(Instruction::LD(Reg::L, Reg::B)),
            0x69 => Some(Instruction::LD(Reg::L, Reg::C)),
            0x6A => Some(Instruction::LD(Reg::L, Reg::D)),
            0x6B => Some(Instruction::LD(Reg::L, Reg::E)),

            0x6C => Some(Instruction::LD(Reg::L, Reg::H)),
            0x6D => Some(Instruction::LD(Reg::L, Reg::L)),
            0x6E => Some(Instruction::LD(Reg::L, Reg::HL)),
            0x6F => Some(Instruction::LD(Reg::L, Reg::A)),

            0x70 => Some(Instruction::LD(Reg::HL, Reg::B)),
            0x71 => Some(Instruction::LD(Reg::HL, Reg::C)),
            0x72 => Some(Instruction::LD(Reg::HL, Reg::D)),
            0x73 => Some(Instruction::LD(Reg::HL, Reg::E)),

            0x74 => Some(Instruction::LD(Reg::HL, Reg::H)),
            0x75 => Some(Instruction::LD(Reg::HL, Reg::L)),
            // 0x76: halt
            0x77 => Some(Instruction::LD(Reg::HL, Reg::A)),

            0x78 => Some(Instruction::LD(Reg::A, Reg::B)),
            0x79 => Some(Instruction::LD(Reg::A, Reg::C)),
            0x7A => Some(Instruction::LD(Reg::A, Reg::D)),
            0x7B => Some(Instruction::LD(Reg::A, Reg::E)),

            0x7C => Some(Instruction::LD(Reg::A, Reg::H)),
            0x7D => Some(Instruction::LD(Reg::A, Reg::L)),
            0x7E => Some(Instruction::LD(Reg::A, Reg::HL)),
            0x7F => Some(Instruction::LD(Reg::A, Reg::A)),

            0x01 => Some(Instruction::LD(Reg::BC, Reg::D16)),
            0x11 => Some(Instruction::LD(Reg::DE, Reg::D16)),
            0x21 => Some(Instruction::LD(Reg::HL, Reg::D16)),
            // 0x31 => Some(Instruction::LD(Reg::SP, Reg::D16)),

            0x02 => Some(Instruction::LD(Reg::Addr(Reg16::BC), Reg::A)),
            0x12 => Some(Instruction::LD(Reg::Addr(Reg16::DE), Reg::A)),
            0x22 => Some(Instruction::LD(Reg::Addr(Reg16::HLI), Reg::A)),
            0x32 => Some(Instruction::LD(Reg::Addr(Reg16::HLD), Reg::A)),

            0x0A => Some(Instruction::LD(Reg::A, Reg::Addr(Reg16::BC))),
            0x1A => Some(Instruction::LD(Reg::A, Reg::Addr(Reg16::DE))),
            0x2A => Some(Instruction::LD(Reg::A, Reg::Addr(Reg16::HLI))),
            0x3A => Some(Instruction::LD(Reg::A, Reg::Addr(Reg16::HLD))),


            0xC5 => Some(Instruction::PUSH(Reg16::BC)),
            0xD5 => Some(Instruction::PUSH(Reg16::DE)),
            0xE5 => Some(Instruction::PUSH(Reg16::HL)),
            0xF5 => Some(Instruction::PUSH(Reg16::AF)),


            0xC1 => Some(Instruction::POP(Reg16::BC)),
            0xD1 => Some(Instruction::POP(Reg16::DE)),
            0xE1 => Some(Instruction::POP(Reg16::HL)),
            0xF1 => Some(Instruction::POP(Reg16::AF)),


            0x76 => Some(Instruction::HALT),


            0x10 => Some(Instruction::IDK),  // TODO: STOP INSTRUCTION


            // 0x27 => Some(Instruction::DAA),


            // UNREAL INSTRUCIONS!
            // DELULU INSTRUCTIONS!

            0xF4 => Some(Instruction::IDK), // doesn't exist in the gameboy. debug purposes (custom)
            0xD3 => Some(Instruction::EXIT),    // not a real instruction(custom)

            _ => None,
        }
    }
}