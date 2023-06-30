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
        display.push_str("           REGISTERS\n");
        display.push_str("         +-----+-----+\n");
        for i in (0..NUM_GPRS/2).step_by(2) {
            if i < 10 { display.push_str(&format!("r{},  r{}  ", i, i+1)); } 
            else { display.push_str(&format!("r{}, r{} ", i, i+1)); }
            display.push_str(&format!("| {:#03x} | {:#03x} |\n", self.0[i].load(), self.0[i+1].load()));
            display.push_str("         +-----+-----+\n");
        }
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod tests {
   // TODO 
}