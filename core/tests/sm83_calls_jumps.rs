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
    fn test_call_nn() {
        let mut cpu = SM83::new();
        cpu.reg.sp = 0xFFFE;

        let rom = create_rom(vec![
            0xcd, // CALL nn
            0x00,
            0x01,
        ]);

        cpu.bus.rom.load_new_rom(&rom).unwrap();

        cpu.step();

        assert_eq!(cpu.reg.sp, 0xFFFC);
        assert_eq!(cpu.pc, 0x100);
    }
}
