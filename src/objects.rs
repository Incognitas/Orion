use constants;
use jcvmerrors::InterpreterError;
use traits::{HasType, DataReader};
use interpreter::BytecodeType;

type InternalBuffer = Vec<i8>;

// a structure representing an object
pub struct JCVMObject {
    owner: i16,
    object_flags: u8,
    primitive_type: constants::PrimitiveType,
    object_length: i16, // length in terms of raw length (not in terms of items etc)
    persistent: bool,
    content: InternalBuffer, // sometimes we don't have arrays
}

impl HasType for JCVMObject {
    ///
    /// Indicates true if the current instance is of type 'cmp_val'
    /// 
    fn is_of_type(&self, cmp_val: constants::PrimitiveType) -> bool {
        self.primitive_type == cmp_val 
    }
}

impl DataReader for JCVMObject {
    fn read_b(&self, offset: usize) -> Result<i8, InterpreterError> {
        self.get(offset)
    }

    fn read_s(&self, offset: usize) -> Result<i16, InterpreterError> {
        let r = (self.get(offset)? as i16) << 8 | self.get(offset + 1)? as i16;
        Ok(r)
    }

    fn read_i(&self, offset: usize) -> Result<i32, InterpreterError> {
        let r1 = (self.get(offset)? as u16) << 8 | self.get(offset + 1)? as u16;
        let r2 = (self.get(offset + 2)? as u16) << 8 | self.get(offset + 3)? as u16;
        let r = (u32::from(r1) << 16) | u32::from(r2);
        Ok(r as i32)
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

    pub fn get(&self, offset: usize) -> Result<BytecodeType, InterpreterError> {
        let res = self.content.get(offset).ok_or(InterpreterError::IndexOutOfBound)?;
        Ok(*res)
    }
            
    pub fn is_array(&self) -> bool {
        ((self.flags() as u8) & (constants::ObjectFlags::ARRAY as u8)) != 0
    }
}
