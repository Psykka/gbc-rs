#[cfg(test)]
mod tests {
    use core::cpu::{registers::WordReg, sm83::SM83};

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

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x07);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_adc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);

        let rom = create_rom(vec![
            0x8e, // ADC A, (HL)
            0x00, // A = 0x00
            0x01  // F = 0x01
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
            0x00, // A = 0x00
            0x01  // F = 0x01
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
}
