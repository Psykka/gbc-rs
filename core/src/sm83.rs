use crate::registers::Registers;
use crate::bus::Bus;

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
        let op = self.bus.mem.read(self.pc as u8);
        self.pc += 1;
        self.run_instruction(op);
    }

    fn run_instruction(&mut self, op: u8) {
        match op {
            0x88 => self.adc(),
            _ => panic!("Unimplemented opcode: {:02x}", op),
        }
    }

    fn adc(&mut self) {
        self.reg.a += self.reg.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.c = 0x02;
        cpu.pc = 0xff;
        cpu.bus.mem.write(0xff, 0x88);

        cpu.step();


        assert_eq!(cpu.reg.a, 0x03)
    }

    // #[test]
    // fn test_sum () {
    //     let z80 = Z80::new();
    //     let result = z80.sum(1, 2);
    //     assert_eq!(result, 3);
    // }
}