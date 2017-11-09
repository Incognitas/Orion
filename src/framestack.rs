use frame::Frame;
use jcvmerrors::InterpreterError;

pub struct FrameStack {
    internal_stack: Vec<Frame>,
}

impl FrameStack {
    pub fn new() -> FrameStack {
        FrameStack {
            internal_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, new_frame: Frame) {
        self.internal_stack.push(new_frame);
    }

    pub fn top(&self) -> Result<&Frame, InterpreterError> {
        if !self.internal_stack.is_empty() {
            Ok(&(self.internal_stack[self.internal_stack.len() - 1]))
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }
}
