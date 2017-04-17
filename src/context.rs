use stack::Stack;

pub struct Context {
    pub variables_stack: Stack
}

impl Context {
    pub fn new() -> Context {
        Context { variables_stack : Stack::new() }
    }
}