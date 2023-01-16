#[cfg(test)]
mod tests {
    use core::{sm83::SM83, types::Size, registers::WordReg};

    #[test]
    fn test_adc_r() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x88);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x89);
        cpu.bus.mem.write(Size::Byte, 0x101, 0x8f);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x07);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_adc_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x8e);
        cpu.bus.mem.write(Size::Byte, 0x102, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);
    }

    #[test]
    fn test_adc_n() {
        let mut cpu = SM83::new();
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0xce);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_add_r() {
        let mut cpu = SM83::new();
        cpu.reg.b = 0x01;
        cpu.reg.c = 0x02;
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x80);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x81);
        cpu.bus.mem.write(Size::Byte, 0x101, 0x87);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x03);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x06);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::HL, 0x102);
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x86);
        cpu.bus.mem.write(Size::Byte, 0x102, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);
    }

    #[test]
    fn test_add_n() {
        let mut cpu = SM83::new();
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0xc6);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }

    #[test]
    fn test_add_hl_rr() {
        let mut cpu = SM83::new();
        cpu.reg.set_word(WordReg::BC, 0x01);
        cpu.reg.set_word(WordReg::DE, 0x02);
        cpu.reg.set_word(WordReg::HL, 0x03);
        cpu.reg.set_word(WordReg::SP, 0x04);
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0x09);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x19);
        cpu.bus.mem.write(Size::Byte, 0x101, 0x29);
        cpu.bus.mem.write(Size::Byte, 0x102, 0x39);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x04);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x100);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x06);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0xc);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x102);

        cpu.step();

        assert_eq!(cpu.reg.get_word(WordReg::HL), 0x10);
        assert_eq!(cpu.reg.f, 0x20);
        assert_eq!(cpu.pc, 0x103);
    }

    #[test]
    fn test_add_sp_n() {
        let mut cpu = SM83::new();
        cpu.reg.sp = 0x01;
        cpu.pc = 0xff;

        cpu.bus.mem.write(Size::Byte, 0xff, 0xe8);
        cpu.bus.mem.write(Size::Byte, 0x100, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.sp, 0x02);
        assert_eq!(cpu.reg.f, 0x00);
        assert_eq!(cpu.pc, 0x101);
    }
}