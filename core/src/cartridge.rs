use crate::{memory::Memory, types::Size};
use std::string;

pub struct Cartridge {
    pub title: String,
    pub cgb_flag: u8,
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub new_licensee_code: u16,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,

    pub rom: Vec<u8>,
    pub ram: Memory,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let mut cart = Self {
            title: String::new(),
            cgb_flag: 0,
            sgb_flag: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            destination_code: 0,
            old_licensee_code: 0,
            new_licensee_code: 0,
            mask_rom_version_number: 0,
            header_checksum: 0,
            global_checksum: 0,

            rom: Vec::new(),
            ram: Memory::new(0),
        };

        cart.title = string::String::from_utf8(rom[0x134..0x143].to_vec())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string();

        cart.cgb_flag = rom[0x143];
        cart.sgb_flag = rom[0x146];
        cart.cartridge_type = rom[0x147];
        cart.rom_size = rom[0x148];
        cart.ram_size = rom[0x149];
        cart.destination_code = rom[0x14A];
        cart.old_licensee_code = rom[0x14B];
        cart.new_licensee_code = (rom[0x144] as u16) << 8 | rom[0x145] as u16;
        cart.mask_rom_version_number = rom[0x14C];
        cart.header_checksum = rom[0x14D];
        cart.global_checksum = (rom[0x14E] as u16) << 8 | rom[0x14F] as u16;

        cart.rom = rom.to_owned();

        match cart.ram_size {
            0x00 => cart.ram = Memory::new(0),
            0x01 => cart.ram = Memory::new(2 * 1024),
            0x02 => cart.ram = Memory::new(8 * 1024),
            0x03 => cart.ram = Memory::new(32 * 1024),
            0x04 => cart.ram = Memory::new(128 * 1024),
            0x05 => cart.ram = Memory::new(64 * 1024),
            _ => return Err(format!("Invalid RAM size: {:#X}", cart.ram_size)),
        }

        Ok(cart)
    }

    pub fn read(&self, size: Size, addr: usize) -> usize {
        match size {
            Size::Byte => self.rom[addr] as usize,
            Size::Word => {
                (self.rom[addr + 1] as usize) << 8 | self.rom[addr] as usize
            }
        }
    }

    pub fn load_new_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        *self = Self::new(rom)?;
        Ok(())
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        let mut rom = vec![0; 0x150];
        rom[0x149] = 0x00; // rom only
        Self::new(&rom).unwrap() // empty header cartridge
    }
}
