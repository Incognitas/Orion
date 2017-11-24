use constants;
use jcvmerrors::InterpreterError;
use traits::{HasType, BufferAccessor};
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

impl BufferAccessor for JCVMObject {
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

    fn write_b(&mut self, offset: usize, val: i8) -> Result<(), InterpreterError> {
        self.put(offset, val)
    }

    fn write_s(&mut self, offset: usize, val: i16) -> Result<(), InterpreterError> {
        self.put(offset, (val >> 8) as i8)?;
        self.put(offset + 1, i8::from(val as i8))
    }

    fn write_i(&mut self, offset: usize, val: i32) -> Result<(), InterpreterError> {
        self.put(offset, (val >> 24) as i8)?;
        self.put(offset + 1, (val >> 16) as i8)?;
        self.put(offset + 2, (val >> 8) as i8)?;
        self.put(offset + 3, i8::from(val as i8))
    }
    
}


impl JCVMObject {
    pub fn new(owner: i16,
               flags_: u8,
               ptype: constants::PrimitiveType,
               length: i16,
               persistent: bool)
               -> JCVMObject {
        JCVMObject {
            owner: owner,
            object_flags: flags_,
            primitive_type: ptype,
            object_length: length,
            persistent: persistent,
            content: vec![0; length as usize],
        }
    }

    pub fn new_array(owner: i16,
                     flags_: u8,
                     ptype: constants::PrimitiveType,
                     length: i16,
                     persistent: bool)
                     -> JCVMObject {
        JCVMObject {
            owner: owner,
            object_flags: flags_ | (constants::ObjectFlags::ARRAY as u8),
            primitive_type: ptype,
            object_length: length,
            persistent: persistent,
            content: vec![0; length as usize],
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

    pub fn get(&self, offset: usize) -> Result<BytecodeType, InterpreterError> {
        let res = self.content.get(offset).ok_or(InterpreterError::IndexOutOfBound)?;
        Ok(*res)
    }

    pub fn put(&mut self, offset: usize, val: i8) -> Result<(), InterpreterError> {
        if offset < self.content.len() {
            self.content[offset] = val;
            Ok(())
        }
        else {
            Err(InterpreterError::IndexOutOfBound)
        }
        
    }

    pub fn is_array(&self) -> bool {
        ((self.flags() as u8) & (constants::ObjectFlags::ARRAY as u8)) != 0
    }
}
