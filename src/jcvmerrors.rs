use constants;

#[derive(Debug)]
pub enum InterpreterError {
    // stack errors
    StackOverflowError,
    StackUnderflowError,
    // errors associated to the bytecode fetching
    EndOfStream,
    IndexOutOfBound,
    UnrecognizedBytecode,
    NoBytecodeToFetch,
    InvalidVariableType(constants::PrimitiveType, constants::PrimitiveType),
}
