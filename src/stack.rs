use std::vec;

#[derive(Debug)]
pub enum StackError {
    StackOverflowError,
    StackUnderflowError
}

pub type StackEntryType = i16;

pub struct Stack{
    internal_stack: Vec<StackEntryType>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { internal_stack: Vec::new() }
    }

    pub fn push(&mut self, value: StackEntryType) {
        self.internal_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<StackEntryType, StackError> {
        self.internal_stack.pop().ok_or(StackError::StackUnderflowError)
    }

    pub fn peek(&self) -> Result<StackEntryType, StackError> {
        self.peekIndex(0)
    }

    pub fn peekIndex(&self, index: u8) -> Result<StackEntryType, StackError> {
        if self.internal_stack.len() > index as usize {
            Ok(self.internal_stack[self.internal_stack.len() - 1 - index as usize])
        }
        else {
            Err(StackError::StackUnderflowError)
        }
    }
}

