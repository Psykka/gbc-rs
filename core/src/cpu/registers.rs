pub enum WordReg {
    AF,
    BC,
    DE,
    HL,
    SP,
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
    L,
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
    pub sp: u16,
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
            sp: 0,
        }
    }

    pub fn get_word(&self, wreg: WordReg) -> u16 {
        match wreg {
            WordReg::AF => ((self.a as u16) << 8) | (self.f as u16),
            WordReg::BC => ((self.b as u16) << 8) | (self.c as u16),
            WordReg::DE => ((self.d as u16) << 8) | (self.e as u16),
            WordReg::HL => ((self.h as u16) << 8) | (self.l as u16),
            WordReg::SP => self.sp,
        }
    }

    pub fn set_word(&mut self, wreg: WordReg, data: u16) {
        match wreg {
            WordReg::AF => {
                self.a = (data >> 8) as u8;
                self.f = data as u8;
            }
            WordReg::BC => {
                self.b = (data >> 8) as u8;
                self.c = data as u8;
            }
            WordReg::DE => {
                self.d = (data >> 8) as u8;
                self.e = data as u8;
            }
            WordReg::HL => {
                self.h = (data >> 8) as u8;
                self.l = data as u8;
            }
            WordReg::SP => self.sp = data,
        }
    }

    pub fn get_byte(&self, breg: ByteReg) -> u8 {
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

    pub fn set_byte(&mut self, breg: ByteReg, data: u8) {
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

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}