use context::Context;
use jcvmerrors::InterpreterError;
use constants::NULL_HANDLE;
use stack::StackEntryType;

// macro allowing to simplify null reference check
macro_rules! check_null_reference {
    ($variable:ident, $ctx:ident) => (
        if !$variable.is_of_type(StackEntryType::reference) || $variable.value == constants::NULL_HANDLE {
            try!(throw_exception($ctx, NULL_POINTER_EXCEPTION));
        }
    )
}

pub const NULL_POINTER_EXCEPTION: i16 = 1;

pub fn throw_exception(ctx: &Context, except: i16) -> Result<(), InterpreterError>
{
    panic!("WIP");
    Ok(())
}