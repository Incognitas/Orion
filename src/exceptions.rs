use context::Context;
use jcvmerrors::InterpreterError;
use exceptions;

#[derive(Debug)]
pub enum InterpreterException {
    NullPointerException,
    ArrayIndexOutOfBoundsException,
    SecurityException,
}

pub fn throw_exception(
    _ctx: &Context,
    _except: exceptions::InterpreterException,
) -> Result<(), InterpreterException> {
    println!("JC Exception raised !");
    panic!("WIP");
    Ok(())
}

pub fn throw_exception_from_interpretererror(
    _ctx: &Context,
    _except: InterpreterError,
) -> Result<(), InterpreterException> {
    println!("JC Exception raised !");
    panic!("WIP");
    Ok(())
}
