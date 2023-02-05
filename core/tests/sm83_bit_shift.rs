#[cfg(test)]
mod tests {
    use core::{types::Size, cpu::{sm83::SM83, registers::WordReg}};

    const ZERO: u8 = 0b1000_0000;
    // const SUB: u8 = 0b0100_0000;
    // const HALF_CARRY: u8 = 0b0010_0000;
    const CARRY: u8 = 0b0001_0000;

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
    fn test_rl_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x80;
        cpu.reg.e = 0x80;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x10, // RL B
            0xcb, // PREFIX
            0x11, // RL C
            0xcb, // PREFIX
            0x17, // RL A
            0xcb, // PREFIX
            0x12, // RL D
            0xcb, // PREFIX
            0x13, // RL E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x04);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.reg.set_flags(CARRY);
        cpu.step();

        assert_eq!(cpu.reg.e, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10a);
    }

    #[test]
    fn test_rl_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x16, // RL (HL)
            0xcb, // PREFIX
            0x16, // RL (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);
        cpu.reg.set_flags(CARRY);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }
}
