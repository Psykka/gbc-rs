pub struct Memory {
    ram: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; 0xFFFF]
        }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        self.ram[addr as usize] = data
    }

    pub fn read(&self, addr: u8) -> u8 {
        self.ram[addr as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn byte_test() {
        let mut mem = Memory::new();
        mem.write(0xFF, 0x01);

        assert_eq!(mem.read(0xFF), 0x01);
    }
}