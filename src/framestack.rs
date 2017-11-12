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
        match self.internal_stack.last() {
            Some(result) => Ok(result),
            None => Err(InterpreterError::StackUnderflowError)
        }
    }

    pub fn top_mut(&mut self) -> Result<&mut Frame, InterpreterError> {
        match self.internal_stack.last_mut() {
            Some(result) => Ok(result),
            None => Err(InterpreterError::StackUnderflowError)
        }
    }
}
