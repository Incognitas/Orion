extern crate byteorder;

use std::io::Cursor;
use std::mem;
use bcutils::byteorder::{BigEndian, ReadBytesExt};
use bytecodes;


/*enum FetchingError {
    EndOfStream
}*/

pub struct BytecodeFetcher<'a> {
    pub bc_array: &'a[u8],
    pub offset: usize
}

impl<'a> BytecodeFetcher<'a> {
    /// Initialization method
    pub fn new(&self, bc_array: &'a [u8], offset: usize) -> BytecodeFetcher {
        BytecodeFetcher { bc_array:bc_array, offset:offset }
    }

    /// fetches a bytecode and return its associated value
    pub fn fetchBytecode(&mut self) -> Option<bytecodes::bytecode> {
        let curVal = self.fetchB().unwrap_or(bytecodes::bytecode::opcode_END as u8);
        if curVal < bytecodes::bytecode::opcode_END as u8 {
            Some(unsafe {mem::transmute(curVal)})
        } else {
            None
        }
    }

    /// Fetches one byte from the internal array at given index and return it (if any)
    pub fn fetchB(&mut self) -> Option<u8>{
        let cur_offset = self.offset;
        self.offset += 1;
        if cur_offset < self.bc_array.len() {
            return Some(self.bc_array[cur_offset]);
        }
        None
    }

    /// Fetches one short from the internal array at given index and return it (if any)
    pub fn fetchS(&mut self) -> Option<u16>{
        let mut buf = Cursor::new(&self.bc_array[self.offset as usize..]);
        self.offset += 2;
        if self.offset < (self.bc_array.len() - 1) {
            return Some(buf.read_u16::<BigEndian>().unwrap());
        }
        None
    }

    /// Fetches one integer from the internal array at given index and return it (if any)
    pub fn fetchI(&mut self) -> Option<u32>{
        let mut buf = Cursor::new(&self.bc_array[self.offset as usize..]);
        self.offset += 4;
        if self.offset < (self.bc_array.len() - 1) {
            return Some(buf.read_u32::<BigEndian>().unwrap());
        }
        None
    }
}
