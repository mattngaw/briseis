use crate::{
    isa::{
        NUM_GPRS,
        WORD_WIDTH
    },
    components::reg::Reg
};

pub struct RegFile([Reg<WORD_WIDTH>; NUM_GPRS]);

impl RegFile {
    pub fn new() -> Self {
        RegFile(
            core::array::from_fn(
                |_| Reg::<WORD_WIDTH>::new()
            )
        )
    }

    pub fn load(&self, index: usize) -> u32 {
        debug_assert!(index < NUM_GPRS);
        self.0[index].load()
    }

    pub fn store(&mut self, index: usize, value: u32) {
        debug_assert!(index < NUM_GPRS);
        self.0[index].store(value);
    }
}

impl std::fmt::Display for RegFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        display.push_str("  REGISTERS\n");
        display.push_str("+-----+-----+\n");
        for i in (0..NUM_GPRS/2).step_by(2) {
            display.push_str(&format!("| {} | {} |", self.0[i], self.0[i+1]));
            if i < 10 { display.push_str(&format!(" r{},  r{}\n", i, i+1)); } 
            else { display.push_str(&format!(" r{}, r{}\n", i, i+1)); }
            display.push_str("+-----+-----+\n");
        }
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load() {
        let mut rf = RegFile::new();
        assert_eq!(rf.load(0), 0x0);
        rf.0[0].store(0xf);
        assert_eq!(rf.load(0), 0xf);
        rf.0[15].store(0x3);
        assert_eq!(rf.load(15), 0x3);
    }

    #[test]
    fn store() {
        let mut rf = RegFile::new();
        rf.store(0, 0xf);
        assert_eq!(rf.load(0), 0xf);
    }

    #[test]
    fn display() {
        let mut rf = RegFile::new();
        assert_eq!(
            &format!("{}", rf), 
            "  REGISTERS\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r0,  r1\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r2,  r3\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r4,  r5\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r6,  r7\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r8,  r9\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r10, r11\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r12, r13\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r14, r15\n\
             +-----+-----+\n\
            "
        );

        rf.store(15, 0xf);
        assert_eq!(
            &format!("{}", rf), 
            "  REGISTERS\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r0,  r1\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r2,  r3\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r4,  r5\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r6,  r7\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r8,  r9\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r10, r11\n\
             +-----+-----+\n\
             | 0x0 | 0x0 | r12, r13\n\
             +-----+-----+\n\
             | 0x0 | 0xf | r14, r15\n\
             +-----+-----+\n\
            "
        );

    }
}