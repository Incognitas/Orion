use jcvmerrors::InterpreterError;

// available types on the stack of variables
#[derive(Debug,Copy,Clone)]
#[derive(PartialEq)]
#[repr(u8)]
pub enum StackEntryType {
    unknown = 0x00,
    byte = 0x01,
    short = 0x02,
    int = 0x04,
    reference = 0x08,
}

// one entry in the stack
#[derive(Copy, Clone)]
pub struct StackEntry {
    pub value: i16,
    entry_type: StackEntryType,
}

// implementation associated to the StackEntry
impl StackEntry {
    pub fn new(val: i16, type_: StackEntryType) -> StackEntry {
        StackEntry {
            value: val,
            entry_type: type_,
        }
    }

    pub fn is_of_type(&self, type_checked: StackEntryType) -> bool {
        return self.entry_type == type_checked;
    }
}

// basic implementation of a stack
pub struct Stack {
    internal_stack: Vec<StackEntry>,
}

// associated methods/functions
impl Stack {
    pub fn new(capacity: usize) -> Stack {
        Stack { internal_stack: Vec::with_capacity(capacity) }
    }

    // push a value on the stack
    pub fn push(&mut self, value: StackEntry) -> Result<(), InterpreterError> {
        if self.internal_stack.len() < self.internal_stack.capacity() {
            self.internal_stack.push(value);
            return Ok(());
        }
        Err(InterpreterError::StackOverflowError)
    }

    pub fn bpush(&mut self, value: i8) -> Result<(), InterpreterError> {
        self.push(StackEntry::new(value as i16, StackEntryType::byte))
    }

    pub fn apush(&mut self, value: i16) -> Result<(), InterpreterError> {
        self.push(StackEntry::new(value, StackEntryType::reference))
    }

    pub fn spush(&mut self, value: i16) -> Result<(), InterpreterError> {
        self.push(StackEntry::new(value, StackEntryType::short))
    }

    pub fn ipush(&mut self, value: i32) -> Result<(), InterpreterError> {
        try!(self.push(StackEntry::new((value & 0xFFFF) as i16, StackEntryType::int)));
        self.push(StackEntry::new((value >> 16) as i16, StackEntryType::int))
    }

    // removes top item and returns its value
    pub fn pop(&mut self) -> Result<StackEntry, InterpreterError> {
        self.internal_stack
            .pop()
            .ok_or(InterpreterError::StackUnderflowError)
    }

    pub fn peek(&self) -> Result<StackEntry, InterpreterError> {
        self.peek_index(0)
    }

    pub fn peek_index(&self, index: u8) -> Result<StackEntry, InterpreterError> {
        if self.internal_stack.len() > index as usize {
            Ok(self.internal_stack[self.internal_stack.len() - 1 - index as usize])
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }

    pub fn peek_index_check_type(&self,
                                 index: u8,
                                 type_: StackEntryType)
                                 -> Result<StackEntry, InterpreterError> {
        if self.internal_stack.len() > index as usize {
            let current_val = self.internal_stack[self.internal_stack.len() - 1 - index as usize];
            if current_val.entry_type == type_ {
                return Ok(current_val);
            }
            Err(InterpreterError::InvalidVariableType(current_val.entry_type, type_))
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }
}
