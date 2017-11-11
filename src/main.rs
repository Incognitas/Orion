#![allow(dead_code)]
extern crate interpreterlib;

use interpreterlib::{interpreter, context};


fn main() {
    let opcodes: Vec<u8> = vec![2];
    let mut execution_context = context::Context::new();
    execution_context.bytecode_fetcher.bc_array = opcodes;

    //println!("Hello, world!");
    let _result = interpreter::interpreter(&mut execution_context);
    println!(
        "Content : {:X}",
        execution_context.variables_stack.pop().unwrap().value
    )
}
