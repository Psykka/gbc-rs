#[cfg(test)]
mod tests {
    use core::cpu::{registers::WordReg, sm83::SM83};

    const ZERO: u8 = 0b1000_0000;
    // const SUB: u8 = 0b0100_0000;
    const HALF_CARRY: u8 = 0b0010_0000;
    // const CARRY: u8 = 0b0001_0000;

    fn create_rom(rom: Vec<u8>) -> Vec<u8> {
        let mut new_rom = vec![0; 0x100];
        new_rom.extend(rom);

        if new_rom.len() < 0x150 {
            new_rom.resize(0x150, 0);
        }

        new_rom
    }

    #[test]
    fn test_and_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;

        let rom = create_rom(vec![
            0xa0, // AND A, B
            0xa1, // AND A, C
            0xa7, // AND A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, HALF_CARRY);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | HALF_CARRY);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_and_hl() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0xa6, // AND A, (HL)
            0x00, // F = 0x00
            0x01, // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, HALF_CARRY);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_and_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xe6, // AND A, n
            0x01, // 0x100
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_or_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0xff;
        cpu.reg.h = 0x00;

        let rom = create_rom(vec![
            0xb0, // OR B
            0xb1, // OR C
            0xb2, // OR D
            0xb3, // OR E
            0xb4, // OR H
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xff);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);

        cpu.reg.a = 0x00;

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x105);
    }

    #[test]
    fn test_or_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xb6, // OR (HL)
            0x00, // F = 0x00
            0x01, // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_or_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xf6, // OR n
            0x01, // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_xor_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.e = 0xff;

        let rom = create_rom(vec![
            0xa8, // XOR A, B
            0xa9, // XOR A, C
            0xab, // XOR A, E
            0xaf, // XOR A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.reg.a = 0x01;

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_xor_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xae, // XOR A, (HL)
            0x00, // F = 0x00
            0x01, // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_xor_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xee, // XOR A, n
            0x01, // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);
    }
}
