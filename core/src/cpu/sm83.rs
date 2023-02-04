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
            0x8f => self.adc_r(ByteReg::A),
            0x88 => self.adc_r(ByteReg::B),
            0x89 => self.adc_r(ByteReg::C),
            0x8a => self.adc_r(ByteReg::D),
            0x8b => self.adc_r(ByteReg::E),
            0x8c => self.adc_r(ByteReg::H),
            0x8d => self.adc_r(ByteReg::L),

            // ADC A, (HL)
            0x8e => self.adc_hl(),

            // ADC A, n
            0xce => self.adc_n(),

            // ADD A, r
            0x80 => self.add_r(ByteReg::B),
            0x81 => self.add_r(ByteReg::C),
            0x82 => self.add_r(ByteReg::D),
            0x83 => self.add_r(ByteReg::E),
            0x84 => self.add_r(ByteReg::H),
            0x85 => self.add_r(ByteReg::L),
            0x87 => self.add_r(ByteReg::A),

            // ADD A, (HL)
            0x86 => self.add_hl(),

            // ADD A, n
            0xc6 => self.add_n(),

            // ADD HL, r
            0x09 => self.add_hl_rr(WordReg::BC),
            0x19 => self.add_hl_rr(WordReg::DE),
            0x29 => self.add_hl_rr(WordReg::HL),
            0x39 => self.add_hl_rr(WordReg::SP),

            // ADD SP, n
            0xe8 => self.add_sp_n(),

            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn adc_r(&mut self, reg: ByteReg) {
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + self.reg.get_byte(reg) + self.reg.get_byte(ByteReg::C),
        );

        check_all!(self, reg, self.reg.a, false);
    }

    fn adc_hl(&mut self) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;
        self.reg.set_byte(
            ByteReg::A,
            self.reg.a + data + self.reg.get_byte(ByteReg::C),
        );

        check_all!(self, hl, self.reg.a, false);
    }

    fn adc_n(&mut self) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_byte(ByteReg::A, self.reg.a + n + self.reg.get_byte(ByteReg::C));

        check_all!(self, n, self.reg.a, false);
    }

    fn add_r(&mut self, reg: ByteReg) {
        self.reg
            .set_byte(ByteReg::A, self.reg.a + self.reg.get_byte(reg));

        check_all!(self, reg, self.reg.a, false);
    }

    fn add_hl(&mut self) {
        let hl = self.reg.get_word(WordReg::HL);
        let data = self.bus.read(Size::Byte, hl as usize) as u8;
        self.reg.set_byte(ByteReg::A, self.reg.a + data);

        check_all!(self, hl, self.reg.a, false);
    }

    fn add_n(&mut self) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.set_byte(ByteReg::A, self.reg.a + n);

        check_all!(self, n, self.reg.a, false);
    }

    fn add_hl_rr(&mut self, reg: WordReg) {
        self.reg.set_word(
            WordReg::HL,
            self.reg.get_word(WordReg::HL) + self.reg.get_word(reg),
        );

        check_all!(self, reg, self.reg.get_word(WordReg::HL) as u8, false);
    }

    fn add_sp_n(&mut self) {
        let n = self.bus.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg
            .set_word(WordReg::SP, self.reg.get_word(WordReg::SP) + n as u16);

        check_all_carrys!(self, n, self.reg.get_word(WordReg::SP) as u8);
    }
}

impl Default for SM83 {
    fn default() -> Self {
        Self::new()
    }
}
