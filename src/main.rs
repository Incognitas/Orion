//use std::io;
mod interpreter;
mod bcutils;
mod bytecodes;
mod stack;
mod context;

fn main() {
    let opcodes:&[u8] = &[1];
    //println!("Hello, world!");
    interpreter::interpreter(opcodes);
}
