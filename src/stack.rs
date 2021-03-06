use jcvmerrors::InterpreterError;
use traits::HasType;
use constants;

// available types on the stack of variables
pub type StackElementType = i16;

// one entry in the stack
#[derive(Copy, Clone)]
pub struct StackEntry {
    pub value: StackElementType,
    entry_type: constants::PrimitiveType,
}

impl HasType for StackEntry {
    fn is_of_type(&self, cmp_val: constants::PrimitiveType) -> bool {
        self.entry_type == cmp_val
    }
}

// implementation associated to the StackEntry
impl StackEntry {
    pub fn new() -> StackEntry {
        StackEntry {
            value: 0,
            entry_type: constants::PrimitiveType::UNKNOWN,
        }
    }
    pub fn from_values(val: StackElementType, type_: constants::PrimitiveType) -> StackEntry {
        StackEntry {
            value: val,
            entry_type: type_,
        }
    }

    pub fn is_of_type(&self, type_checked: constants::PrimitiveType) -> bool {
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
    pub fn push(&mut self, value: StackEntry) {
        if self.internal_stack.len() < self.internal_stack.capacity() {
            self.internal_stack.push(value);
        } else {
            panic!("Stack overflow error, is it a malformed code that is run ?");
        }
    }

    pub fn bpush(&mut self, value: i8) {
        self.push(StackEntry::from_values(
            value as i16,
            constants::PrimitiveType::BYTE,
        ))
    }

    pub fn apush(&mut self, value: i16) {
        self.push(StackEntry::from_values(
            value,
            constants::PrimitiveType::REFERENCE,
        ))
    }

    pub fn spush(&mut self, value: i16) {
        self.push(StackEntry::from_values(
            value,
            constants::PrimitiveType::SHORT,
        ));
    }

    pub fn ipush(&mut self, value: i32) {
        self.push(StackEntry::from_values(
            (value & 0xFFFF) as i16,
            constants::PrimitiveType::INTEGER,
        ));

        self.push(StackEntry::from_values(
            (value >> 16) as i16,
            constants::PrimitiveType::INTEGER,
        ))
    }

    // removes top item and returns its value
    pub fn pop(&mut self) -> Option<StackEntry> {
        match self.internal_stack.pop() {
            Some(res) => Some(res),
            None => panic!("Stack underflow error ! Is it a malformed applet ?"),
        }
    }
    pub fn pop_check_type(
        &mut self,
        type_: constants::PrimitiveType,
    ) -> Result<StackEntry, InterpreterError> {
        let result = match self.pop() {
            Some(entry) => {
                if !entry.is_of_type(type_) {
                    return Err(InterpreterError::InvalidVariableType(
                        entry.entry_type,
                        type_,
                    ));
                }

                return Ok(entry);
            }

            None => {
                return Err(InterpreterError::StackUnderflowError);
            }
        };
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
        type_: constants::PrimitiveType,
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

    pub fn update_index(&mut self, index: i16, newval: StackEntry) -> Result<(), InterpreterError> {
        if (index as usize) < self.internal_stack.len() {
            let maxlen = self.internal_stack.len();
            if let Some(value_to_update) =
                self.internal_stack.get_mut(maxlen - (index as usize) - 1)
            {
                (*value_to_update).value = newval.value;
                (*value_to_update).entry_type = newval.entry_type;
                return Ok(());
            }
        }
        Err(InterpreterError::IndexOutOfBound)
    }
}
