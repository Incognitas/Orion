use bcutils;
use bytecodes::bytecode;
use context::Context;
use stack::StackEntryType;
use jcvmerrors::InterpreterError;

static NULL_HANDLE: i16 = 0;

pub fn interpreter(mut execution_context: &mut Context) -> Result<(), InterpreterError> {

    loop {
        let current_opcode = try!(execution_context.bytecode_fetcher.fetch_bytecode());

        //println!("Found bytecode : {:02X}", current_opcode.unwrap() as u8);
        match current_opcode {
            // bytecode 0 : NOP
            bytecode::opcode_nop => {
                println!("NOP bytecode reached at offset 0x{:X}",
                         execution_context.bytecode_fetcher.current_offset());
                panic!("Unexpected NOP bytecode !");
            }
            // bytecode 1 : ACONST_NULL
            bytecode::opcode_aconst_null => {
                try!(vm_spush(&mut execution_context, NULL_HANDLE));
            }

            // bytecode 2 : SCONST_M1
            bytecode::opcode_sconst_m1 => {
                try!(vm_spush(&mut execution_context, -1 as i16));
            }

            // bytecode 3: SCONST_0
            bytecode::opcode_sconst_0 => try!(vm_spush(&mut execution_context, 0)),
            // bytecode 4: SCONST_1
            bytecode::opcode_sconst_1 => try!(vm_spush(&mut execution_context, 1)),
            // bytecode 5: SCONST_2
            bytecode::opcode_sconst_2 => try!(vm_spush(&mut execution_context, 2)),
            // bytecode 6: SCONST_3
            bytecode::opcode_sconst_3 => try!(vm_spush(&mut execution_context, 3)),
            // bytecode 7: SCONST_4
            bytecode::opcode_sconst_4 => try!(vm_spush(&mut execution_context, 4)),
            // bytecode 8: SCONST_5
            bytecode::opcode_sconst_5 => try!(vm_spush(&mut execution_context, 5)),
            // bytecode 9: ICONST_M1
            bytecode::opcode_iconst_m1 => try!(vm_ipush(&mut execution_context, -1)),
            // bytecode 10: ICONST_0
            bytecode::opcode_iconst_0 => try!(vm_ipush(&mut execution_context, 0)),
            // bytecode 11: ICONST_1
            bytecode::opcode_iconst_1 => try!(vm_ipush(&mut execution_context, 1)),
            // bytecode 12: ICONST_2
            bytecode::opcode_iconst_2 => try!(vm_ipush(&mut execution_context, 2)),
            // bytecode 13: ICONST_3
            bytecode::opcode_iconst_3 => try!(vm_ipush(&mut execution_context, 3)),
            // bytecode 14: ICONST_4
            bytecode::opcode_iconst_4 => try!(vm_ipush(&mut execution_context, 4)),
            // bytecode 15: ICONST_5
            bytecode::opcode_iconst_5 => try!(vm_ipush(&mut execution_context, 5)),
            // bytecode 16: BSPUSH
            bytecode::opcode_bspush => {
                match execution_context.bytecode_fetcher.fetch_b() {
                    Ok(value) => try!(vm_spush(&mut execution_context, value as i16)),
                    Err(e) => return Err(e),
                }
            }
            // bytecode 17: SSPUSH
            bytecode::opcode_sspush => {
                match execution_context.bytecode_fetcher.fetch_s() {
                    Ok(value) => try!(vm_spush(&mut execution_context, value as i16)),
                    Err(e) => return Err(e),
                }
            }
            // bytecode 18: BIPUSH
            bytecode::opcode_bipush => {
                match execution_context.bytecode_fetcher.fetch_b() {
                    Ok(value) => try!(vm_ipush(&mut execution_context, value as i32)),
                    Err(e) => return Err(e),
                }
            }
            // bytecode 19: SIPUSH
            bytecode::opcode_sipush => {
                match execution_context.bytecode_fetcher.fetch_s() {
                    Ok(value) => try!(vm_ipush(&mut execution_context, value as i32)),
                    Err(e) => return Err(e),
                }
            }
            // bytecode 20: IIPUSH
            bytecode::opcode_iipush => {
                match execution_context.bytecode_fetcher.fetch_i() {
                    Ok(value) => try!(vm_ipush(&mut execution_context, value as i32)),
                    Err(e) => return Err(e),
                }
            }
            // bytecode 21: ALOAD
            bytecode::opcode_aload => {
                let index = try!(execution_context.bytecode_fetcher.fetch_b());
                // read local from current frame
                let current_local = match execution_context.current_frame() {
                    Ok(e) => try!(e.getLocal(index)),
                    Err(e) => return Err(e),
                };
                vm_spush(&mut execution_context, current_local);
            }

            _ => break,
        }
    }

    Ok(())
}

fn vm_spush(context: &mut Context, value: StackEntryType) -> Result<(), InterpreterError> {
    context.variables_stack.push(value)
}

fn vm_ipush(context: &mut Context, value: i32) -> Result<(), InterpreterError> {
    try!(vm_spush(context, (value & 0xFFFF) as StackEntryType));
    vm_spush(context, (value >> 16) as StackEntryType)
}