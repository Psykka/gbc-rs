use crate::registers::{Registers, ByteReg};
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

    fn step(&mut self) {
        let op = self.bus.mem.read(Size::Byte, self.pc as usize) as u8;
        self.pc += 1;
        self.run_instruction(op);
    }

    fn run_instruction(&mut self, op: u8) {
        match op {
            0x88 => self.adc_r(ByteReg::B),
            0x89 => self.adc_r(ByteReg::C),

            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn adc_r(&mut self, reg: ByteReg) {
        self.reg.setByte(ByteReg::A, self.reg.getByte(reg) + self.reg.getByte(ByteReg::C));
        
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
    fn test_adc_b() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.pc = 0xff;
        cpu.bus.mem.write(Size::Byte, 0xff, 0x88);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03)
    }
}