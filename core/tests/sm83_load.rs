#[cfg(test)]
mod tests {
    use core::{
        cpu::{registers::WordReg, sm83::SM83},
        types::Size,
    };

    // const ZERO: u8 = 0b1000_0000;
    // const SUB: u8 = 0b0100_0000;
    // const HALF_CARRY: u8 = 0b0010_0000;
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
    fn test_ld_r_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x02;

        let rom = create_rom(vec![
            0x7f, // LD A, A
            0x78, // LD A, B
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_ld_r_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0x3e, // LD A, n
            0x02,
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_ld_r_hl() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x7e, // LD A, (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x02);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_hl_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x77, // LD (HL), A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x01);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_hl_n() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x36, // LD (HL), n
            0x02,
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x02);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_ld_rr_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_word(WordReg::BC, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x02, // LD A, (BC)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x02);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_a_rr() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x62;
        cpu.reg.set_word(WordReg::BC, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x0a, // LD (BC), A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x62);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_a_nn() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x00;

        let rom = create_rom(vec![
            0xfa, // LD (nn), A
            0x62,
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x62);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_ld_nn_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x62;

        cpu.pc = WRAM_00 as u16;

        cpu.bus.write(Size::Word, WRAM_00, 0xea); // LD A, (nn)

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x62);
        assert_eq!(cpu.pc, WRAM_00 as u16 + 0x03);
    }

    #[test]
    fn test_ld_a_hl() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x00;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x2a, // LD A, (HL+)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x62);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x62);
        assert_eq!(cpu.reg.get_word(WordReg::HL), WRAM_00 as u16 + 0x02);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_hl_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x62;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x22, // LD (HL+), A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x62);
        assert_eq!(cpu.reg.get_word(WordReg::HL), WRAM_00 as u16 + 0x02);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_a_hl_2() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x00;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x3a, // LD A, (HL-)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x62);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x62);
        assert_eq!(cpu.reg.get_word(WordReg::HL), WRAM_00 as u16);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_ld_hl_a_2() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x62;
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0x32, // LD (HL-), A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x62);
        assert_eq!(cpu.reg.get_word(WordReg::HL), WRAM_00 as u16);
        assert_eq!(cpu.pc, 0x101);
    }
}
