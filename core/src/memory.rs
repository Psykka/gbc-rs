use crate::types::Size;

pub struct Memory {
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self { ram: vec![0; size] }
    }

    pub fn size(&self) -> usize {
        self.ram.len()
    }

    pub fn write(&mut self, size: Size, addr: usize, data: usize) {
        match size {
            Size::Byte => self.write_byte(addr, data),
            Size::Word => self.write_word(addr, data),
        }
    }

    fn write_byte(&mut self, addr: usize, data: usize) {
        self.ram[addr as usize] = data as u8
    }

    fn write_word(&mut self, addr: usize, data: usize) {
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

impl Default for Memory {
    fn default() -> Self {
        Self::new(0xFFFF)
    }
}
