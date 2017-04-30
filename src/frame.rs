use stack::{Stack, StackEntryType, StackError};

pub struct Frame {
    pub locals_stack: Stack
}


impl Frame {
    pub fn new() -> Frame {
        Frame { locals_stack: Stack::new()}
    }

    pub fn getLocal(&self, index: u8) -> Result<StackEntryType, StackError> {
        self.locals_stack.peekIndex(index)
    }
}