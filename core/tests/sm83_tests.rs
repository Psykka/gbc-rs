#[cfg(test)]
mod tests {
    use core::cpu::{registers::WordReg, sm83::SM83};

    const ZERO: u8 = 0b1000_0000;
    const SUB: u8 = 0b0100_0000;
    const HALF_CARRY: u8 = 0b0010_0000;
    const CARRY: u8 = 0b0001_0000;

    fn create_rom(rom: Vec<u8>) -> Vec<u8> {
        let mut new_rom = vec![0; 0x100];
        new_rom.extend(rom);

        if new_rom.len() < 0x150 {
            new_rom.resize(0x150, 0);
        }

        new_rom
    }

    #[test]
    fn test_adc_r() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;

        let rom = create_rom(vec![
            0x88, // ADC A, B
            0x89, // ADC A, C
            0x8f  // ADC A, A
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

        assert_eq!(cpu.reg.a, 0x06);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_adc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0x8e, // ADC A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_adc_n() {
        let mut cpu = SM83::new();

        let rom = create_rom(vec![
            0xce, // ADC A, n
            0x01  // 0x100
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_add_r() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;

        let rom = create_rom(vec![
            0x80, // ADD A, B
            0x81, // ADD A, C
            0x87  // ADD A, A
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

        assert_eq!(cpu.reg.a, 0x06);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0x86, // ADD A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_add_n() {
        let mut cpu = SM83::new();

        let rom = create_rom(vec![
            0xc6, // ADD A, n
            0x01  // 0x100
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_add_hl_rr() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::BC, 0x01);
        cpu.reg.set_word(WordReg::DE, 0x02);
        cpu.reg.set_word(WordReg::HL, 0x03);
        cpu.reg.set_word(WordReg::SP, 0x04);

        let rom = create_rom(vec![
            0x09, // ADD HL, BC
            0x19, // ADD HL, DE
            0x29, // ADD HL, HL
            0x39  // ADD HL, SP
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x04);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x06);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0xc);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x10);
        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_add_sp_n() {
        let mut cpu = SM83::new();
        cpu.reg.sp = 0x01;

        let rom = create_rom(vec![
            0xe8, // ADD SP, n
            0x01  // 0x100
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.sp, 0x02);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
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
            0xa7  // AND A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
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
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_and_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xe6, // AND A, n
            0x01  // 0x100
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_cp_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;

        let rom = create_rom(vec![
            0xb8, // CP A, B
            0xb9, // CP A, C
            0xbf  // CP A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO | SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_cp_hl() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0xbe, // CP A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_cp_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xfe, // CP A, n
            0x01  // 0x101
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_dec_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;

        let rom = create_rom(vec![
            0x3d, // DEC A
            0x05, // DEC B
            0x0d, // DEC C
            0x15, // DEC D
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x01);
        assert_eq!(cpu.reg.f, SUB);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.d, 0xff);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0x35, // DEC (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_dec_rr() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::BC, 0x102);
        cpu.reg.set_word(WordReg::DE, 0x102);
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.set_word(WordReg::SP, 0x100);

        let rom = create_rom(vec![
            0x0b, // DEC BC
            0x1b, // DEC DE
            0x2b, // DEC HL
            0x3b  // DEC SP
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::BC), 0x101);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::DE), 0x101);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x101);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::SP), 0xff);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn test_inc_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.d = 0x00;
        cpu.reg.e = 0xff;

        let rom = create_rom(vec![
            0x3c, // INC A
            0x04, // INC B
            0x0c, // INC C
            0x14, // INC D
            0x1c, // INC E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.c, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.d, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);

        cpu.step();

        assert_eq!(cpu.reg.e, 0x00);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x105);
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0x34, // INC (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_inc_rr() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::BC, 0x102);
        cpu.reg.set_word(WordReg::DE, 0x102);
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.set_word(WordReg::SP, 0xff);

        let rom = create_rom(vec![
            0x03, // INC BC
            0x13, // INC DE
            0x23, // INC HL
            0x33  // INC SP
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::BC), 0x103);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::DE), 0x103);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x103);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x103);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::SP), 0x100);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x104);
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
            0x01  // A = 0x01
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
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_sbc_r_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.f = 0x10;

        let rom = create_rom(vec![
            0x9f, // SBC A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, SUB | CARRY);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_sbc_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.e = 0xff;


        let rom = create_rom(vec![
            0x98, // SBC A, B
            0x99, // SBC A, C
            0x9b, // SBC A, E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xfe);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xff);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_sbc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.a = 0x01;
        cpu.reg.f = 0x10;

        let rom = create_rom(vec![
            0x9e, // SBC A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, SUB);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_sbc_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.f = 0x10;

        let rom = create_rom(vec![
            0xde, // SBC A, n
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, SUB);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_sub_r_a() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0x97, // SUB A, A
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_sub_r() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.reg.e = 0xff;

        let rom = create_rom(vec![
            0x90, // SUB A, B
            0x91, // SUB A, C
            0x93, // SUB A, E
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xfe);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xff);
        assert_eq!(cpu.reg.f, SUB | HALF_CARRY);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_sub_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0x96, // SUB A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_sub_n() {
        let mut cpu = SM83::new();
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xd6, // SUB A, n
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, ZERO | SUB);
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
    }

    #[test]
    fn test_xor_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.reg.a = 0x01;

        let rom = create_rom(vec![
            0xae, // XOR A, (HL)
            0x00, // F = 0x00
            0x01  // A = 0x01
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
            0x01  // A = 0x01
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, ZERO);
        assert_eq!(cpu.pc, 0x102);
    }
}
