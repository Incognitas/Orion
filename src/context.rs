use std::vec;
use stack::Stack;
use frame::Frame;
use framestack::FrameStack;
use jcvmerrors::InterpreterError;
use bcutils::BytecodeFetcher;
use objects::JCVMObject;

type HandleTable = Vec<Box<JCVMObject>>;

pub struct Context<'a> {
    pub bytecode_fetcher: BytecodeFetcher<'a>,
    pub variables_stack: Stack,
    pub frame_stack: FrameStack,
    pub handle_table: HandleTable,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            bytecode_fetcher: BytecodeFetcher::new(),
            variables_stack: Stack::new(256),
            frame_stack: FrameStack::new(),
            handle_table: Vec::new(), // <== define capacity for this handle table ? Like Vec::with_capacity(1024)
        }
    }

    pub fn current_frame(&self) -> Result<&Frame, InterpreterError> {
        self.frame_stack.top()
    }
}