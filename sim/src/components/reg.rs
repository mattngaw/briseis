use crate::isa::{
    WORD_WIDTH,
    INSTR_WIDTH,
    ADDR_WIDTH,
};

pub struct Reg<const W: usize>(u32);

impl<const W: usize> Reg<W> {
    const MASK: u32 = (1 << W) - 1;

    pub fn new() -> Self {
        Reg(0)
    }

    pub fn load(&self) -> u32 {
        self.0 & Self::MASK
    }

    pub fn store(&mut self, value: u32) {
        self.0 = value & Self::MASK;
    }
}

impl std::fmt::Display for Reg::<WORD_WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#03x}", self.0)
    }
}

impl std::fmt::Display for Reg::<INSTR_WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.0)
    }
}

impl std::fmt::Display for Reg::<ADDR_WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#05x}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load() {
        let mut reg = Reg::<4>::new();
        assert_eq!(reg.load(), 0);
        reg.0 = 0xf;
        assert_eq!(reg.load(), 0xf);
    }

    #[test]
    fn store() {
        let mut reg = Reg::<4>::new();
        reg.store(0xa);
        assert_eq!(reg.0, 0xa);
    }
    
    #[test]
    fn display() {
        let mut reg = Reg::<4>::new();
        reg.store(0xa);
        assert_eq!(&format!("{}", reg), "0xa");
    }

}