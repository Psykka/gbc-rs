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

pub struct SM83 {
    pub reg: Registers,
    pub bus: Bus,
    pub pc: u16,
}

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

            // ADD HL, r
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

            _ => panic!("Unimplemented opcode: {:02x}", op),
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
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + data + self.reg.get_carry() as u8,
        );

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, false);
    }

    fn adc_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_byte(ByteReg::A, self.reg.a + n + self.reg.get_carry() as u8,);

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

        check_all!(self, reg, self.reg.get_word(WordReg::HL) as u8, false);
    }

    fn add_sp_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_word(WordReg::SP, self.reg.get_word(WordReg::SP) + n as u16);

        self.bus.tick(cycles);

        check_all_carrys!(self, n, self.reg.get_word(WordReg::SP) as u8);
    }

    fn and_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg
            .set_byte(ByteReg::A, self.reg.a & self.reg.get_byte(reg));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.check_half_carry(self.reg.a as u16);
    }

    fn and_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a & data);
        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.check_half_carry(self.reg.a as u16);
    }

    fn and_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a & n);

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.check_half_carry(self.reg.a as u16);
    }

    fn cp_r_a(&mut self, cycles: usize) {
        let a = self.reg.a;

        self.reg.check_zero(a.wrapping_sub(a));
        self.reg.subtract(true);

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

        self.reg.check_zero(self.reg.a);
    }

    fn or_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a | data);
        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
    }

    fn or_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a | n);

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
    }

    fn sbc_r_a(&mut self, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(self.reg.a - self.reg.get_carry() as u8)
        );

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(true);
        self.reg.check_half_carry(self.reg.a as u16);
    }

    fn sbc_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a.wrapping_sub(self.reg.get_byte(reg) - self.reg.get_carry() as u8)
        );

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, true);
    }

    fn sbc_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(data - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, true);
    }

    fn sbc_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;

        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(n - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, true);
    }

    fn sub_r_a(&mut self, cycles: usize) {
        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(self.reg.a - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(true);
    }

    fn sub_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(self.reg.get_byte(reg) - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, true);
    }

    fn sub_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;

        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(data - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, true);
    }

    fn sub_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;

        self.reg.set_byte(ByteReg::A, self.reg.a.wrapping_sub(n - self.reg.get_carry() as u8));

        self.bus.tick(cycles);

        check_all!(self, n, self.reg.a, true);
    }
}

impl Default for SM83 {
    fn default() -> Self {
        Self::new()
    }
}
