//use std::io;
mod interpreter;
mod bcutils;
mod bytecodes;
mod stack;
mod context;
mod frame;
mod framestack;
mod jcvmerrors;

fn main() {
    let opcodes: &[u8] = &[1];
    let mut execution_context = context::Context::new();

    //println!("Hello, world!");
    interpreter::interpreter(opcodes, &mut execution_context);
    println!("Content : {:X}",
             execution_context.variables_stack.pop().unwrap())
}
