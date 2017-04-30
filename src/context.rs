use stack::{Stack, StackError};
use frame::Frame;
use framestack::FrameStack;
use std::vec;

pub struct Context {
    pub variables_stack: Stack,
    pub frame_stack: FrameStack
}

impl Context {
    pub fn new() -> Context {
        Context { variables_stack : Stack::new(), frame_stack: FrameStack::new() }
    }

    pub fn currentFrame(&self) -> Result<&Frame, StackError> {
        self.frame_stack.top()
    }
}