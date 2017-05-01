use stack::StackEntryType;

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
    InvalidVariableType(StackEntryType, StackEntryType),
}
