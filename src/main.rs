//use std::io;
mod interpreter;
mod bcutils;
mod bytecodes;

fn main() {
    let opcodes:&[u8] = &[0];
    println!("Hello, world!");
    interpreter::interpreter(&[0]);
}
