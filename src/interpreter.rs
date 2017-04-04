use bcutils;
use bytecodes;

pub fn interpreter(opcodes: &[u8])  {

    let mut fetcher = bcutils::BytecodeFetcher{bc_array:opcodes, offset:0};
    loop {
        let current_opcode = fetcher.fetchBytecode();
        //println!("Found bytecode : {:02X}", current_opcode.unwrap() as u8);
        match current_opcode {
            Some(bytecodes::bytecode::opcode_nop) => {println!("plop")}
            _ => {break}
        }
    }
}
