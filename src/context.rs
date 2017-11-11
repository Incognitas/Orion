use stack::Stack;
use frame::Frame;
use framestack::FrameStack;
use jcvmerrors::InterpreterError;
use bcutils::BytecodeFetcher;
use objectsmanager::ObjectManager;

// pub struct Context<'a> {
//     pub bytecode_fetcher: BytecodeFetcher<'a>,
pub struct Context {
    pub bytecode_fetcher: BytecodeFetcher,
    pub variables_stack: Stack,
    pub frame_stack: FrameStack,
    pub object_manager: ObjectManager,
}

impl Context {
    pub fn new() -> Context {
        Context {
            bytecode_fetcher: BytecodeFetcher::new(),
            variables_stack: Stack::new(256),
            frame_stack: FrameStack::new(),
            object_manager: ObjectManager::new(),
        }
    }

    pub fn current_frame(&self) -> Result<&Frame, InterpreterError> {
        self.frame_stack.top()
    }
}
