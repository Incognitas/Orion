#![allow(dead_code)]
extern crate interpreterlib;

use interpreterlib::{interpreter, context};


fn main() {
    let opcodes: Vec<u8> = vec![2];
    let mut execution_context = context::Context::new(opcodes);

    //println!("Hello, world!");
    let _result = interpreter::interpreter(&mut execution_context);
    println!(
        "Content : {:X}",
        execution_context.variables_stack.pop().unwrap().value
    )
}
