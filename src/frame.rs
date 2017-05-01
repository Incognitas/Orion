use stack::{Stack, StackEntry, StackEntryType};
use jcvmerrors::InterpreterError;

// structure representing one frame
pub struct Frame {
    locals_stack: Stack,
}

// methods/functions associated to the Frame struct
impl Frame {
    pub fn new() -> Frame {
        Frame { locals_stack: Stack::new(255) }
    }

    pub fn get_local(&self, index: u8, type_: StackEntryType) -> Result<StackEntry, InterpreterError> {
        self.locals_stack.peek_index(index)
    }

    pub fn get_local_check_type(&self, index: u8, type_: StackEntryType) -> Result<StackEntry, InterpreterError> {
        self.locals_stack.peek_index_check_type(index, type_)
    }
}