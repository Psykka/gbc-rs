#[cfg(test)]
mod tests {
    use core::cartridge::Cartridge;

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

        let cart = Cartridge::new(&rom);

        cart.unwrap()
    }

    #[test]
    fn test_cartridge_title() {
        let cart = create_fake_cartridge(None);
        assert_eq!(cart.title, "TEST ROM");
    }

    #[test]
    fn test_cartridge_type() {
        let cart = create_fake_cartridge(None);
        assert_eq!(cart.cartridge_type, 0x00);
    }

    #[test]
    fn test_cartridge_ram_size() {
        let cart = create_fake_cartridge(None);
        assert_eq!(cart.ram_size, 0x01);
        assert_eq!(cart.ram.size(), 2 * 1024);
    }
}
