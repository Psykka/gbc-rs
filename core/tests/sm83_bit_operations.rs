#[cfg(test)]
mod tests {
    use core::{types::Size, cpu::{sm83::SM83, registers::WordReg}};


    const ZERO: u8 = 0b1000_0000;
    // const SUB: u8 = 0b0100_0000;
    const HALF_CARRY: u8 = 0b0010_0000;
    // const CARRY: u8 = 0b0001_0000;

    const WRAM_00: usize = 0xC000;

    fn create_rom(rom: Vec<u8>) -> Vec<u8> {
        let mut new_rom = vec![0; 0x100];
        new_rom.extend(rom);

        if new_rom.len() < 0x150 {
            new_rom.resize(0x150, 0);
        }

        new_rom
    }

    #[test]
    fn test_bit_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x04;
        cpu.reg.e = 0x08;
        cpu.reg.h = 0x10;
        cpu.reg.l = 0x20;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x47, // BIT 0, A
            0xcb, // PREFIX
            0x48, // BIT 1, B
            0xcb, // PREFIX
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x01);
        assert_eq!(cpu.reg.f, ZERO | HALF_CARRY);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_bit_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x46, // BIT 0, (HL)
            0xcb, // PREFIX
            0x4e, // BIT 1, (HL)
            0xcb, // PREFIX
            0x56, // BIT 2, (HL)
            0xcb, // PREFIX
            0x5e, // BIT 3, (HL)
            0xcb, // PREFIX
            0x66, // BIT 4, (HL)
            0xcb, // PREFIX
            0x6e, // BIT 5, (HL)
            0xcb, // PREFIX
            0x76, // BIT 6, (HL)
            0xcb, // PREFIX
            0x7e, // BIT 7, (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0xff);

        cpu.step();

        assert_eq!(cpu.reg.f, HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x00);

        cpu.step();

        assert_eq!(cpu.reg.f, ZERO | HALF_CARRY);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_res_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0xff;
        cpu.reg.b = 0xff;
        cpu.reg.c = 0xff;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x87, // RES 0, A
            0xcb, // PREFIX
            0x88, // RES 1, B
            0xcb, // PREFIX
            0x91, // RES 2, C
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0xfe);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0xfd);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0xfb);
        assert_eq!(cpu.pc, 0x106);
    }

    #[test]
    fn test_res_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x86, // RES 0, (HL)
            0xcb, // PREFIX
            0x8e, // RES 1, (HL)
            0xcb, // PREFIX
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0xff);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0xfe);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0xff);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0xfd);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_set_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x00;
        cpu.reg.b = 0x00;
        cpu.reg.c = 0x00;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0xc7, // SET 0, A
            0xcb, // PREFIX
            0xc8, // SET 1, B
            0xcb, // PREFIX
            0xd1, // SET 2, C
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x04);
        assert_eq!(cpu.pc, 0x106);
    }

    #[test]
    fn test_set_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0xc6, // SET 0, (HL)
            0xcb, // PREFIX
            0xce, // SET 1, (HL)
            0xcb, // PREFIX
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x00);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x01);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x00);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x02);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_swap_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x12;
        cpu.reg.b = 0x34;
        cpu.reg.c = 0x56;
        cpu.reg.d = 0x00;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x37, // SWAP A
            0xcb, // PREFIX
            0x30, // SWAP B
            0xcb, // PREFIX
            0x31, // SWAP C
            0xcb, // PREFIX
            0x32, // SWAP D
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x21);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x43);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x65);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);
    }

    #[test]
    fn test_swap_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x36, // SWAP (HL)
            0xcb, // PREFIX
            0x36, // SWAP (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x12);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x21);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x00);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);
    }
}
