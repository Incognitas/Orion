//use std::io;
mod interpreter;
mod bcutils;
mod bytecodes;
mod stack;
mod context;
mod frame;
mod framestack;
use std::borrow::Borrow;

fn main() {
    let opcodes:&[u8] = &[1];
    let mut executionContext = Box::new(context::Context::new());

    //println!("Hello, world!");
    interpreter::interpreter(opcodes, executionContext);
    println!("Content : {:X}", (*executionContext).variables_stack.pop().unwrap())
}
