use crate::isa::{
    ROM_SIZE,
    INSTR_MASK
};

pub struct Rom {
    data: [u32; ROM_SIZE],
    addr: u32,
}

impl Rom {
    pub fn load(&self, addr: u32) -> u32 {
        assert
    }

    pub fn store(&mut self, addr: u32, value: u32) {

    }
}
