use crate::{
    isa::{
        STACK_SIZE,
        ADDR_MASK,
        addr_to_hex_string, 
    }
};

pub struct Stack {
    data: [u32; STACK_SIZE],
    len: usize,
    ptr: usize,
}

impl Stack {
    const CAPACITY: usize = STACK_SIZE - 1;

    pub fn new() -> Self {
        Stack {
            data: [0; STACK_SIZE],
            len: 0,
            ptr: 0
        }
    }

    pub fn get_pc(&self) -> u32 {
        self.data[self.ptr] & ADDR_MASK
    }

    pub fn set_pc(&mut self, addr: u32) {
        self.data[self.ptr] = addr & ADDR_MASK;
    }

    pub fn push(&mut self, addr: u32) -> bool {
        let addr = addr & ADDR_MASK;
        self.data[(self.ptr + 1) % STACK_SIZE] = self.data[self.ptr];
        self.data[self.ptr] = addr;
        self.ptr = (self.ptr + 1) % STACK_SIZE;
        let no_overwrite = self.len < Self::CAPACITY;
        if no_overwrite {
            self.len += 1;
        }
        no_overwrite
    }

    pub fn pop(&mut self) -> Result<u32, ()> {
        if self.len == 0 {
            return Err(())
        }
        let addr = self.data[(self.ptr - 1) % STACK_SIZE];
        self.data[(self.ptr - 1) % STACK_SIZE] = self.data[self.ptr];
        self.ptr = self.ptr - 1 % STACK_SIZE;
        self.len -= 1;
        Ok(addr & ADDR_MASK)
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        display.push_str("     STACK  \n");
        display.push_str("   +-------+\n");
        for (i, addr) in self.data.into_iter().rev().enumerate() {
            let index = Self::CAPACITY - i;
            display.push_str({
                if index == self.ptr { "PC " }
                else { "   " }
            });
            display.push_str(&format!("| {} |\n", addr_to_hex_string(addr)));
            display.push_str("   +-------+\n");
        }
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        assert!(stack.push(0x123));
        assert!(stack.push(0x456));
        assert!(stack.push(0x789));
        assert!(!stack.push(0x012));
        assert_eq!(stack.data[0], 0x000);
        assert_eq!(stack.data[1], 0x456);
        assert_eq!(stack.data[2], 0x789);
        assert_eq!(stack.data[3], 0x012);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        stack.push(0x123);
        stack.push(0x456);
        stack.push(0x789);
        assert_eq!(stack.pop(), Ok(0x789));
        assert_eq!(stack.pop(), Ok(0x456));
        assert_eq!(stack.pop(), Ok(0x123));
        assert_eq!(stack.pop(), Err(()));
    }

    #[test]
    fn pc() {
        let mut stack = Stack::new();
        assert_eq!(stack.get_pc(), 0x000);
        stack.push(0x123);
        assert_eq!(stack.get_pc(), 0x000);
        stack.set_pc(0x999);
        assert_eq!(stack.get_pc(), 0x999);
        assert_eq!(stack.pop(), Ok(0x123));
        assert_eq!(stack.get_pc(), 0x999);
    }
    
    #[test]
    fn display() {
        let mut stack = Stack::new();
        assert_eq!(
            &format!("{}", stack),
            "     STACK  \n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n\
            PC | 0x000 |\n   \
               +-------+\n\
            "
        );

        stack.push(0x123);
        assert_eq!(
            &format!("{}", stack),
            "     STACK  \n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n\
            PC | 0x000 |\n   \
               +-------+\n   \
               | 0x123 |\n   \
               +-------+\n\
            "
        );
        
        stack.set_pc(0x999);
        assert_eq!(
            &format!("{}", stack),
            "     STACK  \n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n   \
               | 0x000 |\n   \
               +-------+\n\
            PC | 0x999 |\n   \
               +-------+\n   \
               | 0x123 |\n   \
               +-------+\n\
            "
        );

    }

}