use std::process::exit;
use crate::{instructions::{Instruction, JumpTypes}, memory::MemoryBus, registers::*, utils::*};

pub struct CPU {
    pub regs: Registers,
    pub pc: u16,
    pub mem_bus: MemoryBus,
    pub is_halted: bool,
    pub rom_size: usize,
}


// IMPLEMENTATIONS

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: Registers::new(),
            pc: 0, // TODO: might not be correct as an initial value
            mem_bus: MemoryBus::new(),
            is_halted: false,
            rom_size: 0,
        }
    }

    pub fn step(&mut self) {
        // TODO: I don't think this is correct, nor necessary. HOW WILL WE RE-ENABLE INTERRUPTS????
        if self.is_halted {
            return;
        }

        let mut instruction_byte = self.mem_bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            self.pc += 1;
            instruction_byte = self.mem_bus.read_byte(self.pc);
        }

        if DEBUG_ENABLED {
            let log: String = format!(
                "[0x{:04X}]{}{:?}:0x{instruction_byte:02X}", 
                self.pc, if prefixed { " (0xCB) " } else { " " },
                if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {instruction} else {Instruction::IDK}
            );
            println!("{}", log);
            debug_logs(&log);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let byte_name = format!("0x{}{:X}", if prefixed { "cb" } else { "" }, instruction_byte);
            let error_message = format!("'{}' not recognized as an instruction at 0x{:04X}", byte_name, self.pc);
            panic_log(&error_message);
        };

        if DEBUG_ENABLED {
            let log: String = format!(
                "A:{:08b}|B:{:08b}|C:{:08b}|D:{:08b}|E:{:08b}|F:{:08b}|H:{:08b}|L:{:08b}\n\
                SP:{:08b}|BC:{:016b}|DE:{:016b}|HL:{:016b}|AF:{:016b}\n",
                self.regs.a, self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.flags.into_u8(), self.regs.h, self.regs.l,
                self.regs.sp, self.regs.get_vreg_value(Reg16::BC).0, self.regs.get_vreg_value(Reg16::DE).0, 
                self.regs.get_vreg_value(Reg16::HL).0, self.regs.get_vreg_value(Reg16::AF).0
            );
            
            println!("{log}");
            debug_logs(&log);
        }

        self.pc = next_pc;

        if self.pc as usize >= (self.rom_size * 1024) {
            log("SOP");
            self.is_halted = true;
        }

        // delay(10);
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc; // ig that's correct? cuz we not advancing!
        }

        match instruction.clone() {
            // JUMPS!!!11!!!1!1!

            Instruction::JP(jump_type) | Instruction::CALL(jump_type) | 
            Instruction::RET(jump_type) | Instruction::JR(jump_type ) => {
                let jump_condition = self.get_jump_condition(jump_type).0;

                return match instruction.clone() {
                    Instruction::JP(_) => self.jump(jump_condition),
                    Instruction::JR(_) => self.relative_jump(jump_condition),
                    Instruction::CALL(_) => self.call(jump_condition),
                    Instruction::RET(_) => self.ret(jump_condition),

                    _ => unreachable!("WHAT are you?"),
                }
            },

            Instruction::LD(dst, src) => {
                return self.ld(src, dst);
            },

            Instruction::PUSH(target16) => {
                let value: u16 = self.regs.get_vreg_value(target16).0;
                self.push(value);
                return self.pc.wrapping_add(1);
            },

            Instruction::POP(target16) => {
                let value: u16 = self.pop();
                self.regs.set_vreg(target16.into(), value);
                return self.pc.wrapping_add(1);
            },

            // Add 'target' to regA
            Instruction::ADD(target) => {
                let value_to_add: u8 = if target == Reg::D8 {
                    self.read_next_byte()
                } else {
                    self.regs.get_reg_value(target).0
                };


                // when adding 16bit reg to 8bit reg, we get
                // the lower 8bits of the 16bit reg
                self.regs.a = self.gb_add(self.regs.a, value_to_add);
            },

            Instruction::ADC(target) => {
                // TODO: Should the new value REALLY be 8bit? We're storing it in Reg::A which is 8bit, but idk...
                let new_value: u8 = if target == Reg::HL {
                    self.gb_add16(self.regs.a as u16, self.regs.get_vreg_value(Reg16::HL).0) as u8
                } else {
                    self.gb_add(self.regs.a, self.regs.get_reg_value(target).0)
                };

                let carry = self.regs.flags.carry as u8;
                let (final_value, final_overflow) = new_value.overflowing_add(carry);

                self.regs.a = final_value;
                self.regs.flags.carry = final_overflow;
            },

            // Add 'target' to regHL
            Instruction::ADDHL(target) => {
                let add_num = self.regs.get_vreg_value(target.into()).0;
                let hl = self.regs.get_vreg_value(Reg16::HL).0;
                let sum = self.gb_add16(hl, add_num);

                self.regs.set_vreg(Reg16::HL, sum);
            },

            // Subtract 'target' from regA
            Instruction::SUB(target) => {
                let subtraction = self.gb_sub(self.regs.a, self.regs.get_reg_value(target).0);
                self.regs.a = subtraction;
            },

            Instruction::SBC(target) => {
                let new_value = self.gb_sub(self.regs.a, self.regs.get_reg_value(target).0);
                let carry = self.regs.flags.carry as u8;
                let (final_value, final_overflow) = new_value.overflowing_sub(carry);

                self.regs.a = final_value;
                self.regs.flags.carry = final_overflow;
            },

            Instruction::AND(target) => {
                let reg = self.regs.get_reg_value(target).0;
                self.regs.a &= reg;

                self.regs.flags.zero = self.regs.a == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = true;
                self.regs.flags.carry = false;

            },

            Instruction::OR(target) => {
                let reg = self.regs.get_reg_value(target).0;
                self.regs.a |= reg;

                self.regs.flags.zero = self.regs.a == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = false;
            },

            Instruction::CP(target) => {
                let reg_value = self.regs.get_reg_value(target).0;
                let diff = self.gb_sub(self.regs.a, reg_value);

                self.regs.flags.zero = diff == 0;
                self.regs.flags.subtract = true;
                self.regs.flags.half_carry = (self.regs.a & 0xF) < (reg_value & 0xF); // TODO: understand
                self.regs.flags.carry = self.regs.a < reg_value; // TODO: understand
            },

            Instruction::XOR(target) => {
                let reg = self.regs.get_reg_value(target).0;
                self.regs.a ^= reg;

                self.regs.flags.zero = self.regs.a == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = false;
            }

            Instruction::INC(target) => {
                // if target is 16bit
                self.inc(target);
            },

            Instruction::DEC(target) => {
                self.dev(target);
            },

            Instruction::CCF => {
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = !self.regs.flags.carry;
            },

            Instruction::SCF => {
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = true;
            },

            Instruction::RRA => {
                let new_carry = (self.regs.a & 0x1) != 0;
                let new_a = ((self.regs.flags.carry as u8) << 7) | (self.regs.a >> 1);
                self.regs.a = new_a;

                self.regs.flags.zero = false;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RLA => {
                let new_carry = (self.regs.a & 0b10000000) != 0;
                let new_a = (self.regs.a << 1) | ((self.regs.flags.carry as u8) & 0x1);

                self.regs.a = new_a;

                self.regs.flags.zero = false;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RRCA => {
                let new_carry = (self.regs.a & 0x1) != 0;
                let new_a = ((new_carry as u8) << 7) | (self.regs.a >> 1);

                self.regs.a = new_a;

                self.regs.flags.zero = false;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RLCA => {
                let new_carry = (self.regs.a & 0b10000000) != 0;
                let new_a = (self.regs.a << 1) | ((new_carry as u8) & 0x1);

                self.regs.a = new_a;

                self.regs.flags.zero = false;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },
            
            Instruction::CPL => {
                let new_a = self.regs.a ^ 0xFF;

                self.regs.a = new_a;

                self.regs.flags.subtract = true;
                self.regs.flags.half_carry = true;
            },

            Instruction::BIT(target, bit_pos) => {
                if bit_pos > 7 {
                    panic_log("Invalid bit position! (b > 7)");
                }

                let reg_value = self.regs.get_reg_value(target).0;
                let result = (reg_value & (1 << bit_pos)) == 0;

                self.regs.flags.zero = result;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = true;
            },

            Instruction::RESET(target, bit_pos) => {
                if bit_pos > 7 {
                    panic_log("Invalid bit position! (b > 7)");
                }

                // TODO: implement for 16bit regs

                let reg = self.regs.get_reg(target).0;
                *reg &= !(1 << bit_pos);
            },

            Instruction::SET(target, bit_pos) => {
                self.set(target, bit_pos);
            },

            Instruction::SRL(target) => {
                let reg = self.regs.get_reg(target).0;
                *reg = *reg >> 1;
            },

            Instruction::RR(target) => {
                let old_carry = self.regs.flags.carry as u8;
                let reg = self.regs.get_reg(target).0;
                let new_carry = (*reg & 0x1) != 0;
                let new_reg = (old_carry << 7) | (*reg >> 1);
                
                *reg = new_reg;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RL(target) => {
                let old_carry = self.regs.flags.carry as u8;
                let reg = self.regs.get_reg(target).0;
                let new_carry = (*reg & 0b10000000) != 0;
                let new_reg = (*reg << 1) | (old_carry & 0x1);

                *reg = new_reg;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RRC(target) => {
                let reg = self.regs.get_reg(target).0;
                let new_carry = (*reg & 0x1) != 0;
                let new_reg = ((new_carry as u8) << 7) | (*reg >> 1);

                *reg = new_reg;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::RLC(target) => {
                let reg = self.regs.get_reg(target).0;
                let new_carry = (*reg & 0b10000000) != 0;
                let new_reg = (*reg << 1) | ((new_carry as u8) & 0x1);

                *reg = new_reg;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::SRA(target) => {
                let reg = self.regs.get_reg(target).0;
                let old_msb = *reg & (1 << 7); // sign bit
                let new_value = *reg >> 1;
                let new_carry = (*reg & 0x1) != 0;

                *reg = old_msb | new_value;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::SLA(target) => {
                let reg = self.regs.get_reg(target).0;
                let new_value = *reg << 1;
                let new_carry = (*reg & (1 << 7)) != 0;

                *reg = new_value;

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = new_carry;
            },

            Instruction::SWAP(target) => {
                // swap nibbles
                let reg = self.regs.get_reg(target).0;
                let upper = *reg & 0b11110000;
                let lower = *reg & 0b00001111;

                *reg = (upper >> 4) | (lower << 4);

                self.regs.flags.zero = *reg == 0;
                self.regs.flags.subtract = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = false;
            },

            Instruction::HALT => {
                log("Halting");
                self.is_halted = true;
                return self.pc;
            },

            Instruction::NOP => {
                // we return early just cuz. even tho when we do nothing, we return pc+1 at the end
                return self.pc.wrapping_add(1);
            },


            // delulu instructions

            Instruction::IDK => {
                log("(bro doesn't know)");
            },

            Instruction::EXIT => {
                exit(0);
            }

            #[allow(unreachable_patterns)]
            _ => {
                unreachable!("Unknown Instruction");
            }
        }

        self.pc.wrapping_add(1)
    }



    // instructions

    pub fn ld(&mut self, src: Reg, dst: Reg) -> u16 {
        let source_value: u16 = match src {
        Reg::D8 => self.read_next_byte().into(),
        Reg::D16 => self.read_next_word(),
        Reg::HLI | Reg::HLD => self.mem_bus.read_byte(self.regs.get_vreg_value(Reg16::HL).0).into(),
        Reg::Addr(reg16) => {
            let vreg = self.regs.get_vreg_value(reg16);
            let byte = self.mem_bus.read_byte(vreg.0);

            if matches!(reg16, Reg16::HLD | Reg16::HLI) {
                let hl_value = self.regs.get_vreg_value(Reg16::HL).0;
                self.regs.set_vreg(Reg16::HL,
                    if reg16 == Reg16::HLI {
                        hl_value.wrapping_add(1)
                    } else {
                        hl_value.wrapping_sub(1)
                    }
                );
            }

            if DEBUG_ENABLED {
                log(&format!("Loading 0x{byte:04X} from {:04X} into {dst:?}", vreg.0));
            }

            byte.into()
        },
        _ => { 
            if self.regs.reg8_can_be_reg16(src) {
                self.regs.get_vreg_value(src.into()).0
            } else {
                self.regs.get_reg_value(src).0.into()
            }
        }
        };

        if DEBUG_ENABLED {
            log(&format!("Source Value: 0x{source_value:04X}"));
        }

        if matches!(dst, Reg::HLI | Reg::HLD) {
            let hl_address = self.regs.get_vreg_value(Reg16::HL).0;
            self.mem_bus.write_byte(hl_address, source_value as u8);
                
            // decrement/increment HL
            self.regs.set_vreg(Reg16::HL, if dst == Reg::HLI {
                hl_address.wrapping_add(1)  // HL++
            } else {
                hl_address.wrapping_sub(1)  // HL--
            });
        } else {
            if matches!(dst, Reg::BC | Reg::DE | Reg::HL /*| Reg::SP*/) {
                self.regs.set_vreg(dst.into(), source_value);
            } else {
                match dst {
                    Reg::Addr(addr_reg) => {
                        let address: u16 = if matches!(addr_reg, Reg16::HLD | Reg16::HLI) {
                            let hl_value = self.regs.get_vreg_value(Reg16::HL).0;
                            self.regs.set_vreg(Reg16::HL, 
                                    if addr_reg == Reg16::HLI { hl_value.wrapping_add(1) } else { hl_value.wrapping_sub(1) }
                            );
                            hl_value
                        } else {
                            self.regs.get_vreg_value(addr_reg).0
                        };

                        if DEBUG_ENABLED {
                            let address_value = self.mem_bus.read_byte(address);
                            log(&format!("Writing 0x{:04X} to 0x{address:04X}", address_value));
                        }

                        self.mem_bus.write_byte(address, source_value as u8);
                    },
                    _ => {
                        let reg = self.regs.get_reg(dst);
                        let the_reg: &mut u8 = reg.0;

                        *the_reg = source_value as u8;
                    }
                }
            }
        }

        return match src {
            Reg::D8 => self.pc.wrapping_add(2),
            Reg::D16 => self.pc.wrapping_add(3),
            _ => self.pc.wrapping_add(1)
        };
    }

    pub fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let greater_byte = self.mem_bus.read_byte(self.pc + 2) as u16;
            let smaller_byte = self.mem_bus.read_byte(self.pc + 1) as u16;

            // TODO: undestand little-endian

            let addr: u16 = (greater_byte << 8) | smaller_byte;
            if DEBUG_ENABLED {
                log(&format!("Jumped to: 0x{:04x}", addr));
            }
            addr
        } else {
            // skip the opcode byte & 2 address bytes
            self.pc.wrapping_add(3)
        }
    }

    pub fn relative_jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let relative = self.read_next_byte() as i8;

            let new_pc = self.pc.wrapping_add(relative as i16 as u16);

            log(&format!("Jump to: 0x{new_pc:04X}"));

            new_pc
        } else {
            self.pc.wrapping_add(2)
        }
    }

    pub fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);

        if DEBUG_ENABLED {
            log(&format!("Calling function at 0x{:04X}", next_pc));
        }

        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    pub fn ret(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let return_addr = self.pop();
            if DEBUG_ENABLED { log(&format!("Returning to address 0x{:04X}", return_addr)); }
            return_addr
        } else {
            self.pc.wrapping_add(1)
        }
    }

    pub fn inc(&mut self, target: Reg) -> () {
        if self.regs.reg8_can_be_reg16(target) {
            let incremented_value = self.gb_add16(self.regs.get_vreg_value(target.into()).0, 1);
            self.regs.set_vreg(target.into(), incremented_value);
        } else {
            let (reg_value, target) = self.regs.get_reg_value(target);
            let new_reg_value = self.gb_add(reg_value, 1);
            *self.regs.get_reg(target).0 = new_reg_value;
        }
    }

    pub fn dev(&mut self, target: Reg) -> () {
        if self.regs.reg8_can_be_reg16(target) {
            let decremented_value = self.gb_sub16(self.regs.get_vreg_value(target.into()).0, 1);
            self.regs.set_vreg(target.into(), decremented_value);
        } else {
            let (reg_value, target) = self.regs.get_reg_value(target);
            let new_reg_value = self.gb_sub(reg_value, 1);
            *self.regs.get_reg(target).0 = new_reg_value;
        }
    }

    pub fn set(&mut self, target: Reg, bit_position: u8) -> () {
        
        if bit_position > 7 {
            panic_log("Invalid bit position! (b > 7)");
        }

        if target != Reg::HL && matches!(target, Reg::A | Reg::B | Reg::C | Reg::D | Reg::E | Reg::H | Reg::L | Reg::HL) {
            let reg_dat = self.regs.get_reg(target);
            *reg_dat.0 |= 1 << bit_position;
        } else if target == Reg::HL {
            let mut hl_byte: u16 = self.regs.get_vreg_value(Reg16::HL).0;
            hl_byte |= 1 << bit_position;
            self.regs.set_vreg(Reg16::HL, hl_byte);
        } else {
            panic_log("Tried setting registers' bit other than A, B, C, D, E, H, L, HL!");
        }
    }




    pub fn gb_add16(&mut self, reg_target: u16, value: u16) -> u16 {
        let (result, overflow) = reg_target.overflowing_add(value);
        
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = (reg_target & 0xFFF) + (value & 0xFFF) > 0xFFF; // TODO: understand
        self.regs.flags.carry = overflow;

        result
    }

    pub fn gb_add(&mut self, reg_target: u8, value: u8) -> u8 {
        let (new_value, did_overflow) = reg_target.overflowing_add(value);

        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = false;   // we adding, not subtracting!
        self.regs.flags.half_carry = (reg_target & 0xF) + (value & 0xF) > 0xF; // TODO: Understand that
        self.regs.flags.carry = did_overflow;

        new_value
    }

    #[allow(unused)]
    pub fn gb_sub16(&mut self, reg_target: u16, value: u16) -> u16 {
        let (result, overflow) = reg_target.overflowing_sub(value);
        
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = (reg_target & 0xFFF) < (value & 0xFFF); // TODO: understand
        self.regs.flags.carry = overflow;

        result
    }

    pub fn gb_sub(&mut self, reg_target: u8, value: u8) -> u8 {
        let (new_value, carry) = reg_target.overflowing_sub(value);

        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = true;    // NOW we subtracting!
        self.regs.flags.half_carry = (reg_target & 0xF) < (value & 0xF); // TODO: Understand that
        self.regs.flags.carry = carry;

        new_value
    }

    pub fn push(&mut self, value: u16) -> () {
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        self.mem_bus.write_byte(self.regs.sp, ((value & 0xFF00) >> 8) as u8);

        self.regs.sp = self.regs.sp.wrapping_sub(1);
        self.mem_bus.write_byte(self.regs.sp, (value & 0x00FF) as u8);
    }

    pub fn pop(&mut self) -> u16 {
        // TODO: maybe shouldn't check?
        if self.regs.sp >= 0xFFFE {
            if DEBUG_ENABLED { log("Stack is 0xFFFE! Cannot pop anymore!"); }
            return 0;
        }

        let lsb = self.mem_bus.read_byte(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        let msb = self.mem_bus.read_byte(self.regs.sp) as u16;
        self.regs.sp = self.regs.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn get_jump_condition(&self, jump_type: JumpTypes) -> (bool, JumpTypes) {
        (match jump_type {
            JumpTypes::NotZero => !self.regs.flags.zero,
            JumpTypes::Zero => self.regs.flags.zero,
            JumpTypes::NotCarry => !self.regs.flags.carry,
            JumpTypes::Carry => self.regs.flags.carry,
            JumpTypes::Always => true
        }, jump_type)
    }

    pub fn read_next_byte(&self) -> u8 {
        self.mem_bus.memory[(self.pc + 1) as usize]
    }

    pub fn read_next_word(&self) -> u16 {
        let lsb = self.mem_bus.memory[(self.pc + 1) as usize] as u16;
        let msb = self.mem_bus.memory[(self.pc + 2) as usize] as u16;

        (msb << 8) | lsb
    }
}