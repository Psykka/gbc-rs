use crate::registers::{Registers, ByteReg, WordReg};
use crate::bus::Bus;
use crate::types::Size;

pub struct SM83 {
    reg: Registers,
    bus: Bus,
    pc: u16
}

impl SM83 {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            bus: Bus::new(),
            pc: 0,
        }
    }

    pub fn step(&mut self) {
        let op = self.bus.mem.read(Size::Byte, self.pc as usize) as u8;
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

            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn adc_r(&mut self, reg: ByteReg) {
        self.reg.setByte(ByteReg::A, self.reg.a + self.reg.getByte(reg) + self.reg.getByte(ByteReg::C));

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(false);
        self.reg.check_half_carry(self.reg.a as u16);
        self.reg.check_carry(self.reg.a as u16);
    }

    fn adc_hl(&mut self) {
        let hl = self.reg.getWord(WordReg::HL);
        let data = self.bus.mem.read(Size::Byte, hl as usize) as u8;
        self.reg.setByte(ByteReg::A, self.reg.a + data + self.reg.getByte(ByteReg::C));

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(false);
        self.reg.check_half_carry(self.reg.a as u16);
        self.reg.check_carry(self.reg.a as u16);
    }

    fn adc_n(&mut self) {
        let n = self.bus.mem.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.reg.setByte(ByteReg::A, self.reg.a + n + self.reg.getByte(ByteReg::C));

        self.reg.check_zero(self.reg.a);
        self.reg.subtract(false);
        self.reg.check_half_carry(self.reg.a as u16);
        self.reg.check_carry(self.reg.a as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_r() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x88);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x89);
        cpu.bus.mem.write(Size::Byte, 0x101, 0x8f);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x07);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_adc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.setWord(WordReg::HL, 0x102);
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x8e);
        cpu.bus.mem.write(Size::Byte, 0x102, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);
    }

    #[test]
    fn test_adc_n() {
        let mut cpu = SM83::new();
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0xce);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }
}