use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::types::Size;

const ROM_BANK_00: usize = 0x0000;
const ROM_BANK_00_END: usize = 0x3fff;

const WRAM_00: usize = 0xC000;
const WRAM_00_END: usize = 0xCFFF;

const WRAM_01: usize = 0xD000;
const WRAM_01_END: usize = 0xDFFF;

pub struct Bus {
    pub mem: Memory,
    pub rom: Cartridge,
    pub cycles: usize,
}

impl Bus {
    pub fn new(cart: Option<Cartridge>) -> Self {
        Self {
            mem: Memory::default(),
            rom: cart.unwrap_or_default(),
            cycles: 0,
        }
    }

    pub fn read(&self, size: Size, addr: usize) -> usize {
        match addr {
            ROM_BANK_00 ..= ROM_BANK_00_END => self.rom.read(size, addr),
            WRAM_00 ..= WRAM_00_END => self.mem.read(size, addr - WRAM_00),
            WRAM_01 ..= WRAM_01_END => self.rom.ram.read(size, addr - WRAM_01),
            _ => {
                println!("Ignored read from address: {:04X}", addr);
                0
            }
        }
    }

    pub fn write(&mut self, size: Size, addr: usize, data: usize) {
        match addr {
            WRAM_00 ..= WRAM_00_END => self.mem.write(size, addr - WRAM_00, data),
            WRAM_01 ..= WRAM_01_END => self.rom.ram.write(size, addr - WRAM_01, data),
            _ => println!("Ignored write to address: {:04X}", addr),
        }
    }

    pub fn tick(&mut self, times: usize) {
        if times <= 0 {
            return;
        }

        for _ in 0..times {
            self.cycles += 1;
            println!("Tick! {}", self.cycles);
            // All components who need to be ticked
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(None)
    }
}
