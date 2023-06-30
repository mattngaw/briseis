// Architectural Constants
pub const NUM_GPRS: usize = 32;
pub const STACK_SIZE: usize = 4;


// Words
pub const WORD_WIDTH: usize = 4;
pub const WORD_MASK: u32 = (1 << WORD_WIDTH) - 1;

pub fn word_to_hex_string(word: u32) -> String {
    format!("{:#03x}", word)
}


// Instructions
pub const INSTR_WIDTH: usize = 8;
pub const INSTR_MASK: u32 = (1 << INSTR_WIDTH) - 1;

pub fn instr_to_hex_string(instr: u32) -> String {
    format!("{:#04x}", instr)
}


// Addresses
pub const ADDR_WIDTH: usize = 12;
pub const ADDR_MASK: u32 = (1 << ADDR_WIDTH) - 1;

pub fn addr_to_hex_string(addr: u32) -> String {
    format!("{:#05x}", addr)
}
