pub mod sm83;
pub mod bus;
pub mod memory;
pub mod registers;
pub mod types;

pub use sm83::SM83;

pub struct Core {
    pub sm83: SM83,
}

impl Core {
    pub fn new() -> Self {
        Self {
            sm83: SM83::new(),
        }
    }

    pub fn step(&mut self) {
        self.sm83.step();
    }
}