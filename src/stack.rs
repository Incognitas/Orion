use std::vec;

#[derive(Debug)]
pub enum StackError {
    StackOverflowError,
    StackUnderflowError
}


pub struct Stack{
    internal_stack: Vec<i16>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { internal_stack: Vec::new() }
    }

    pub fn push(&mut self, value: i16) {
        self.internal_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<i16, StackError> {
        self.internal_stack.pop().ok_or(StackError::StackUnderflowError)
    }

    pub fn peek(&self) -> Result<i16, StackError> {
        self.peekIndex(0)
    }

    pub fn peekIndex(&self, index: usize) -> Result<i16, StackError> {
        if self.internal_stack.len() > index {
            Ok(self.internal_stack[self.internal_stack.len() - 1 - index])
        }
        else {
            Err(StackError::StackUnderflowError)
        }
    }
}

