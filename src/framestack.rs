use frame::Frame;
use stack::StackError;
use std::vec;

pub struct FrameStack {
    internal_stack: Vec<Frame>
}

impl FrameStack {
    pub fn new() -> FrameStack {
        FrameStack {internal_stack: Vec::new()}
    }

    pub fn newEntry(&mut self) -> &Frame {
        self.internal_stack.push(Frame::new());
        &self.internal_stack[self.internal_stack.len() - 1]
    }

    pub fn top(&self) -> Result<&Frame, StackError> {
        if !self.internal_stack.is_empty() {
            Ok(&self.internal_stack[self.internal_stack.len() - 1])
        } else {
            Err(StackError::StackUnderflowError)
        }
    }
}
