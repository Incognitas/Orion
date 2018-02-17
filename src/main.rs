#![allow(dead_code)]
extern crate interpreterlib;

use interpreterlib::{context, interpreter};
use interpreter::BytecodeData;

fn main() {
    let opcodes: &BytecodeData = &[2, 5, 4];
    let mut execution_context = context::Context::new(opcodes);

    //println!("Hello, world!");
    let _result = interpreter::interpreter(&mut execution_context);
    println!(
        "Content : {:X}",
        execution_context.operand_stack.pop().unwrap().value
    )
}
