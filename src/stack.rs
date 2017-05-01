use jcvmerrors::InterpreterError;

pub type StackEntryType = i16;

// basic implementation of a stack
pub struct Stack {
    internal_stack: Vec<StackEntryType>,
}

// associated methods/functions
impl Stack {
    pub fn new(capacity: usize) -> Stack {
        Stack { internal_stack: Vec::with_capacity(capacity) }
    }

    // push a value on the stack
    pub fn push(&mut self, value: StackEntryType) -> Result<(), InterpreterError> {
        if self.internal_stack.len() < self.internal_stack.capacity() {
            self.internal_stack.push(value);
            return Ok(());
        }
        Err(InterpreterError::StackOverflowError)
    }

    // removes top item and returns its value
    pub fn pop(&mut self) -> Result<StackEntryType, InterpreterError> {
        self.internal_stack
            .pop()
            .ok_or(InterpreterError::StackUnderflowError)
    }

    pub fn peek(&self) -> Result<StackEntryType, InterpreterError> {
        self.peekIndex(0)
    }

    pub fn peekIndex(&self, index: u8) -> Result<StackEntryType, InterpreterError> {
        if self.internal_stack.len() > index as usize {
            Ok(self.internal_stack[self.internal_stack.len() - 1 - index as usize])
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }
}
