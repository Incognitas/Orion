#![allow(dead_code)]
#[macro_use]
mod exceptions;
mod interpreter;
mod bcutils;
mod bytecodes;
mod stack;
mod context;
mod frame;
mod framestack;
mod jcvmerrors;
mod constants;
mod objects;
mod objectsmanager;

fn main() {
    let opcodes: &[u8] = &[2];
    let mut execution_context = context::Context::new();
    execution_context.bytecode_fetcher.bc_array = Some(opcodes);

    //println!("Hello, world!");
    interpreter::interpreter(&mut execution_context);
    println!(
        "Content : {:X}",
        execution_context.variables_stack.pop().unwrap().value
    )
}
