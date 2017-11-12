use bytecodes::bytecode;
use jcvmerrors::InterpreterError;

use interpreter::{BytecodeType, BytecodeData};

pub struct BytecodeFetcher {
    bc_array: BytecodeData,
    offset: usize,
}

impl BytecodeFetcher {
    /// Initialization method
    pub fn new(bc: BytecodeData) -> BytecodeFetcher {
        BytecodeFetcher {
            bc_array: bc,
            offset: 0,
        }
    }

    pub fn get(&self, i: usize) -> Result<BytecodeType, InterpreterError> {
        let res = self.bc_array.get(i)
                               .ok_or(InterpreterError::EndOfStream)?;
        Ok(*res)
    }

    pub fn current_offset(&self) -> usize {
        self.offset
    }

    /// fetches a bytecode and return its associated value
    pub fn fetch_bytecode(&mut self) -> Result<bytecode, InterpreterError> {
        bytecode::from(self.fetch_b()? as u8)
    }

    /// Fetches one byte from the internal array at given index and return it (if any)
    pub fn fetch_b(&mut self) -> Result<BytecodeType, InterpreterError> {
        let r = self.get(self.offset)?;
        self.offset += 1;
        Ok(r)
    }

    /// Fetches one short from the internal array at given index and return it (if any)
    pub fn fetch_s(&mut self) -> Result<u16, InterpreterError> {
        let r = (self.get(self.offset)? as u16) << 8
                | self.get(self.offset+1)? as u16; 
        self.offset += 2;
        Ok(r)
    }

    /// Fetches one integer from the internal array at given index and return it (if any)
    pub fn fetch_i(&mut self) -> Result<u32, InterpreterError> {
        let r1 = (self.get(self.offset+0)? as u32) << 8 | self.get(self.offset+1)? as u32; 
        let r2 = (self.get(self.offset+2)? as u32) << 8 | self.get(self.offset+3)? as u32; 
        let r = r1 << 16 | r2;
        self.offset += 4;
        Ok(r)
    }
}
