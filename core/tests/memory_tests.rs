#[cfg(test)]
mod tests {
    use core::{memory::Memory, types::Size};

    #[test]
    fn test_byte() {
        let mut mem = Memory::default();
        mem.write(Size::Byte, 0x00, 0x01);
        assert_eq!(mem.read(Size::Byte, 0x00), 0x01);
    }

    #[test]
    fn test_word() {
        let mut mem = Memory::default();
        mem.write(Size::Word, 0x00, 0x0102);
        assert_eq!(mem.read(Size::Word, 0x00), 0x0102);
    }
}
