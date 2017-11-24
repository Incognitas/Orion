use constants::PrimitiveType;
use jcvmerrors::InterpreterError;

///
/// Provides methods allowing to check the primitive type associated to an instance
///
pub trait HasType {
    fn is_of_type(&self, cmp_val: PrimitiveType) -> bool;
}

pub trait BufferAccessor {
    fn read_b(&self, offset: usize) -> Result<i8, InterpreterError>;
    fn read_s(&self, offset: usize) -> Result<i16, InterpreterError>;
    fn read_i(&self, offset: usize) -> Result<i32, InterpreterError>;
    fn write_b(&mut self, offset: usize, val: i8) -> Result<(), InterpreterError>;
    fn write_s(&mut self, offset: usize, val: i16) -> Result<(), InterpreterError>;
    fn write_i(&mut self, offset: usize, val: i32) -> Result<(), InterpreterError>;
}
