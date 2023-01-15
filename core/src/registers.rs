pub enum WordReg {
    AF,
    BC,
    DE,
    HL
}

#[derive(Debug)]
pub enum ByteReg {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn getWord(&self, wreg: WordReg) -> u16 {
        match wreg {
            WordReg::AF => ((self.a as u16) << 8) | (self.f as u16),
            WordReg::BC => ((self.b as u16) << 8) | (self.c as u16),
            WordReg::DE => ((self.d as u16) << 8) | (self.e as u16),
            WordReg::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    pub fn setWord(&mut self, wreg: WordReg, data: u16) {
        match wreg {
            WordReg::AF => {
                self.a = (data >> 8) as u8;
                self.f = data as u8;
            },
            WordReg::BC => {
                self.b = (data >> 8) as u8;
                self.c = data as u8;
            },
            WordReg::DE => {
                self.d = (data >> 8) as u8;
                self.e = data as u8;
            },
            WordReg::HL => {
                self.h = (data >> 8) as u8;
                self.l = data as u8;
            },
        }
    }

    pub fn getByte(&self, breg: ByteReg) -> u8 {
        match breg {
            ByteReg::A => self.a,
            ByteReg::B => self.b,
            ByteReg::C => self.c,
            ByteReg::D => self.d,
            ByteReg::E => self.e,
            ByteReg::F => self.f,
            ByteReg::H => self.h,
            ByteReg::L => self.l,
        }
    }

    pub fn setByte(&mut self, breg: ByteReg, data: u8) {
        match breg {
            ByteReg::A => self.a = data,
            ByteReg::B => self.b = data,
            ByteReg::C => self.c = data,
            ByteReg::D => self.d = data,
            ByteReg::E => self.e = data,
            ByteReg::F => self.f = data,
            ByteReg::H => self.h = data,
            ByteReg::L => self.l = data,
        }
    }

    pub fn check_zero(&mut self, data: u8) {
        if data == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }

    pub fn subtract(&mut self, data: bool) {
        if data {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }

    pub fn check_half_carry(&mut self, data: u16) {
        if data > 0xf {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    pub fn check_carry(&mut self, data: u16) {
        if data > 0xff {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_register() {
        let mut regs = Registers::new();
        regs.setByte(ByteReg::A, 0x01);
        assert_eq!(regs.getByte(ByteReg::A), 0x01);
    }

    #[test]
    fn test_word_register() {
        let mut regs = Registers::new();
        regs.setWord(WordReg::AF, 0x0102);
        assert_eq!(regs.getWord(WordReg::AF), 0x0102);
    }

    #[test]
    fn test_check_zero() {
        let mut regs = Registers::new();

        regs.check_zero(0);
        assert_eq!(regs.f, 0x80);

        regs.check_zero(1);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_subtract() {
        let mut regs = Registers::new();

        regs.subtract(true);
        assert_eq!(regs.f, 0x40);

        regs.subtract(false);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_half_carry() {
        let mut regs = Registers::new();

        regs.check_half_carry(0x10);
        assert_eq!(regs.f, 0x20);

        regs.check_half_carry(0x0f);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_carry() {
        let mut regs = Registers::new();

        regs.check_carry(0x100);
        assert_eq!(regs.f, 0x10);

        regs.check_carry(0xff);
        assert_eq!(regs.f, 0);
    }
}