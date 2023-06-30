use crate::{
    isa::{
        WORD_WIDTH,
    },
    components::{
        rom::Rom,
        ram::Ram,
        reg::Reg,
        regfile::RegFile,
        stack::Stack,
    }
};

pub struct Core {
    regfile: RegFile,
    accum: Reg<WORD_WIDTH>,
    stack: Stack,
    carry: bool,
    ram: Ram,
    rom: Rom,
}