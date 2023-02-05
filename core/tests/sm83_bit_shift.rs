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

    #[test]
    fn test_rlc_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x80;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x07, // RLC A
            0xcb, // PREFIX
            0x00, // RLC B
            0xcb, // PREFIX
            0x01, // RLC C
            0xcb, // PREFIX
            0x02, // RLC D
            0xcb, // PREFIX
            0x03, // RLC E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x04);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.step();

        assert_eq!(cpu.reg.e, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x10a);
    }

    #[test]
    fn test_rr_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x01;
        cpu.reg.e = 0x01;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x1f, // RR A
            0xcb, // PREFIX
            0x18, // RR B
            0xcb, // PREFIX
            0x19, // RR C
            0xcb, // PREFIX
            0x1a, // RR D
            0xcb, // PREFIX
            0x1b, // RR E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();


        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.reg.set_flags(CARRY);
        cpu.step();

        assert_eq!(cpu.reg.e, 0x80);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10a);
    }

    #[test]
    fn test_rr_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x1e, // RR (HL)
            0xcb, // PREFIX
            0x1e, // RR (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);
        cpu.reg.set_flags(CARRY);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x80);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_rrc_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x80;
        cpu.reg.h = 0xff;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x0f, // RRC A
            0xcb, // PREFIX
            0x08, // RRC B
            0xcb, // PREFIX
            0x09, // RRC C
            0xcb, // PREFIX
            0x0a, // RRC D
            0xcb, // PREFIX
            0x0b, // RRC E
            0xcb, // PREFIX
            0x0c, // RRC H
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.step();

        assert_eq!(cpu.reg.e, 0x40);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10a);

        cpu.step();

        assert_eq!(cpu.reg.h, 0x7f);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10c);
    }

    #[test]
    fn test_rrc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x0e, // RRC (HL)
            0xcb, // PREFIX
            0x0e, // RRC (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x40);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_sla_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x80;
        cpu.reg.h = 0xff;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x27, // SLA A
            0xcb, // PREFIX
            0x20, // SLA B
            0xcb, // PREFIX
            0x21, // SLA C
            0xcb, // PREFIX
            0x22, // SLA D
            0xcb, // PREFIX
            0x23, // SLA E
            0xcb, // PREFIX
            0x24, // SLA H
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x04);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.step();

        assert_eq!(cpu.reg.e, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x10a);

        cpu.step();

        assert_eq!(cpu.reg.h, 0xfe);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10c);
    }

    #[test]
    fn test_sla_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x26, // SLA (HL)
            0xcb, // PREFIX
            0x26, // SLA (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x02);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_sra_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x80;
        cpu.reg.h = 0xff;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x2f, // SRA A
            0xcb, // PREFIX
            0x28, // SRA B
            0xcb, // PREFIX
            0x29, // SRA C
            0xcb, // PREFIX
            0x2a, // SRA D
            0xcb, // PREFIX
            0x2b, // SRA E
            0xcb, // PREFIX
            0x2c, // SRA H
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.step();

        assert_eq!(cpu.reg.e, 0xc0);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10a);

        cpu.step();

        assert_eq!(cpu.reg.h, 0xff);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10c);
    }

    #[test]
    fn test_sra_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x2e, // SRA (HL)
            0xcb, // PREFIX
            0x2e, // SRA (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0xc0);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_srl_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0x80;
        cpu.reg.h = 0xff;

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x3f, // SRL A
            0xcb, // PREFIX
            0x38, // SRL B
            0xcb, // PREFIX
            0x39, // SRL C
            0xcb, // PREFIX
            0x3a, // SRL D
            0xcb, // PREFIX
            0x3b, // SRL E
            0xcb, // PREFIX
            0x3c, // SRL H
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.f, 0);
        assert_eq!(cpu.pc, 0x106);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x108);

        cpu.step();

        assert_eq!(cpu.reg.e, 0x40);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10a);

        cpu.step();

        assert_eq!(cpu.reg.h, 0x7f);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x10c);
    }

    #[test]
    fn test_srl_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, WRAM_00 as u16 + 0x01);

        let rom = create_rom(vec![
            0xcb, // PREFIX
            0x3e, // SRL (HL)
            0xcb, // PREFIX
            0x3e, // SRL (HL)
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x01);

        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.bus.write(Size::Byte, WRAM_00 + 0x01, 0x80);
        cpu.step();

        assert_eq!(cpu.bus.read(Size::Byte, WRAM_00 + 0x01), 0x40);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }
}
