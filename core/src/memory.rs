use crate::types::Size;

pub struct Memory {
    ram: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; 0xFFFF]
        }
    }

    pub fn write(&mut self, size: Size, addr: u8, data: usize) {
        match size {
            Size::Byte => self.write_byte(addr, data),
            Size::Word => self.write_word(addr, data)
        }
    }

    fn write_byte(&mut self, addr: u8, data: usize) {
        self.ram[addr as usize] = data as u8
    }
    
    fn write_word(&mut self, addr: u8, data: usize) {
        self.ram[addr as usize] = (data & 0xff) as u8;
        self.ram[(addr + 1) as usize] = ((data >> 8) & 0xff) as u8;
    }

    pub fn read(&self, size: Size, addr: usize) -> usize {
        match size {
            Size::Byte => self.read_byte(addr),
            Size::Word => self.read_word(addr),
        }
    }

    fn read_byte(&self, addr: usize) -> usize {
        self.ram[addr as usize] as usize
    }

    fn read_word(&self, addr: usize) -> usize {
        (self.ram[addr as usize] as usize) | ((self.ram[(addr + 1) as usize] as usize) << 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_byte() {
        let mut mem = Memory::new();
        mem.write(Size::Byte, 0x00, 0x01);
        assert_eq!(mem.read(Size::Byte, 0x00), 0x01);
    }

    #[test]
    fn test_word() {
        let mut mem = Memory::new();
        mem.write(Size::Word, 0x00, 0x0102);
        assert_eq!(mem.read(Size::Word, 0x00), 0x0102);
    }
}