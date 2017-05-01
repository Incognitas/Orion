use stack::{Stack, StackEntryType};
use jcvmerrors::InterpreterError;

pub struct Frame {
    pub locals_stack: Stack,
}


impl Frame {
    pub fn new() -> Frame {
        Frame { locals_stack: Stack::new(255) }
    }

    pub fn getLocal(&self, index: u8) -> Result<StackEntryType, InterpreterError> {
        self.locals_stack.peekIndex(index)
    }
}