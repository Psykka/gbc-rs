#[cfg(test)]
mod tests {
    use core::registers::{Registers, ByteReg, WordReg};

    #[test]
    fn test_byte_register() {
        let mut regs = Registers::new();
        regs.set_byte(ByteReg::A, 0x01);
        assert_eq!(regs.get_byte(ByteReg::A), 0x01);
    }

    #[test]
    fn test_word_register() {
        let mut regs = Registers::new();
        regs.set_word(WordReg::AF, 0x0102);
        assert_eq!(regs.get_word(WordReg::AF), 0x0102);
    }

    #[test]
    fn test_check_zero() {
        let mut regs = Registers::new();

        regs.check_zero(0);
        assert_eq!(regs.f, 0x80);

        regs.check_zero(1);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_subtract() {
        let mut regs = Registers::new();

        regs.subtract(true);
        assert_eq!(regs.f, 0x40);

        regs.subtract(false);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_half_carry() {
        let mut regs = Registers::new();

        regs.check_half_carry(0x10);
        assert_eq!(regs.f, 0x20);

        regs.check_half_carry(0x0f);
        assert_eq!(regs.f, 0);
    }

    #[test]
    fn test_check_carry() {
        let mut regs = Registers::new();

        regs.check_carry(0x100);
        assert_eq!(regs.f, 0x10);

        regs.check_carry(0xff);
        assert_eq!(regs.f, 0);
    }
}