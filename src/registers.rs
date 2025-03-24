use crate::utils::{log, panic_log};

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Reg {
    A, B, C, D, E, FLAGS, H, L, 
    AF, BC, HL, DE, D8, D16, HLI, HLD,
    SP,
    // only r16 can store addresses
    Addr(Reg16),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Reg16 {
    AF, BC, DE, HL, HLI, HLD, SP
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub flags: FlagsRegister,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
}

pub struct FlagsRegister {
    pub zero: bool,         // bit 7
    pub subtract: bool,     // bit 6
    pub half_carry: bool,   // bit 5
    pub carry: bool,        // bit 4
}



impl From<Reg16> for Reg {
    fn from(reg: Reg16) -> Self {
        match reg {
            Reg16::HL => Reg::HL,
            Reg16::BC => Reg::BC,
            Reg16::DE => Reg::DE,
            Reg16::AF => Reg::AF,
            _ => unreachable!("the others are unique, man!"),
        }
    }
}

impl From<Reg> for Reg16 {
    fn from(reg: Reg) -> Reg16 {
        match reg {
            Reg::HL => Reg16::HL,
            Reg::BC => Reg16::BC,
            Reg::DE => Reg16::DE,
            Reg::AF => Reg16::AF,
            Reg::HLD => Reg16::HLD,
            Reg::HLI => Reg16::HLI,
            Reg::SP => Reg16::SP,

            _ => {
                unreachable!("Normally, you wouldn't try to convert a 8bit register into a 16bit one");
            }
        }
    }
}



impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            flags: FlagsRegister::new(),
            h: 0,
            l: 0,
            sp: 0xFFFE
        }
    }

    pub fn get_reg_value(&self, reg: Reg) -> (u8, Reg) {
        (
            match reg {
                Reg::A => self.a,
                Reg::B => self.b,
                Reg::C => self.c,
                Reg::D => self.d,
                Reg::E => self.e,
                Reg::H => self.h,
                Reg::L => self.l,
                Reg::FLAGS => self.flags.into_u8(),
                Reg::HL | Reg::DE | Reg::AF | 
                Reg::BC | Reg::SP => panic_log(&format!("Hey smarty, {reg:?} is 16bit.")),
                Reg::D8 => unreachable!("'D8' means '8 direct bits'. It's not a register"),
                Reg::D16 => unreachable!("'D16' means '16 direct bits'. It's not a register"),
                Reg::HLI => unreachable!("'HLI' is 'HL' but incremented. It's not EXACTLY a register"),
                Reg::HLD => unreachable!("'HLD' is 'HL' but decremented. It's not EXACTLY a register"),
                Reg::Addr(_) => todo!("Can't get address as reg, yet..."),
            },
            reg
        )
    }

    pub fn get_reg(&mut self, reg: Reg) -> (&mut u8, Reg) {

        (
            match reg {
                Reg::A => &mut self.a,
                Reg::B => &mut self.b,
                Reg::C => &mut self.c,
                Reg::D => &mut self.d,
                Reg::E => &mut self.e,
                Reg::H => &mut self.h,
                // TODO: should HL be here?
                Reg::L | Reg::HL => &mut self.l,

                // normally, u can't get the reference of non-existing variable.
                // the following registers are calculated on runtime (ex. reg A + reg B)
                Reg::FLAGS => unreachable!("Cannot get REG!"),
                Reg::DE => unreachable!("Cannot get REG!"),
                Reg::AF => unreachable!("Cannot get REG!"),
                Reg::BC => unreachable!("Cannot get REG!"),
                Reg::SP => panic_log("SP is 16bit, sucker!"),
                Reg::D8 => unreachable!("'D8' means '8 direct bits'. It's not a register"),
                Reg::D16 => unreachable!("'D16' means '16 direct bits'. It's not a register"),
                Reg::HLI => unreachable!("'HLI' is 'HL' but incremented. It's not EXACTLY a register"),
                Reg::HLD => unreachable!("'HLD' is 'HL' but decremented. It's not EXACTLY a register"),
                Reg::Addr(_) => todo!("Can't get address as reg, yet..."),
            },
            reg
        )
    }

    pub fn get_vreg_value(&self, vreg: Reg16) -> (u16, Reg16) {
        // VRegs list: af, bc, de, hl
        (match vreg {
            Reg16::AF => {
                ((self.a as u16) << 8) | (self.flags.into_u8() as u16)
            },

            Reg16::BC => {
                ((self.b as u16) << 8) | (self.c as u16)
            },

            Reg16::DE => {
                ((self.d as u16) << 8) | (self.e as u16)
            },

            Reg16::HL => {
                ((self.h as u16) << 8) | (self.l as u16)
            },

            Reg16::HLD | Reg16::HLI => {
                let result = ((self.h as u16) << 8) | (self.l as u16);

                if vreg == Reg16::HLD {
                    result.wrapping_sub(1)
                } else {
                    result.wrapping_add(1)
                }
            },

            Reg16::SP => self.sp,

            #[allow(unreachable_patterns)]
            _ => {
                panic_log("[vreg_value] Invalid registers pair! (must be AF, BC, DE, HL)");
            },
        }, vreg)
    }

    #[allow(unused)]
    pub fn set_vreg_liberal(&mut self, reg1: Reg, reg2: Reg, value: u16) -> () {
        // VRegs list: af, bc, de, hl
        let left_bits: u8 = ((value & 0xFF00) >> 8) as u8;
        let right_bits: u8 = (value & 0x00FF) as u8;

        match (reg1, reg2) {
            (Reg::A, Reg::FLAGS) => {
                self.a = left_bits;
                self.flags = FlagsRegister::from_u8(right_bits);
            },

            (Reg::B, Reg::C) => {
                self.b = left_bits;
                self.c = right_bits;
            },

            (Reg::D, Reg::E) => {
                self.d = left_bits;
                self.e = right_bits;
            },

            (Reg::H, Reg::L) => {
                self.h = left_bits;
                self.l = right_bits;
            },

            _ => panic_log("[liberal_vreg] Invalid registers pair! (must be AF, BC, DE, HL)"),
        }
    }

    pub fn set_vreg(&mut self, reg: Reg16, value: u16) -> () {
        let msb: u8 = ((value & 0xFF00) >> 8) as u8;
        let lsb: u8 = (value & 0x00FF) as u8;

        match reg {
            Reg16::AF => {
                self.a = msb;
                self.flags = FlagsRegister::from_u8(lsb);
            },

            Reg16::BC => {
                self.b = msb;
                self.c = lsb;
            },

            Reg16::DE => {
                self.d = msb;
                self.e = lsb;
            },

            Reg16::HL => {
                self.h = msb;
                self.l = lsb;
            },

            #[allow(unreachable_patterns)]
            _ => {
                log("Can't set non-vreg");
            }
        }
    }

    pub fn reg8_can_be_reg16(&self, reg: Reg) -> bool {
        matches!(reg, 
            Reg::AF | Reg::BC | Reg::DE | Reg::HL | 
            Reg::HLI | Reg::HLD | Reg::SP
        )
    }
}

impl FlagsRegister {
    pub fn new() -> Self {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn into_u8(&self) -> u8 {
        const ZERO_FLAG_BYTE_POSITION: u8 = 7;
        const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
        const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
        const CARRY_FLAG_BYTE_POSITION: u8 = 4;

        (if self.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if self.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if self.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if self.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }

    pub fn from_u8(byte: u8) -> Self {
        const ZERO_FLAG_BYTE_POSITION: u8 = 7;
        const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
        const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
        const CARRY_FLAG_BYTE_POSITION: u8 = 4;

        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero, 
            subtract, 
            half_carry, 
            carry
        }
    }
}