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
            0xbf => self.cp_r(ByteReg::A, 4),
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

            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn adc_r(&mut self, reg: ByteReg, cycles: usize) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + self.reg.get_byte(reg) + self.reg.get_byte(ByteReg::C),
        );

        self.bus.tick(cycles);

        check_all!(self, reg, self.reg.a, false);
    }

    fn adc_hl(&mut self, cycles: usize) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + data + self.reg.get_byte(ByteReg::C),
        );

        self.bus.tick(cycles);

        check_all!(self, hl, self.reg.a, false);
    }

    fn adc_n(&mut self, cycles: usize) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_byte(ByteReg::A, self.reg.a + n + self.reg.get_byte(ByteReg::C));

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
}

impl Default for SM83 {
    fn default() -> Self {
        Self::new()
    }
}
