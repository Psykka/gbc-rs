pub struct Registers {
    pub a: u8,
    pub c: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            c: 0
        }
    }
}