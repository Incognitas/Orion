use constants;
use jcvmerrors::InterpreterError;
use traits::HasType;


// a structure representing an object
pub struct JCVMObject {
    owner: i16,
    object_flags: u8,
    primitive_type: constants::PrimitiveType,
    object_length: i16, // length in terms of raw length (not in terms of items etc)
    persistent: bool,
    pub content: Vec<i8>, // sometimes we don't have arrays
}

impl HasType for JCVMObject {
    ///
    /// Indicates true if the current instance is of type 'cmp_val'
    /// 
    fn is_of_type(&self, cmp_val: constants::PrimitiveType) -> bool {
        self.primitive_type == cmp_val 
    }
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

    pub fn new_array(
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
