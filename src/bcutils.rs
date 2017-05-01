extern crate byteorder;

use std::io::Cursor;
use std::mem;
use bcutils::byteorder::{BigEndian, ReadBytesExt};
use bytecodes::bytecode;
use jcvmerrors::InterpreterError;

pub struct BytecodeFetcher<'a> {
    pub bc_array: &'a [u8],
    pub offset: usize,
}

impl<'a> BytecodeFetcher<'a> {
    /// Initialization method
    pub fn new(&self, bc_array: &'a [u8], offset: usize) -> BytecodeFetcher {
        BytecodeFetcher {
            bc_array: bc_array,
            offset: offset,
        }
    }

    pub fn current_offset(&self) -> usize {
        self.offset
    }

    /// fetches a bytecode and return its associated value
    pub fn fetch_bytecode(&mut self) -> Result<bytecode, InterpreterError> {
        match self.fetch_b() {
            Ok(result) => {
                if result < bytecode::opcode_END as u8 {
                    Ok(unsafe { mem::transmute(result) })
                } else {
                    Err(InterpreterError::UnrecognizedBytecode)
                }
            }

            Err(why) => Err(why),
        }
    }

    /// Fetches one byte from the internal array at given index and return it (if any)
    pub fn fetch_b(&mut self) -> Result<u8, InterpreterError> {
        let cur_offset = self.offset;
        self.offset += 1;
        if cur_offset < self.bc_array.len() {
            Ok(self.bc_array[cur_offset])
        } else {
            Err(InterpreterError::EndOfStream)
        }
    }

    /// Fetches one short from the internal array at given index and return it (if any)
    pub fn fetch_s(&mut self) -> Result<u16, InterpreterError> {
        let mut buf = Cursor::new(&self.bc_array[self.offset as usize..]);
        self.offset += 2;
        if self.offset < (self.bc_array.len() - 1) {
            Ok(buf.read_u16::<BigEndian>().unwrap())
        } else {
            Err(InterpreterError::EndOfStream)
        }

    }

    /// Fetches one integer from the internal array at given index and return it (if any)
    pub fn fetch_i(&mut self) -> Result<u32, InterpreterError> {
        let mut buf = Cursor::new(&self.bc_array[self.offset as usize..]);
        self.offset += 4;
        if self.offset < (self.bc_array.len() - 1) {
            Ok(buf.read_u32::<BigEndian>().unwrap())
        } else {
            Err(InterpreterError::EndOfStream)
        }
    }
}
