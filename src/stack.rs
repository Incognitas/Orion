use jcvmerrors::InterpreterError;

// available types on the stack of variables
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum StackEntryType {
    Unknown = 0x00,
    Byte = 0x01,
    Short = 0x02,
    Int = 0x04,
    Reference = 0x08,
}

pub type StackElementType = i16;

// one entry in the stack
#[derive(Copy, Clone)]
pub struct StackEntry {
    pub value: StackElementType,
    entry_type: StackEntryType,
}

// implementation associated to the StackEntry
impl StackEntry {
    pub fn new() -> StackEntry {
        StackEntry {
            value: 0,
            entry_type: StackEntryType::Unknown
        }
    }
    pub fn from_values(val: StackElementType, type_: StackEntryType) -> StackEntry {
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
        Stack {
            internal_stack: Vec::with_capacity(capacity),
        }
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
        self.push(StackEntry::from_values(value as i16, StackEntryType::Byte))
    }

    pub fn apush(&mut self, value: i16) -> Result<(), InterpreterError> {
        self.push(StackEntry::from_values(value, StackEntryType::Reference))
    }

    pub fn spush(&mut self, value: i16) -> Result<(), InterpreterError> {
        self.push(StackEntry::from_values(value, StackEntryType::Short))
    }

    pub fn ipush(&mut self, value: i32) -> Result<(), InterpreterError> {
        try!(self.push(StackEntry::from_values(
            (value & 0xFFFF) as i16,
            StackEntryType::Int
        )));
        self.push(StackEntry::from_values((value >> 16) as i16, StackEntryType::Int))
    }

    // removes top item and returns its value
    pub fn pop(&mut self) -> Result<StackEntry, InterpreterError> {
        self.internal_stack
            .pop()
            .ok_or(InterpreterError::StackUnderflowError)
    }

    /// returns the top element of the stack without removing it from the stack
    pub fn top(&self) -> Result<StackEntry, InterpreterError> {
        self.peek_index(0)
    }

    pub fn peek_index(&self, index: i16) -> Result<StackEntry, InterpreterError> {
        if self.internal_stack.len() > index as usize {
            Ok(self.internal_stack[self.internal_stack.len() - 1 - index as usize])
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }

    pub fn peek_index_check_type(
        &self,
        index: i16,
        type_: StackEntryType,
    ) -> Result<StackEntry, InterpreterError> {
        if self.internal_stack.len() > index as usize {
            let current_val = self.internal_stack[self.internal_stack.len() - 1 - index as usize];
            if current_val.entry_type == type_ {
                return Ok(current_val);
            }
            Err(InterpreterError::InvalidVariableType(
                current_val.entry_type,
                type_,
            ))
        } else {
            Err(InterpreterError::StackUnderflowError)
        }
    }

    pub fn update_index(&mut self, index: i16, newval: &StackEntry) -> Result<(), InterpreterError> {
        if (index as usize) < self.internal_stack.capacity() {
            let maxlen = self.internal_stack.capacity();
            if let Some(value_to_update) = self.internal_stack.get_mut(maxlen - (index as usize) - 1) {
                (*value_to_update).value = newval.value;
                (*value_to_update).entry_type = newval.entry_type;
                return Ok(());
            }
        }
        Err(InterpreterError::IndexOutOfBound)
    }
}
