#[cfg(test)]
mod tests {
    use core::bus::Bus;
    use core::cartridge::Cartridge;
    use core::types::Size;

    fn create_fake_cartridge(ram_size: Option<u8>) -> Cartridge {
        let mut rom = vec![0; 0x150];

        // Title
        let title = "TEST ROM";
        for (i, byte) in title.bytes().enumerate() {
            rom[i + 0x134] = byte;
        }

        // Cartridge type
        rom[0x147] = 0x00;

        // RAM size
        rom[0x149] = ram_size.unwrap_or(0x01);

        // ROM size
        rom[0x148] = 0x01;

        let cart = Cartridge::new(&rom);

        cart.unwrap()
    }

    #[test]
    fn test_hram_00() {
        let mut bus = Bus::default();
        bus.write(Size::Byte, 0xC000, 0x01);

        assert_eq!(bus.mem.read(Size::Byte, 0x0000), 0x01);
        assert_eq!(bus.read(Size::Byte, 0xC000), 0x01);

        bus.write(Size::Byte, 0xC001, 0x02);

        assert_eq!(bus.mem.read(Size::Byte, 0x0001), 0x02);
        assert_eq!(bus.read(Size::Byte, 0xC001), 0x02);

        bus.write(Size::Byte, 0xCFFF, 0x03);

        assert_eq!(bus.mem.read(Size::Byte, 0x0FFF), 0x03);
        assert_eq!(bus.read(Size::Byte, 0xCFFF), 0x03);
    }

    #[test]
    fn test_hram_01() {
        let cart = create_fake_cartridge(None); // 2KB RAM
        let mut bus = Bus::new(Some(cart));

        bus.write(Size::Byte, 0xD000, 0x01);

        assert_eq!(bus.rom.ram.read(Size::Byte, 0x0000), 0x01);
        assert_eq!(bus.read(Size::Byte, 0xD000), 0x01);

        bus.write(Size::Byte, 0xD001, 0x02);

        assert_eq!(bus.rom.ram.read(Size::Byte, 0x0001), 0x02);
        assert_eq!(bus.read(Size::Byte, 0xD001), 0x02);
    }

    #[test]
    fn test_rom_00() {
        let cart = create_fake_cartridge(None); // 2KB RAM
        let bus = Bus::new(Some(cart));

        assert_eq!(bus.read(Size::Byte, 0x148), 0x01); // ROM size
        assert_eq!(bus.read(Size::Byte, 0x149), 0x01); // RAM size
        assert_eq!(bus.read(Size::Byte, 0x134), 0x54); // T from header title
    }
}
