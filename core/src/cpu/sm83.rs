use crate::bus::Bus;
use crate::cpu::registers::{ByteReg, Registers, WordReg};
use crate::types::Size;

macro_rules! check_all {
    ($self:ident, $reg:ident, $val:expr, $subtract:expr) => {
        $self.reg.check_zero($val);
        $self.reg.subtract($subtract);
        $self.reg.check_half_carry($val as u16);
        $self.reg.check_carry($val as u16);
    };
}

macro_rules! check_all_carrys {
    ($self:ident, $reg:ident, $val:expr) => {
        $self.reg.check_half_carry($val as u16);
        $self.reg.check_carry($val as u16);
    };
}

const ZERO: u8 = 0b1000_0000;
const SUBTRACT: u8 = 0b0100_0000;
const HALF_CARRY: u8 = 0b0010_0000;
// const CARRY: u8 = 0b0001_0000;

pub struct SM83 {
    pub reg: Registers,
    pub bus: Bus,
    pub pc: u16,
}

// TODO: remove self.reg.substract, set with self.reg.set_flags
impl SM83 {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            bus: Bus::new(None),
            pc: 0x100,
        }
    }

    pub fn step(&mut self) {
        let op = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.run_instruction(op);
    }

    fn run_instruction(&mut self, op: u8) {
        match op {
            // ADC A, r
            0x88 => self.adc_r(ByteReg::B, 4),
            0x89 => self.adc_r(ByteReg::C, 4),
            0x8a => self.adc_r(ByteReg::D, 4),
            0x8b => self.adc_r(ByteReg::E, 4),
            0x8c => self.adc_r(ByteReg::H, 4),
            0x8d => self.adc_r(ByteReg::L, 4),
            0x8f => self.adc_r(ByteReg::A, 4),

            // ADC A, (HL)
            0x8e => self.adc_hl(8),

            // ADC A, n
            0xce => self.adc_n(8),

            // ADD A, r
            0x80 => self.add_r(ByteReg::B, 4),
            0x81 => self.add_r(ByteReg::C, 4),
            0x82 => self.add_r(ByteReg::D, 4),
            0x83 => self.add_r(ByteReg::E, 4),
            0x84 => self.add_r(ByteReg::H, 4),
            0x85 => self.add_r(ByteReg::L, 4),
            0x87 => self.add_r(ByteReg::A, 4),

            // ADD A, (HL)
            0x86 => self.add_hl(8),

            // ADD A, n
            0xc6 => self.add_n(8),

            // ADD HL, rr
            0x09 => self.add_hl_rr(WordReg::BC, 8),
            0x19 => self.add_hl_rr(WordReg::DE, 8),
            0x29 => self.add_hl_rr(WordReg::HL, 8),
            0x39 => self.add_hl_rr(WordReg::SP, 8),

            // ADD SP, n
            0xe8 => self.add_sp_n(16),

            // AND A, r
            0xa0 => self.and_r(ByteReg::B, 4),
            0xa1 => self.and_r(ByteReg::C, 4),
            0xa2 => self.and_r(ByteReg::D, 4),
            0xa3 => self.and_r(ByteReg::E, 4),
            0xa4 => self.and_r(ByteReg::H, 4),
            0xa5 => self.and_r(ByteReg::L, 4),
            0xa7 => self.and_r(ByteReg::A, 4),

            // AND A, (HL)
            0xa6 => self.and_hl(8),

            // AND A, n
            0xe6 => self.and_n(8),

            // CP A, r
            0xbf => self.cp_r_a(4),
            0xb8 => self.cp_r(ByteReg::B, 4),
            0xb9 => self.cp_r(ByteReg::C, 4),
            0xba => self.cp_r(ByteReg::D, 4),
            0xbb => self.cp_r(ByteReg::E, 4),
            0xbc => self.cp_r(ByteReg::H, 4),
            0xbd => self.cp_r(ByteReg::L, 4),

            // CP A, (HL)
            0xbe => self.cp_hl(8),

            // CP A, n
            0xfe => self.cp_n(8),

            // DEC r
            0x3d => self.dec_r(ByteReg::A, 4),
            0x05 => self.dec_r(ByteReg::B, 4),
            0x0d => self.dec_r(ByteReg::C, 4),
            0x15 => self.dec_r(ByteReg::D, 4),
            0x1d => self.dec_r(ByteReg::E, 4),
            0x25 => self.dec_r(ByteReg::H, 4),
            0x2d => self.dec_r(ByteReg::L, 4),

            // DEC (HL)
            0x35 => self.dec_hl(12),

            // DEC rr
            0x0b => self.dec_rr(WordReg::BC, 8),
            0x1b => self.dec_rr(WordReg::DE, 8),
            0x2b => self.dec_rr(WordReg::HL, 8),
            0x3b => self.dec_rr(WordReg::SP, 8),

            // INC r
            0x3c => self.inc_r(ByteReg::A, 4),
            0x04 => self.inc_r(ByteReg::B, 4),
            0x0c => self.inc_r(ByteReg::C, 4),
            0x14 => self.inc_r(ByteReg::D, 4),
            0x1c => self.inc_r(ByteReg::E, 4),
            0x24 => self.inc_r(ByteReg::H, 4),
            0x2c => self.inc_r(ByteReg::L, 4),

            // INC (HL)
            0x34 => self.inc_hl(12),

            // INC rr
            0x03 => self.inc_rr(WordReg::BC, 8),
            0x13 => self.inc_rr(WordReg::DE, 8),
            0x23 => self.inc_rr(WordReg::HL, 8),
            0x33 => self.inc_rr(WordReg::SP, 8),

            // OR A, r
            0xb7 => self.or_r(ByteReg::A, 4),
            0xb0 => self.or_r(ByteReg::B, 4),
            0xb1 => self.or_r(ByteReg::C, 4),
            0xb2 => self.or_r(ByteReg::D, 4),
            0xb3 => self.or_r(ByteReg::E, 4),
            0xb4 => self.or_r(ByteReg::H, 4),
            0xb5 => self.or_r(ByteReg::L, 4),

            // OR A, (HL)
            0xb6 => self.or_hl(8),

            // OR A, n
            0xf6 => self.or_n(8),

            // SBC A, r
            0x9f => self.sbc_r_a(4),
            0x98 => self.sbc_r(ByteReg::B, 4),
            0x99 => self.sbc_r(ByteReg::C, 4),
            0x9a => self.sbc_r(ByteReg::D, 4),
            0x9b => self.sbc_r(ByteReg::E, 4),
            0x9c => self.sbc_r(ByteReg::H, 4),
            0x9d => self.sbc_r(ByteReg::L, 4),

            // SBC A, (HL)
            0x9e => self.sbc_hl(8),

            // SBC A, n
            0xde => self.sbc_n(8),

            // SUB A, r
            0x97 => self.sub_r_a(4),
            0x90 => self.sub_r(ByteReg::B, 4),
            0x91 => self.sub_r(ByteReg::C, 4),
            0x92 => self.sub_r(ByteReg::D, 4),
            0x93 => self.sub_r(ByteReg::E, 4),
            0x94 => self.sub_r(ByteReg::H, 4),
            0x95 => self.sub_r(ByteReg::L, 4),

            // SUB A, (HL)
            0x96 => self.sub_hl(8),

            // SUB A, n
            0xd6 => self.sub_n(8),

            // XOR A, r
            0xaf => self.xor_r_a(4),
            0xa8 => self.xor_r(ByteReg::B, 4),
            0xa9 => self.xor_r(ByteReg::C, 4),
            0xaa => self.xor_r(ByteReg::D, 4),
            0xab => self.xor_r(ByteReg::E, 4),
            0xac => self.xor_r(ByteReg::H, 4),
            0xad => self.xor_r(ByteReg::L, 4),

            // XOR A, (HL)
            0xae => self.xor_hl(8),

            // XOR A, n
            0xee => self.xor_n(8),

            // PREFIX CB
            0xcb => self.prefix_cb(),

            // RLA
            // 0x17 => self.rla(4),

            // RLCA
            // 0x07 => self.rlca(4),

            // RRA
            // 0x1f => self.rra(4),

            // RRCA
            // 0x0f => self.rrca(4),

            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn prefix_cb(&mut self) {
        let op = self.bus.read(Size::Byte, self.pc as usize);
        self.pc += 1;

        match op {
            // BIT b0, r
            0x47 => self.bit_r(0, ByteReg::A, 8),
            0x40 => self.bit_r(0, ByteReg::B, 8),
            0x41 => self.bit_r(0, ByteReg::C, 8),
            0x42 => self.bit_r(0, ByteReg::D, 8),
            0x43 => self.bit_r(0, ByteReg::E, 8),
            0x44 => self.bit_r(0, ByteReg::H, 8),
            0x45 => self.bit_r(0, ByteReg::L, 8),

            // BIT b0, (HL)
            0x46 => self.bit_hl(0, 12),

            // BIT b1, r
            0x4f => self.bit_r(1, ByteReg::A, 8),
            0x48 => self.bit_r(1, ByteReg::B, 8),
            0x49 => self.bit_r(1, ByteReg::C, 8),
            0x4a => self.bit_r(1, ByteReg::D, 8),
            0x4b => self.bit_r(1, ByteReg::E, 8),
            0x4c => self.bit_r(1, ByteReg::H, 8),
            0x4d => self.bit_r(1, ByteReg::L, 8),

            // BIT b1, (HL)
            0x4e => self.bit_hl(1, 12),

            // BIT b2, r
            0x57 => self.bit_r(2, ByteReg::A, 8),
            0x50 => self.bit_r(2, ByteReg::B, 8),
            0x51 => self.bit_r(2, ByteReg::C, 8),
            0x52 => self.bit_r(2, ByteReg::D, 8),
            0x53 => self.bit_r(2, ByteReg::E, 8),
            0x54 => self.bit_r(2, ByteReg::H, 8),
            0x55 => self.bit_r(2, ByteReg::L, 8),

            // BIT b2, (HL)
            0x56 => self.bit_hl(2, 12),

            // BIT b3, r
            0x5f => self.bit_r(3, ByteReg::A, 8),
            0x58 => self.bit_r(3, ByteReg::B, 8),
            0x59 => self.bit_r(3, ByteReg::C, 8),
            0x5a => self.bit_r(3, ByteReg::D, 8),
            0x5b => self.bit_r(3, ByteReg::E, 8),
            0x5c => self.bit_r(3, ByteReg::H, 8),
            0x5d => self.bit_r(3, ByteReg::L, 8),

            // BIT b3, (HL)
            0x5e => self.bit_hl(3, 12),

            // BIT b4, r
            0x67 => self.bit_r(4, ByteReg::A, 8),
            0x60 => self.bit_r(4, ByteReg::B, 8),
            0x61 => self.bit_r(4, ByteReg::C, 8),
            0x62 => self.bit_r(4, ByteReg::D, 8),
            0x63 => self.bit_r(4, ByteReg::E, 8),
            0x64 => self.bit_r(4, ByteReg::H, 8),
            0x65 => self.bit_r(4, ByteReg::L, 8),

            // BIT b4, (HL)
            0x66 => self.bit_hl(4, 12),

            // BIT b5, r
            0x6f => self.bit_r(5, ByteReg::A, 8),
            0x68 => self.bit_r(5, ByteReg::B, 8),
            0x69 => self.bit_r(5, ByteReg::C, 8),
            0x6a => self.bit_r(5, ByteReg::D, 8),
            0x6b => self.bit_r(5, ByteReg::E, 8),
            0x6c => self.bit_r(5, ByteReg::H, 8),
            0x6d => self.bit_r(5, ByteReg::L, 8),

            // BIT b5, (HL)
            0x6e => self.bit_hl(5, 12),

            // BIT b6, r
            0x77 => self.bit_r(6, ByteReg::A, 8),
            0x70 => self.bit_r(6, ByteReg::B, 8),
            0x71 => self.bit_r(6, ByteReg::C, 8),
            0x72 => self.bit_r(6, ByteReg::D, 8),
            0x73 => self.bit_r(6, ByteReg::E, 8),
            0x74 => self.bit_r(6, ByteReg::H, 8),
            0x75 => self.bit_r(6, ByteReg::L, 8),

            // BIT b6, (HL)
            0x76 => self.bit_hl(6, 12),

            // BIT b7, r
            0x7f => self.bit_r(7, ByteReg::A, 8),
            0x78 => self.bit_r(7, ByteReg::B, 8),
            0x79 => self.bit_r(7, ByteReg::C, 8),
            0x7a => self.bit_r(7, ByteReg::D, 8),
            0x7b => self.bit_r(7, ByteReg::E, 8),
            0x7c => self.bit_r(7, ByteReg::H, 8),
            0x7d => self.bit_r(7, ByteReg::L, 8),

            // BIT b7, (HL)
            0x7e => self.bit_hl(7, 12),

            // RES b0, r
            0x87 => self.res_r(0, ByteReg::A, 8),
            0x80 => self.res_r(0, ByteReg::B, 8),
            0x81 => self.res_r(0, ByteReg::C, 8),
            0x82 => self.res_r(0, ByteReg::D, 8),
            0x83 => self.res_r(0, ByteReg::E, 8),
            0x84 => self.res_r(0, ByteReg::H, 8),
            0x85 => self.res_r(0, ByteReg::L, 8),

            // RES b0, (HL)
            0x86 => self.res_hl(0, 16),

            // RES b1, r
            0x8f => self.res_r(1, ByteReg::A, 8),
            0x88 => self.res_r(1, ByteReg::B, 8),
            0x89 => self.res_r(1, ByteReg::C, 8),
            0x8a => self.res_r(1, ByteReg::D, 8),
            0x8b => self.res_r(1, ByteReg::E, 8),
            0x8c => self.res_r(1, ByteReg::H, 8),
            0x8d => self.res_r(1, ByteReg::L, 8),

            // RES b1, (HL)
            0x8e => self.res_hl(1, 16),

            // RES b2, r
            0x97 => self.res_r(2, ByteReg::A, 8),
            0x90 => self.res_r(2, ByteReg::B, 8),
            0x91 => self.res_r(2, ByteReg::C, 8),
            0x92 => self.res_r(2, ByteReg::D, 8),
            0x93 => self.res_r(2, ByteReg::E, 8),
            0x94 => self.res_r(2, ByteReg::H, 8),
            0x95 => self.res_r(2, ByteReg::L, 8),
            
            // RES b2, (HL)
            0x96 => self.res_hl(2, 16),

            // RES b3, r
            0x9f => self.res_r(3, ByteReg::A, 8),
            0x98 => self.res_r(3, ByteReg::B, 8),
            0x99 => self.res_r(3, ByteReg::C, 8),
            0x9a => self.res_r(3, ByteReg::D, 8),
            0x9b => self.res_r(3, ByteReg::E, 8),
            0x9c => self.res_r(3, ByteReg::H, 8),
            0x9d => self.res_r(3, ByteReg::L, 8),

            // RES b3, (HL)
            0x9e => self.res_hl(3, 16),

            // RES b4, r
            0xa7 => self.res_r(4, ByteReg::A, 8),
            0xa0 => self.res_r(4, ByteReg::B, 8),
            0xa1 => self.res_r(4, ByteReg::C, 8),
            0xa2 => self.res_r(4, ByteReg::D, 8),
            0xa3 => self.res_r(4, ByteReg::E, 8),
            0xa4 => self.res_r(4, ByteReg::H, 8),
            0xa5 => self.res_r(4, ByteReg::L, 8),

            // RES b4, (HL)
            0xa6 => self.res_hl(4, 16),

            // RES b5, r
            0xaf => self.res_r(5, ByteReg::A, 8),
            0xa8 => self.res_r(5, ByteReg::B, 8),
            0xa9 => self.res_r(5, ByteReg::C, 8),
            0xaa => self.res_r(5, ByteReg::D, 8),
            0xab => self.res_r(5, ByteReg::E, 8),
            0xac => self.res_r(5, ByteReg::H, 8),
            0xad => self.res_r(5, ByteReg::L, 8),

            // RES b5, (HL)
            0xae => self.res_hl(5, 16),
            
            // RES b6, r
            0xb7 => self.res_r(6, ByteReg::A, 8),
            0xb0 => self.res_r(6, ByteReg::B, 8),
            0xb1 => self.res_r(6, ByteReg::C, 8),
            0xb2 => self.res_r(6, ByteReg::D, 8),
            0xb3 => self.res_r(6, ByteReg::E, 8),
            0xb4 => self.res_r(6, ByteReg::H, 8),
            0xb5 => self.res_r(6, ByteReg::L, 8),
            
            // RES b6, (HL)
            0xb6 => self.res_hl(6, 16),

            // RES b7, r
            0xbf => self.res_r(7, ByteReg::A, 8),
            0xb8 => self.res_r(7, ByteReg::B, 8),
            0xb9 => self.res_r(7, ByteReg::C, 8),
            0xba => self.res_r(7, ByteReg::D, 8),
            0xbb => self.res_r(7, ByteReg::E, 8),
            0xbc => self.res_r(7, ByteReg::H, 8),
            0xbd => self.res_r(7, ByteReg::L, 8),

            // RES b7, (HL)
            0xbe => self.res_hl(7, 16),

            // SET b0, r
            0xc7 => self.set_r(0, ByteReg::A, 8),
            0xc0 => self.set_r(0, ByteReg::B, 8),
            0xc1 => self.set_r(0, ByteReg::C, 8),
            0xc2 => self.set_r(0, ByteReg::D, 8),
            0xc3 => self.set_r(0, ByteReg::E, 8),
            0xc4 => self.set_r(0, ByteReg::H, 8),
            0xc5 => self.set_r(0, ByteReg::L, 8),

            // SET b0, (HL)
            0xc6 => self.set_hl(0, 16),

            // SET b1, r
            0xcf => self.set_r(1, ByteReg::A, 8),
            0xc8 => self.set_r(1, ByteReg::B, 8),
            0xc9 => self.set_r(1, ByteReg::C, 8),
            0xca => self.set_r(1, ByteReg::D, 8),
            0xcb => self.set_r(1, ByteReg::E, 8),
            0xcc => self.set_r(1, ByteReg::H, 8),
            0xcd => self.set_r(1, ByteReg::L, 8),

            // SET b1, (HL)
            0xce => self.set_hl(1, 16),

            // SET b2, r
            0xd7 => self.set_r(2, ByteReg::A, 8),
            0xd0 => self.set_r(2, ByteReg::B, 8),
            0xd1 => self.set_r(2, ByteReg::C, 8),
            0xd2 => self.set_r(2, ByteReg::D, 8),
            0xd3 => self.set_r(2, ByteReg::E, 8),
            0xd4 => self.set_r(2, ByteReg::H, 8),
            0xd5 => self.set_r(2, ByteReg::L, 8),

            // SET b2, (HL)
            0xd6 => self.set_hl(2, 16),

            // SET b3, r
            0xdf => self.set_r(3, ByteReg::A, 8),
            0xd8 => self.set_r(3, ByteReg::B, 8),
            0xd9 => self.set_r(3, ByteReg::C, 8),
            0xda => self.set_r(3, ByteReg::D, 8),
            0xdb => self.set_r(3, ByteReg::E, 8),
            0xdc => self.set_r(3, ByteReg::H, 8),
            0xdd => self.set_r(3, ByteReg::L, 8),

            // SET b3, (HL)
            0xde => self.set_hl(3, 16),

            // SET b4, r
            0xe7 => self.set_r(4, ByteReg::A, 8),
            0xe0 => self.set_r(4, ByteReg::B, 8),
            0xe1 => self.set_r(4, ByteReg::C, 8),
            0xe2 => self.set_r(4, ByteReg::D, 8),
            0xe3 => self.set_r(4, ByteReg::E, 8),
            0xe4 => self.set_r(4, ByteReg::H, 8),
            0xe5 => self.set_r(4, ByteReg::L, 8),

            // SET b4, (HL)
            0xe6 => self.set_hl(4, 16),

            // SET b5, r
            0xef => self.set_r(5, ByteReg::A, 8),
            0xe8 => self.set_r(5, ByteReg::B, 8),
            0xe9 => self.set_r(5, ByteReg::C, 8),
            0xea => self.set_r(5, ByteReg::D, 8),
            0xeb => self.set_r(5, ByteReg::E, 8),
            0xec => self.set_r(5, ByteReg::H, 8),
            0xed => self.set_r(5, ByteReg::L, 8),

            // SET b5, (HL)
            0xee => self.set_hl(5, 16),

            // SET b6, r
            0xf7 => self.set_r(6, ByteReg::A, 8),
            0xf0 => self.set_r(6, ByteReg::B, 8),
            0xf1 => self.set_r(6, ByteReg::C, 8),
            0xf2 => self.set_r(6, ByteReg::D, 8),
            0xf3 => self.set_r(6, ByteReg::E, 8),
            0xf4 => self.set_r(6, ByteReg::H, 8),
            0xf5 => self.set_r(6, ByteReg::L, 8),

            // SET b6, (HL)
            0xf6 => self.set_hl(6, 16),

            // SET b7, r
            0xff => self.set_r(7, ByteReg::A, 8),
            0xf8 => self.set_r(7, ByteReg::B, 8),
            0xf9 => self.set_r(7, ByteReg::C, 8),
            0xfa => self.set_r(7, ByteReg::D, 8),
            0xfb => self.set_r(7, ByteReg::E, 8),
            0xfc => self.set_r(7, ByteReg::H, 8),
            0xfd => self.set_r(7, ByteReg::L, 8),

            // SET b7, (HL)
            0xfe => self.set_hl(7, 16),

            // SWAP r
            0x37 => self.swap_r(ByteReg::A, 8),
            0x30 => self.swap_r(ByteReg::B, 8),
            0x31 => self.swap_r(ByteReg::C, 8),
            0x32 => self.swap_r(ByteReg::D, 8),
            0x33 => self.swap_r(ByteReg::E, 8),
            0x34 => self.swap_r(ByteReg::H, 8),
            0x35 => self.swap_r(ByteReg::L, 8),

            // SWAP (HL)
            0x36 => self.swap_hl(16),

            // RL r
            0x17 => self.rl_r(ByteReg::A, 8),
            0x10 => self.rl_r(ByteReg::B, 8),
            0x11 => self.rl_r(ByteReg::C, 8),
            0x12 => self.rl_r(ByteReg::D, 8),
            0x13 => self.rl_r(ByteReg::E, 8),
            0x14 => self.rl_r(ByteReg::H, 8),
            0x15 => self.rl_r(ByteReg::L, 8),

            // RL (HL)
            0x16 => self.rl_hl(16),

            // // RLC r
            // 0x07 => self.rlc_r(ByteReg::A, 8),
            // 0x00 => self.rlc_r(ByteReg::B, 8),
            // 0x01 => self.rlc_r(ByteReg::C, 8),
            // 0x02 => self.rlc_r(ByteReg::D, 8),
            // 0x03 => self.rlc_r(ByteReg::E, 8),
            // 0x04 => self.rlc_r(ByteReg::H, 8),
            // 0x05 => self.rlc_r(ByteReg::L, 8),

            // // RLC (HL)
            // 0x06 => self.rlc_hl(16),

            // // RR r
            // 0x1f => self.rr_r(ByteReg::A, 8),
            // 0x18 => self.rr_r(ByteReg::B, 8),
            // 0x19 => self.rr_r(ByteReg::C, 8),
            // 0x1a => self.rr_r(ByteReg::D, 8),
            // 0x1b => self.rr_r(ByteReg::E, 8),
            // 0x1c => self.rr_r(ByteReg::H, 8),
            // 0x1d => self.rr_r(ByteReg::L, 8),

            // // RR (HL)
            // 0x1e => self.rr_hl(16),

            // // RRC r
            // 0x0f => self.rrc_r(ByteReg::A, 8),
            // 0x08 => self.rrc_r(ByteReg::B, 8),
            // 0x09 => self.rrc_r(ByteReg::C, 8),
            // 0x0a => self.rrc_r(ByteReg::D, 8),
            // 0x0b => self.rrc_r(ByteReg::E, 8),
            // 0x0c => self.rrc_r(ByteReg::H, 8),
            // 0x0d => self.rrc_r(ByteReg::L, 8),

            // // RRC (HL)
            // 0x0e => self.rrc_hl(16),

            // // SLA r
            // 0x27 => self.sla_r(ByteReg::A, 8),
            // 0x20 => self.sla_r(ByteReg::B, 8),
            // 0x21 => self.sla_r(ByteReg::C, 8),
            // 0x22 => self.sla_r(ByteReg::D, 8),
            // 0x23 => self.sla_r(ByteReg::E, 8),
            // 0x24 => self.sla_r(ByteReg::H, 8),
            // 0x25 => self.sla_r(ByteReg::L, 8),

            // // SLA (HL)
            // 0x26 => self.sla_hl(16),

            // // SRA r
            // 0x2f => self.sra_r(ByteReg::A, 8),
            // 0x28 => self.sra_r(ByteReg::B, 8),
            // 0x29 => self.sra_r(ByteReg::C, 8),
            // 0x2a => self.sra_r(ByteReg::D, 8),
            // 0x2b => self.sra_r(ByteReg::E, 8),
            // 0x2c => self.sra_r(ByteReg::H, 8),
            // 0x2d => self.sra_r(ByteReg::L, 8),

            // // SRA (HL)
            // 0x2e => self.sra_hl(16),

            // // SRL r
            // 0x3f => self.srl_r(ByteReg::A, 8),
            // 0x38 => self.srl_r(ByteReg::B, 8),
            // 0x39 => self.srl_r(ByteReg::C, 8),
            // 0x3a => self.srl_r(ByteReg::D, 8),
            // 0x3b => self.srl_r(ByteReg::E, 8),
            // 0x3c => self.srl_r(ByteReg::H, 8),
            // 0x3d => self.srl_r(ByteReg::L, 8),

            // // SRL (HL)
            // 0x3e => self.srl_hl(16),

            _ => panic!("Unimplemented prefix $cb: {:02x}", op),
        }
    }

    fn adc_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + self.reg.get_byte(reg) + self.reg.get_carry() as u8,
        );

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, false);
    }

    fn adc_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;
        self.reg
            .set_byte(ByteReg::A, self.reg.a + data + self.reg.get_carry() as u8);

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, false);
    }

    fn adc_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_byte(ByteReg::A, self.reg.a + n + self.reg.get_carry() as u8);

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, false);
    }

    fn add_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(ByteReg::A, self.reg.a + self.reg.get_byte(reg));

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, false);
    }

    fn add_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a + data);
        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, false);
    }

    fn add_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a + n);

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, false);
    }

    fn add_hl_rr(&mut self, reg: WordReg, cycles: usize) {
        self.reg.set_word(
            WordReg::HL,
            self.reg.get_word(WordReg::HL) + self.reg.get_word(reg),
        );

        self.bus.tick(cycles);

        self.reg.subtract(false);
        self.reg
            .check_half_carry(self.reg.get_word(WordReg::HL) as u16);
        self.reg.check_carry(self.reg.get_word(WordReg::HL) as u16);
    }

    fn add_sp_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_word(WordReg::SP, self.reg.get_word(WordReg::SP) + n as u16);

        self.bus.tick(cycles);

        self.reg.set_flags(0);
        check_all_carrys!(self, n, self.reg.get_word(WordReg::SP) as u8);
    }

    fn and_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(ByteReg::A, self.reg.a & self.reg.get_byte(reg));

        self.bus.tick(cycles);

        self.reg.set_flags(HALF_CARRY);
        self.reg.check_zero(self.reg.a);
    }

    fn and_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a & data);
        self.bus.tick(cycles);

        self.reg.set_flags(HALF_CARRY);
        self.reg.check_zero(self.reg.a);
    }

    fn and_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a & n);

        self.bus.tick(cycles);

        self.reg.set_flags(HALF_CARRY);
        self.reg.check_zero(self.reg.a);
    }

    fn cp_r_a(&mut self, cycles: usize) {
        let a = self.reg.a;

        self.reg.check_zero(a.wrapping_sub(a));

        self.reg.set_flags(ZERO | SUBTRACT);

        self.bus.tick(cycles);
    }

    fn cp_r(&mut self, reg: ByteReg, cycles: usize) {
        let a = self.reg.a;
        let b = self.reg.get_byte(reg);

        check_all!(self, reg, a.wrapping_sub(b), true);

        self.bus.tick(cycles);
    }

    fn cp_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;
        let a = self.reg.a;

        check_all!(self, hl, a.wrapping_sub(data), true);

        self.bus.tick(cycles);
    }

    fn cp_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        let a = self.reg.a;

        check_all!(self, n, a.wrapping_sub(n), true);

        self.bus.tick(cycles);
    }

    fn dec_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(reg, self.reg.get_byte(reg).wrapping_sub(1));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.get_byte(reg));
        self.reg.subtract(true);
        self.reg.check_half_carry(self.reg.get_byte(reg) as u16);
    }

    fn dec_hl(&mut self, cycles: usize) {
        self.reg
            .set_word(WordReg::HL, self.reg.get_word(WordReg::HL).wrapping_sub(1));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.get_word(WordReg::HL) as u8);
        self.reg.subtract(true);
        self.reg.check_half_carry(self.reg.get_word(WordReg::HL));
    }

    fn dec_rr(&mut self, reg: WordReg, cycles: usize) {
        let data = self.reg.get_word(reg).wrapping_sub(1);

        self.reg.set_word(reg, data);

        self.bus.tick(cycles);
    }

    fn inc_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(reg, self.reg.get_byte(reg).wrapping_add(1));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.get_byte(reg));
        self.reg.subtract(false);
        self.reg.check_half_carry(self.reg.get_byte(reg) as u16);
    }

    fn inc_hl(&mut self, cycles: usize) {
        self.reg
            .set_word(WordReg::HL, self.reg.get_word(WordReg::HL).wrapping_add(1));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.get_word(WordReg::HL) as u8);
        self.reg.subtract(false);
        self.reg.check_half_carry(self.reg.get_word(WordReg::HL));
    }

    fn inc_rr(&mut self, reg: WordReg, cycles: usize) {
        let data = self.reg.get_word(reg).wrapping_add(1);

        self.reg.set_word(reg, data);

        self.bus.tick(cycles);
    }

    fn or_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(ByteReg::A, self.reg.a | self.reg.get_byte(reg));

        self.bus.tick(cycles);

        self.reg.set_flags(0);
        self.reg.check_zero(self.reg.a);
    }

    fn or_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a | data);
        self.bus.tick(cycles);

        self.reg.set_flags(0);
        self.reg.check_zero(self.reg.a);
    }

    fn or_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a | n);

        self.bus.tick(cycles);

        self.reg.set_flags(0);
        self.reg.check_zero(self.reg.a);
    }

    fn sbc_r_a(&mut self, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg
                .a
                .wrapping_sub(self.reg.a - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(true);
        self.reg.check_half_carry(self.reg.a as u16);
    }

    fn sbc_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg
                .a
                .wrapping_sub(self.reg.get_byte(reg) - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, true);
    }

    fn sbc_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(data - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, true);
    }

    fn sbc_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;

        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(n - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, true);
    }

    fn sub_r_a(&mut self, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg
                .a
                .wrapping_sub(self.reg.a - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        self.reg.set_flags(ZERO | SUBTRACT);
        self.reg.check_zero(self.reg.a);
    }

    fn sub_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg
                .a
                .wrapping_sub(self.reg.get_byte(reg) - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, true);
    }

    fn sub_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(data - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, true);
    }

    fn sub_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;

        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(n - self.reg.get_carry() as u8),
        );

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, true);
    }

    fn xor_r_a(&mut self, cycles: usize) {
        self.bus.tick(cycles);

        self.reg.set_flags(ZERO);
    }

    fn xor_r(&mut self, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a ^ self.reg.get_byte(reg));
    }

    fn xor_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a ^ data);
    }

    fn xor_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a ^ n);
    }

    fn bit_r(&mut self, bit: u8, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        self.reg.set_flags(HALF_CARRY);
        self.reg.check_zero(self.reg.get_byte(reg) & (1 << bit));
    }

    fn bit_hl(&mut self, bit: u8, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        self.reg.set_flags(HALF_CARRY);
        self.reg.check_zero(data & (1 << bit));
    }

    fn res_r(&mut self, bit: u8, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        println!("#{:4X}",self. reg.get_byte(reg) & !(1 << bit));
        self.reg.set_byte(reg, self.reg.get_byte(reg) & !(1 << bit));
    }

    fn res_hl(&mut self, bit: u8, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL) as usize;
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        self.bus.write(Size::Byte, hl, (data & !(1 << bit)) as usize);
    }

    fn set_r(&mut self, bit: u8, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        self.reg.set_byte(reg, self.reg.get_byte(reg) | (1 << bit));
    }

    fn set_hl(&mut self, bit: u8, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL) as usize;
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        self.bus.write(Size::Byte, hl, (data | (1 << bit)) as usize);
    }

    fn swap_r(&mut self, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        let data = self.reg.get_byte(reg);
        let swapped = (data << 4) | (data >> 4);

        self.reg.set_byte(reg, swapped);

        self.reg.set_flags(0);
        self.reg.check_zero(swapped);
    }

    fn swap_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL) as usize;
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        let swapped = (data << 4) | (data >> 4);

        self.bus.write(Size::Byte, hl, swapped as usize);

        self.reg.set_flags(0);
        self.reg.check_zero(swapped);
    }

    fn rl_r(&mut self, reg: ByteReg, cycles: usize) {
        self.bus.tick(cycles);

        let data = self.reg.get_byte(reg);
        let carry = self.reg.get_carry() as u8;

        let result: u16 = ((data << 1) | carry) as u16;

        self.reg.set_byte(reg, result as u8);

        self.reg.set_flags(0);
        self.reg.check_carry(result);
        self.reg.check_zero(self.reg.get_byte(reg));
    }

    fn rl_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL) as usize;
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.bus.tick(cycles);

        let carry = self.reg.get_carry() as u8;

        let result: u16 = ((data << 1) | carry) as u16;

        self.bus.write(Size::Byte, hl, result as usize);

        self.reg.set_flags(0);
        self.reg.check_carry(result);
        self.reg.check_zero(result as u8);
    }
}

impl Default for SM83 {
    fn default() -> Self {
        Self::new()
    }
}
