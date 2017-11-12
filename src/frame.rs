use stack::{Stack, StackEntry, StackEntryType};
use jcvmerrors::InterpreterError;

// structure representing one frame
pub struct Frame {
    locals_stack: Stack,
}

// methods/functions associated to the Frame struct
impl Frame {
    pub fn new(max_stack: u8) -> Frame {
        let mut result = Frame {
            locals_stack: Stack::new(max_stack as usize),
        };

        {
            // initialize the locals stack with StackEntry so that we can
            // access and modify them later            
            let to_be_initialised = &mut result;
            for _ in 0..max_stack {
                to_be_initialised.locals_stack.push(StackEntry::new()).unwrap();
            }
        }

        result
    }

    pub fn get_local(&self, index: i16) -> Result<StackEntry, InterpreterError> {
        self.locals_stack.peek_index(index)
    }

    pub fn get_local_check_type(
        &self,
        index: i16,
        type_: StackEntryType,
    ) -> Result<StackEntry, InterpreterError> {
        self.locals_stack.peek_index_check_type(index, type_)
    }

    pub fn set_local(&mut self, index: i16, entry: &StackEntry) -> Result<(), InterpreterError> {
        self.locals_stack.update_index(index, &entry)
    }
}
