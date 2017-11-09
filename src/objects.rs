use constants;
use jcvmerrors::InterpreterError;

// a structure representing an object
pub struct JCVMObject {
    owner: i16,
    object_flags: u8,
    primitive_type: constants::PrimitiveType,
    object_length: i16, // length in terms of raw length (not in terms of items etc)
    persistent: bool,
    pub content: Vec<i8>, // sometimes we don't have arrays
}


impl JCVMObject {
    pub fn new(
        owner: i16,
        flags_: u8,
        ptype: constants::PrimitiveType,
        length: i16,
        persistent: bool,
    ) -> JCVMObject {
        JCVMObject {
            owner: owner,
            object_flags: flags_,
            primitive_type: ptype,
            object_length: length,
            persistent: persistent,
            content: Vec::new(),
        }
    }

    pub fn owner(&self) -> i16 {
        self.owner
    }

    pub fn flags(&self) -> u8 {
        self.object_flags
    }

    pub fn length(&self) -> i16 {
        self.object_length
    }

    pub fn primitive_type(&self) -> &constants::PrimitiveType {
        &self.primitive_type
    }

    pub fn at_index_b(&self, index: usize) -> Result<i8, InterpreterError> {
        if index < self.content.len() {
            return Ok(self.content[index]);
        }
        Err(InterpreterError::IndexOutOfBound)
    }
    /*
    pub fn at_index_s(&self, index: usize) -> Result<i16, InterpreterError> {
        if index < self.content.len() {
            return Ok(self.content[index]);
        }
        Err(InterpreterError::IndexOutOfBound)
    }*/
}
