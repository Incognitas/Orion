use stack::Stack;
use frame::Frame;
use framestack::FrameStack;
use jcvmerrors::InterpreterError;

pub struct Context {
    pub variables_stack: Stack,
    pub frame_stack: FrameStack,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables_stack: Stack::new(255),
            frame_stack: FrameStack::new(),
        }
    }

    pub fn currentFrame(&self) -> Result<&Frame, InterpreterError> {
        self.frame_stack.top()
    }
}