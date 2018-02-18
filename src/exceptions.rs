use context::Context;
use jcvmerrors::InterpreterError;
use exceptions;

#[derive(Debug)]
pub enum InterpreterException {
    NullPointerException,
    ArrayIndexOutOfBoundsException,
}

pub fn throw_exception(_ctx: &Context, _except: exceptions::InterpreterException) {
    println!("JC Exception raised !");
    panic!("WIP");
}

pub fn throw_exception_from_interpretererror(_ctx: &Context, _except: InterpreterError) {
    println!("JC Exception raised !");
    panic!("WIP");
}
