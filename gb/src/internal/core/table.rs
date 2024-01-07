use crate::internal::core::component::{MicroInstr, Byte, CPU, Instruction};
use crate::internal::core::registers::{Register, Flag};
use std::fs::OpenOptions;
use std::io::prelude::*;

impl CPU {
    pub fn decode_instr(&self, opcode: u8) -> Vec<MicroInstr> {
        let instruction = match opcode {
            0x26 => Instruction{ name: format!("LD H, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::H)] },
            0x0E => Instruction{ name: format!("LD C, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C)] },
            0x06 => Instruction{ name: format!("LD B, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::B)] },
            0x2E => Instruction{ name: format!("LD L, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::L)] },
            0x16 => Instruction{ name: format!("LD D, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::D)] },
            0x1E => Instruction{ name: format!("LD E, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::E)] },
            0x11 => Instruction{ name: format!("LD DE, 0x{:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::E), MicroInstr::LDRN(Register::D)] },
            0x21 => Instruction{ name: format!("LD HL, 0x{:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::L), MicroInstr::LDRN(Register::H)] },
            0x01 => Instruction{ name: format!("LD BC, 0x{:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C), MicroInstr::LDRN(Register::B)] },
            0x47 => Instruction{ name: format!("LD B, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::B, Register::A)] },
            0x78 => Instruction{ name: format!("LD A, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::A, Register::B)] },
            0x7D => Instruction{ name: format!("LD A, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::A, Register::L)] },
            0x7C => Instruction{ name: format!("LD A, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::A, Register::H)] },
            0x5F => Instruction{ name: format!("LD E, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::E, Register::A)] },
            0x79 => Instruction{ name: format!("LD A, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::A, Register::C)] },
            0x4F => Instruction{ name: format!("LD C, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::C, Register::A)] },
            0x7A => Instruction{ name: format!("LD A, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::A, Register::D)] },
            0x57 => Instruction{ name: format!("LD D, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::D, Register::A)] },
            0x7B => Instruction{ name: format!("LD A, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::A, Register::E)] },
            0x6F => Instruction{ name: format!("LD L, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::L, Register::A)] },
            0x5D => Instruction{ name: format!("LD E, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::E, Register::L)] },
            0x67 => Instruction{ name: format!("LD H, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::H, Register::A)] },
            0x40 => Instruction{ name: format!("LD B, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::B, Register::B)] },
            0x41 => Instruction{ name: format!("LD B, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::B, Register::C)] },
            0x42 => Instruction{ name: format!("LD B, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::B, Register::D)] },
            0x43 => Instruction{ name: format!("LD B, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::B, Register::E)] },
            0x44 => Instruction{ name: format!("LD B, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::B, Register::H)] },
            0x45 => Instruction{ name: format!("LD B, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::B, Register::L)] },
            0x48 => Instruction{ name: format!("LD C, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::C, Register::B)] }, 
            0x49 => Instruction{ name: format!("LD C, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::C, Register::C)] }, 
            0x4A => Instruction{ name: format!("LD C, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::C, Register::D)] },
            0x4B => Instruction{ name: format!("LD C, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::C, Register::E)] },
            0x4C => Instruction{ name: format!("LD C, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::C, Register::H)] },
            0x4D => Instruction{ name: format!("LD C, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::C, Register::L)] },
            0x50 => Instruction{ name: format!("LD D, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::D, Register::B)] }, 
            0x51 => Instruction{ name: format!("LD D, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::D, Register::C)] }, 
            0x52 => Instruction{ name: format!("LD D, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::D, Register::D)] }, 
            0x53 => Instruction{ name: format!("LD D, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::D, Register::E)] }, 
            0x54 => Instruction{ name: format!("LD D, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::D, Register::H)] }, 
            0x55 => Instruction{ name: format!("LD D, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::D, Register::L)] },
            0x58 => Instruction{ name: format!("LD E, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::E, Register::B)] },
            0x59 => Instruction{ name: format!("LD E, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::E, Register::C)] },
            0x5A => Instruction{ name: format!("LD E, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::E, Register::D)] },
            0x5B => Instruction{ name: format!("LD E, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::E, Register::E)] },
            0x5C => Instruction{ name: format!("LD E, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::E, Register::H)] },
            0x60 => Instruction{ name: format!("LD H, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::H, Register::B)] },
            0x61 => Instruction{ name: format!("LD H, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::H, Register::C)] },
            0x62 => Instruction{ name: format!("LD H, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::H, Register::D)] },
            0x63 => Instruction{ name: format!("LD H, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::H, Register::E)] },
            0x64 => Instruction{ name: format!("LD H, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::H, Register::H)] },
            0x65 => Instruction{ name: format!("LD H, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::H, Register::L)] }, 
            0x68 => Instruction{ name: format!("LD L, B: 0x{:02X}", self.registers[Register::B]), steps: vec![MicroInstr::LDRR(Register::L, Register::B)] }, 
            0x69 => Instruction{ name: format!("LD L, C: 0x{:02X}", self.registers[Register::C]), steps: vec![MicroInstr::LDRR(Register::L, Register::C)] }, 
            0x6A => Instruction{ name: format!("LD L, D: 0x{:02X}", self.registers[Register::D]), steps: vec![MicroInstr::LDRR(Register::L, Register::D)] }, 
            0x6B => Instruction{ name: format!("LD L, E: 0x{:02X}", self.registers[Register::E]), steps: vec![MicroInstr::LDRR(Register::L, Register::E)] }, 
            0x6C => Instruction{ name: format!("LD L, H: 0x{:02X}", self.registers[Register::H]), steps: vec![MicroInstr::LDRR(Register::L, Register::H)] }, 
            0x6D => Instruction{ name: format!("LD L, L: 0x{:02X}", self.registers[Register::L]), steps: vec![MicroInstr::LDRR(Register::L, Register::L)] }, 
            0x7F => Instruction{ name: format!("LD A, A: 0x{:02X}", self.registers[Register::A]), steps: vec![MicroInstr::LDRR(Register::L, Register::L)] }, 
            0x2A => Instruction{ name: format!("LD A, (0x{:04X}: HL+)", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDAHLINC]},
            0x22 => Instruction{ name: format!("LD (0x{:04X}: HL+), A: 0x{:02X}", self.registers.get_hl(), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDHLINCA]},
            0x32 => Instruction{ name: format!("LD (0x{:04X}: HL-), A: 0x{:02X}", self.registers.get_hl(), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDHLDECA]},
            0x3A => Instruction{ name: format!("LD A, (0x{:04X}: HL-)", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDAHLDEC]},
            0x12 => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", self.registers.get_de(), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_de(), Register::A, false)] }, 
            0x77 => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", self.registers.get_hl(), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::A, false)] }, 
            0x72 => Instruction{ name: format!("LD (0x{:04X}), D: 0x{:02X}", self.registers.get_hl(), self.registers[Register::D]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::D, false)] }, 
            0x71 => Instruction{ name: format!("LD (0x{:04X}), C: 0x{:02X}", self.registers.get_hl(), self.registers[Register::C]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::C, false)] }, 
            0x70 => Instruction{ name: format!("LD (0x{:04X}), B: 0x{:02X}", self.registers.get_hl(), self.registers[Register::B]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::B, false)] }, 
            0x73 => Instruction{ name: format!("LD (0x{:04X}), E: 0x{:02X}", self.registers.get_hl(), self.registers[Register::E]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::E, false)] }, 
            0x74 => Instruction{ name: format!("LD (0x{:04X}), H: 0x{:02X}", self.registers.get_hl(), self.registers[Register::H]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::H, false)] }, 
            0x75 => Instruction{ name: format!("LD (0x{:04X}), L: 0x{:02X}", self.registers.get_hl(), self.registers[Register::L]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::L, false)] }, 
            0x02 => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", self.registers.get_bc(), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_bc(), Register::A, false)] }, 
            0x36 => Instruction{ name: format!("LD (0x{:04X}), 0x{:02X}", self.registers.get_hl(), self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDHLN]},
            0x1A => Instruction{ name: format!("LD A, (0x{:04X})", self.registers.get_de()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_de(), false)] }, 
            0x46 => Instruction{ name: format!("LD B, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::B, self.registers.get_hl(), false)] }, 
            0x4E => Instruction{ name: format!("LD C, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::C, self.registers.get_hl(), false)] }, 
            0x56 => Instruction{ name: format!("LD D, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::D, self.registers.get_hl(), false)] }, 
            0x6E => Instruction{ name: format!("LD L, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::L, self.registers.get_hl(), false)] }, 
            0x7E => Instruction{ name: format!("LD A, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_hl(), false)] },
            0x5E => Instruction{ name: format!("LD E, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::E, self.registers.get_hl(), false)] }, 
            0x66 => Instruction{ name: format!("LD H, (0x{:04X})", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::H, self.registers.get_hl(), false)] }, 
            0x0A => Instruction{ name: format!("LD A, (0x{:04X})", self.registers.get_bc()), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_bc(), false)] }, 
            0x31 => Instruction{ name: format!("LD SP, 0x{:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDSPNN]}, // 
            0x08 => Instruction{ name: format!("LD (0x{:04X}), SP: 0x{:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16), self.sp), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDNNSP(Byte::LSB), MicroInstr::LDNNSP(Byte::MSB)] },
            0xF9 => Instruction{ name: format!("LD SP, HL: 0x{:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::LDSPHL]},
            0xF8 => Instruction{ name: format!("LD HL, SP+i8"), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDHLSPN]},
            0xEA => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDNNR(0, Register::A, false)]},
            0x3E => Instruction{ name: format!("LD A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::NOP, MicroInstr::LDRN(Register::A)]},
            0xE0 => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", 0xFF00 | (self.bus.read(self.pc) as u16), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDNNR(0xFF00, Register::A, true)]},
            0xE2 => Instruction{ name: format!("LD (0x{:04X}), A: 0x{:02X}", 0xFF00 | (self.registers[Register::C] as u16), self.registers[Register::A]), steps: vec![MicroInstr::NOP, MicroInstr::LDNNR(0xFF00 + (self.registers[Register::C] as u16), Register::A, false)]},
            0xF0 => Instruction{ name: format!("LD A, (0x{:04X})", 0xFF00 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDRNN(Register::A, 0xFF00, true)]},
            0xF2 => Instruction{ name: format!("LD A, (0x{:04X})", 0xFF00 | (self.registers[Register::C] as u16)), steps: vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, 0xFF00 + (self.registers[Register::C] as u16), false)]},
            0xFA => Instruction{ name: format!("LD A, (0x{:04X})", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDRNN(Register::A, 0, false)]},

            0x18 => Instruction{ name: format!("JR i8"), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::NOP, MicroInstr::JR]},
            0x20 => Instruction{ name: format!("JR NZ, i8"), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::JR]},
            0x30 => Instruction{ name: format!("JR NC, i8"), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::C, false), MicroInstr::JR]},
            0x38 => Instruction{ name: format!("JR C, i8"), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::C, true), MicroInstr::JR]},
            0x28 => Instruction{ name: format!("JR Z, i8"), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::JR]},
            0xC3 => Instruction{ name: format!("JP ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::NOP, MicroInstr::JP]},
            0xC2 => Instruction{ name: format!("JP NZ, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::JP]},
            0xCA => Instruction{ name: format!("JP Z, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::JP]},
            0xD2 => Instruction{ name: format!("JP NC, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, false), MicroInstr::JP]},
            0xDA => Instruction{ name: format!("JP C, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, true), MicroInstr::JP]},
            0xE9 => Instruction{ name: format!("JP ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::JPHL] },
            0xCD => Instruction{ name: format!("CALL ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP]},
            0xC4 => Instruction{ name: format!("CALL NZ, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP]}, // CALL NZ,u16
            0xCC => Instruction{ name: format!("CALL Z, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP]},
            0xD4 => Instruction{ name: format!("CALL NC, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, false), MicroInstr::PUSH(((self.pc + 2) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP]},
            0xDC => Instruction{ name: format!("CALL C, ${:04X}", (self.bus.read(self.pc + 1) as u16) << 8 | (self.bus.read(self.pc) as u16)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, true), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP]},
            0xC9 => Instruction{ name: format!("RET"), steps: vec![MicroInstr::NOP, MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP] },
            0xD0 => Instruction{ name: format!("RET NC"), steps: vec![MicroInstr::NOP, MicroInstr::Cond(Flag::C, false), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP] },
            0xC8 => Instruction{ name: format!("RET Z"), steps: vec![MicroInstr::NOP, MicroInstr::Cond(Flag::Z, true), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP] },
            0xC0 => Instruction{ name: format!("RET NZ"), steps: vec![MicroInstr::NOP, MicroInstr::Cond(Flag::Z, false), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP] },
            0xD8 => Instruction{ name: format!("RET C"), steps: vec![MicroInstr::NOP, MicroInstr::Cond(Flag::C, true), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP] },
            0xD9 => Instruction{ name: format!("RETI"), steps: vec![MicroInstr::NOP, MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::RETI] },
            0xC7 => Instruction{ name: format!("RST 00h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0000)] }, 
            0xCF => Instruction{ name: format!("RST 08h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0008)] }, 
            0xD7 => Instruction{ name: format!("RST 10h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0010)] }, 
            0xDF => Instruction{ name: format!("RST 18h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0018)] }, 
            0xE7 => Instruction{ name: format!("RST 20h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0020)] }, 
            0xEF => Instruction{ name: format!("RST 28h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0028)] }, 
            0xF7 => Instruction{ name: format!("RST 30h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0030)] }, 
            0xFF => Instruction{ name: format!("RST 38h"), steps: vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0038)] }, 

            0x34 => Instruction{ name: format!("INC ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::INCHLADDR]},
            0x13 => Instruction{ name: format!("INC DE"), steps: vec![MicroInstr::NOP, MicroInstr::INCDE] },
            0x23 => Instruction{ name: format!("INC HL"), steps: vec![MicroInstr::NOP, MicroInstr::INCHL] },
            0x03 => Instruction{ name: format!("INC BC"), steps: vec![MicroInstr::NOP, MicroInstr::INCBC] },
            0x33 => Instruction{ name: format!("INC SP"), steps: vec![MicroInstr::NOP, MicroInstr::INCSP] },
            0x1C => Instruction{ name: format!("INC E"), steps: vec![MicroInstr::INC(Register::E)] },
            0x14 => Instruction{ name: format!("INC D"), steps: vec![MicroInstr::INC(Register::D)] },
            0x2C => Instruction{ name: format!("INC L"), steps: vec![MicroInstr::INC(Register::L)] },
            0x24 => Instruction{ name: format!("INC H"), steps: vec![MicroInstr::INC(Register::H)] },
            0x3C => Instruction{ name: format!("INC A"), steps: vec![MicroInstr::INC(Register::A)] },
            0x04 => Instruction{ name: format!("INC B"), steps: vec![MicroInstr::INC(Register::B)] },
            0x0C => Instruction{ name: format!("INC C"), steps: vec![MicroInstr::INC(Register::C)] },
            
            0x1B => Instruction{ name: format!("DEC DE"), steps: vec![MicroInstr::NOP, MicroInstr::DECDE] },
            0x0B => Instruction{ name: format!("DEC BC"), steps: vec![MicroInstr::NOP, MicroInstr::DECBC] },
            0x2B => Instruction{ name: format!("DEC HL"), steps: vec![MicroInstr::NOP, MicroInstr::DECHL] },
            0x3B => Instruction{ name: format!("DEC SP"), steps: vec![MicroInstr::NOP, MicroInstr::DECSP] },
            0x0D => Instruction{ name: format!("DEC C"), steps: vec![MicroInstr::DEC(Register::C)] },
            0x05 => Instruction{ name: format!("DEC B"), steps: vec![MicroInstr::DEC(Register::B)] },
            0x2D => Instruction{ name: format!("DEC L"), steps: vec![MicroInstr::DEC(Register::L)] },
            0x25 => Instruction{ name: format!("DEC H"), steps: vec![MicroInstr::DEC(Register::H)] },
            0x3D => Instruction{ name: format!("DEC A"), steps: vec![MicroInstr::DEC(Register::A)] },
            0x1D => Instruction{ name: format!("DEC E"), steps: vec![MicroInstr::DEC(Register::E)] },
            0x15 => Instruction{ name: format!("DEC D"), steps: vec![MicroInstr::DEC(Register::D)] },
            0x35 => Instruction{ name: format!("DEC ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::DECNN(self.registers.get_hl())]},
            
            0xB1 => Instruction{ name: format!("OR A, C"), steps: vec![MicroInstr::OR(Register::C)] },
            0xB7 => Instruction{ name: format!("OR A, A"), steps: vec![MicroInstr::OR(Register::A)] },
            0xB0 => Instruction{ name: format!("OR A, B"), steps: vec![MicroInstr::OR(Register::B)] },
            0xB2 => Instruction{ name: format!("OR A, D"), steps: vec![MicroInstr::OR(Register::D)] },
            0xB3 => Instruction{ name: format!("OR A, E"), steps: vec![MicroInstr::OR(Register::E)] },
            0xB4 => Instruction{ name: format!("OR A, H"), steps: vec![MicroInstr::OR(Register::H)] },
            0xB5 => Instruction{ name: format!("OR A, L"), steps: vec![MicroInstr::OR(Register::L)] },
            0xB6 => Instruction{ name: format!("OR A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::ORHL] },
            0xF6 => Instruction{ name: format!("OR A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::ORN] },

            0xAF => Instruction{ name: format!("XOR A, A"), steps: vec![MicroInstr::XOR(Register::A)] },
            0xA9 => Instruction{ name: format!("XOR A, C"), steps: vec![MicroInstr::XOR(Register::C)] },
            0xAD => Instruction{ name: format!("XOR A, L"), steps: vec![MicroInstr::XOR(Register::L)] },
            0xA8 => Instruction{ name: format!("XOR A, B"), steps: vec![MicroInstr::XOR(Register::B)] },
            0xAA => Instruction{ name: format!("XOR A, D"), steps: vec![MicroInstr::XOR(Register::D)] },
            0xAB => Instruction{ name: format!("XOR A, E"), steps: vec![MicroInstr::XOR(Register::E)] },
            0xAC => Instruction{ name: format!("XOR A, H"), steps: vec![MicroInstr::XOR(Register::H)] },
            0xEE => Instruction{ name: format!("XOR A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::XORN]},
            0xAE => Instruction{ name: format!("XOR A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::XORHL]},

            0xA0 => Instruction{ name: format!("AND A,B"), steps: vec![MicroInstr::AND(Register::B)] },
            0xA1 => Instruction{ name: format!("AND A,C"), steps: vec![MicroInstr::AND(Register::C)] },
            0xA2 => Instruction{ name: format!("AND A,D"), steps: vec![MicroInstr::AND(Register::D)] },
            0xA3 => Instruction{ name: format!("AND A,E"), steps: vec![MicroInstr::AND(Register::E)] },
            0xA4 => Instruction{ name: format!("AND A,H"), steps: vec![MicroInstr::AND(Register::H)] },
            0xA5 => Instruction{ name: format!("AND A,L"), steps: vec![MicroInstr::AND(Register::L)] },
            0xA7 => Instruction{ name: format!("AND A,A"), steps: vec![MicroInstr::AND(Register::A)] },
            0xA6 => Instruction{ name: format!("AND A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::ANDHL]},
            0xE6 => Instruction{ name: format!("AND A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::ANDN]},

            0xBB => Instruction{ name: format!("CP A, E"), steps: vec![MicroInstr::CP(Register::E)] },
            0xBA => Instruction{ name: format!("CP A, D"), steps: vec![MicroInstr::CP(Register::D)] },
            0xB9 => Instruction{ name: format!("CP A, C"), steps: vec![MicroInstr::CP(Register::C)] },
            0xB8 => Instruction{ name: format!("CP A, B"), steps: vec![MicroInstr::CP(Register::B)] },
            0xBC => Instruction{ name: format!("CP A, H"), steps: vec![MicroInstr::CP(Register::H)] },
            0xBD => Instruction{ name: format!("CP A, L"), steps: vec![MicroInstr::CP(Register::L)] },
            0xBF => Instruction{ name: format!("CP A, A"), steps: vec![MicroInstr::CP(Register::A)] },
            0xBE => Instruction{ name: format!("CP A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::CPHL]},
            0xFE => Instruction{ name: format!("CP A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::CPN]},

            0x80 => Instruction{ name: format!("ADD A, B"), steps: vec![MicroInstr::ADD(Register::B)] }, 
            0x81 => Instruction{ name: format!("ADD A, C"), steps: vec![MicroInstr::ADD(Register::C)] }, 
            0x82 => Instruction{ name: format!("ADD A, D"), steps: vec![MicroInstr::ADD(Register::D)] }, 
            0x83 => Instruction{ name: format!("ADD A, E"), steps: vec![MicroInstr::ADD(Register::E)] }, 
            0x84 => Instruction{ name: format!("ADD A, H"), steps: vec![MicroInstr::ADD(Register::H)] }, 
            0x85 => Instruction{ name: format!("ADD A, L"), steps: vec![MicroInstr::ADD(Register::L)] }, 
            0x87 => Instruction{ name: format!("ADD A, A"), steps: vec![MicroInstr::ADD(Register::A)] }, 
            0x86 => Instruction{ name: format!("ADD A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::ADDHL] },
            0xC6 => Instruction{ name: format!("ADD A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::ADDN]},
            0x29 => Instruction{ name: format!("ADD HL, HL"), steps: vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_hl())] }, 
            0x09 => Instruction{ name: format!("ADD HL, BC"), steps: vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_bc())] }, 
            0x19 => Instruction{ name: format!("ADD HL, DE"), steps: vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_de())] }, 
            0x39 => Instruction{ name: format!("ADD HL, SP"), steps: vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.sp)]},
            0xE8 => Instruction{ name: format!("ADD SP, i8"), steps: vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::NOP, MicroInstr::ADDSPN]}, // ADD SP,i8

            0x88 => Instruction{ name: format!("ADC A, B"), steps: vec![MicroInstr::ADC(Register::B)] }, 
            0x89 => Instruction{ name: format!("ADC A, C"), steps: vec![MicroInstr::ADC(Register::C)] }, 
            0x8A => Instruction{ name: format!("ADC A, D"), steps: vec![MicroInstr::ADC(Register::D)] }, 
            0x8B => Instruction{ name: format!("ADC A, E"), steps: vec![MicroInstr::ADC(Register::E)] }, 
            0x8C => Instruction{ name: format!("ADC A, H"), steps: vec![MicroInstr::ADC(Register::H)] }, 
            0x8D => Instruction{ name: format!("ADC A, L"), steps: vec![MicroInstr::ADC(Register::L)] }, 
            0x8F => Instruction{ name: format!("ADC A, A"), steps: vec![MicroInstr::ADC(Register::A)] }, 
            0x8E => Instruction{ name: format!("ADC A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::ADCHL] },
            0xCE => Instruction{ name: format!("ADC A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::ADCN] },

            0x90 => Instruction{ name: format!("SUB A, B"), steps: vec![MicroInstr::SUB(Register::B)] }, 
            0x91 => Instruction{ name: format!("SUB A, C"), steps: vec![MicroInstr::SUB(Register::C)] }, 
            0x92 => Instruction{ name: format!("SUB A, D"), steps: vec![MicroInstr::SUB(Register::D)] }, 
            0x93 => Instruction{ name: format!("SUB A, E"), steps: vec![MicroInstr::SUB(Register::E)] }, 
            0x94 => Instruction{ name: format!("SUB A, H"), steps: vec![MicroInstr::SUB(Register::H)] }, 
            0x95 => Instruction{ name: format!("SUB A, L"), steps: vec![MicroInstr::SUB(Register::L)] }, 
            0x97 => Instruction{ name: format!("SUB A, A"), steps: vec![MicroInstr::SUB(Register::A)] }, 
            0x96 => Instruction{ name: format!("SUB A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::SUBHL] },
            0xD6 => Instruction{ name: format!("SUB A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::SUBN] },

            0x98 => Instruction{ name: format!("SBC A, B"), steps: vec![MicroInstr::SBC(Register::B)] }, 
            0x99 => Instruction{ name: format!("SBC A, C"), steps: vec![MicroInstr::SBC(Register::C)] }, 
            0x9A => Instruction{ name: format!("SBC A, D"), steps: vec![MicroInstr::SBC(Register::D)] }, 
            0x9B => Instruction{ name: format!("SBC A, E"), steps: vec![MicroInstr::SBC(Register::E)] }, 
            0x9C => Instruction{ name: format!("SBC A, H"), steps: vec![MicroInstr::SBC(Register::H)] }, 
            0x9D => Instruction{ name: format!("SBC A, L"), steps: vec![MicroInstr::SBC(Register::L)] }, 
            0x9F => Instruction{ name: format!("SBC A, A"), steps: vec![MicroInstr::SBC(Register::A)] }, 
            0x9E => Instruction{ name: format!("SBC A, ${:04X}", self.registers.get_hl()), steps: vec![MicroInstr::NOP, MicroInstr::SBCHL] },
            0xDE => Instruction{ name: format!("SBC A, 0x{:02X}", self.bus.read(self.pc)), steps: vec![MicroInstr::Read(Byte::LSB), MicroInstr::SBCN] },

            0xF5 => Instruction{ name: format!("PUSH AF"), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::A]), MicroInstr::PUSH(self.registers[Register::F])] }, 
            0xE5 => Instruction{ name: format!("PUSH HL"), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::H]), MicroInstr::PUSH(self.registers[Register::L])] }, 
            0xC5 => Instruction{ name: format!("PUSH BC"), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::B]), MicroInstr::PUSH(self.registers[Register::C])] }, 
            0xD5 => Instruction{ name: format!("PUSH DE"), steps: vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::D]), MicroInstr::PUSH(self.registers[Register::E])] }, 
            0xE1 => Instruction{ name: format!("POP HL"), steps: vec![MicroInstr::NOP, MicroInstr::POPR(Register::L), MicroInstr::POPR(Register::H)] }, 
            0xF1 => Instruction{ name: format!("POP AF"), steps: vec![MicroInstr::NOP, MicroInstr::POPR(Register::F), MicroInstr::POPR(Register::A)] }, 
            0xC1 => Instruction{ name: format!("POP BC"), steps: vec![MicroInstr::NOP, MicroInstr::POPR(Register::C), MicroInstr::POPR(Register::B)] }, 
            0xD1 => Instruction{ name: format!("POP DE"), steps: vec![MicroInstr::NOP, MicroInstr::POPR(Register::E), MicroInstr::POPR(Register::D)] }, 

            0x00 => Instruction{ name: format!("NOP"), steps: vec![MicroInstr::NOP] },
            0x1F => Instruction{ name: format!("RRA"), steps: vec![MicroInstr::RRA] },
            0x2F => Instruction{ name: format!("CPL"), steps: vec![MicroInstr::CPL] },
            0x27 => Instruction{ name: format!("DAA"), steps: vec![MicroInstr::DAA] },
            0x37 => Instruction{ name: format!("SCF"), steps: vec![MicroInstr::SCF] },
            0x3F => Instruction{ name: format!("CCF"), steps: vec![MicroInstr::CCF] },
            0xF3 => Instruction{ name: format!("DI"), steps: vec![MicroInstr::DI] },
            0x07 => Instruction{ name: format!("RLCA"), steps: vec![MicroInstr::RLCA] },
            0x17 => Instruction{ name: format!("RLA"), steps: vec![MicroInstr::RLA] },
            0x0F => Instruction{ name: format!("RRCA"), steps: vec![MicroInstr::RRCA] },
            0xFB => Instruction{ name: format!("EI"), steps: vec![MicroInstr::EI] },
            0x76 => Instruction{ name: format!("HALT"), steps: vec![MicroInstr::HALT] },
            0x10 => Instruction{ name: format!("STOP"), steps: vec![MicroInstr::STOP] },
            0xCB => Instruction{ name: format!(""), steps: vec![] },

            _ => panic!("Unexpected opcode encountered")
        };

        if !self.bus.boot_rom_mounted && self.bus.debug {
            let line = format!("{} ~ PC: ${:04X} IF: 0x{:08b} IE: 0x{:08b} IME: {}", instruction.name, self.pc - 1, self.bus.intf, self.bus.inte, self.ime);
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("output.txt")
                .unwrap();

            if let Err(e) = writeln!(file, "{}", line) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }

        return instruction.steps
    }
}