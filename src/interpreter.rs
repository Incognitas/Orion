use bcutils;
use bytecodes::bytecode;
use context::Context;

static null_handle: i16 = 0;

pub fn interpreter(opcodes: &[u8])  {

    let mut executionContext = Context::new();
    let mut fetcher = bcutils::BytecodeFetcher{bc_array:opcodes, offset:0};
    loop {
        let current_opcode = fetcher.fetchBytecode();
        //println!("Found bytecode : {:02X}", current_opcode.unwrap() as u8);
        match current_opcode.unwrap() {
            // bytecode 0 : NOP
            bytecode::opcode_nop => {
                    println!("NOP bytecode reached at offset 0x{:X}", fetcher.currentOffset());
                    panic!("Unexpected NOP bytecode !");
                }
            // bytecode 1 : ACONST_NULL
            bytecode::opcode_aconst_null => {
                vm_spush(&mut executionContext, null_handle);
            }

            // bytecode 2 : SCONST_M1
            bytecode::opcode_sconst_m1 => {
                vm_spush(&mut executionContext, -1 as i16);
            }

            // bytecode 3: SCONST_0
            bytecode::opcode_sconst_0 => vm_spush(&mut executionContext, 0),
            // bytecode 4: SCONST_1
            bytecode::opcode_sconst_1 => vm_spush(&mut executionContext, 1),
            // bytecode 5: SCONST_2
            bytecode::opcode_sconst_2 => vm_spush(&mut executionContext, 2),
            // bytecode 6: SCONST_3
            bytecode::opcode_sconst_3 => vm_spush(&mut executionContext, 3),
            // bytecode 7: SCONST_4
            bytecode::opcode_sconst_4 => vm_spush(&mut executionContext, 4),
            // bytecode 8: SCONST_5
            bytecode::opcode_sconst_5 => vm_spush(&mut executionContext, 5),
            // bytecode 9: ICONST_M1
            bytecode::opcode_iconst_m1 => vm_ipush(&mut executionContext, -1),
            // bytecode 10: ICONST_0
            bytecode::opcode_iconst_0 => vm_ipush(&mut executionContext, 0),
            // bytecode 11: ICONST_1
            bytecode::opcode_iconst_1 => vm_ipush(&mut executionContext, 1),
            // bytecode 12: ICONST_2
            bytecode::opcode_iconst_2 => vm_ipush(&mut executionContext, 2),
            // bytecode 13: ICONST_3
            bytecode::opcode_iconst_3 => vm_ipush(&mut executionContext, 3),
            // bytecode 14: ICONST_4
            bytecode::opcode_iconst_4 => vm_ipush(&mut executionContext, 4),
            // bytecode 15: ICONST_5
            bytecode::opcode_iconst_5 => vm_ipush(&mut executionContext, 5),
            // bytecode 16: BSPUSH
            bytecode::opcode_bspush => vm_spush(&mut executionContext, fetcher.fetchB().unwrap() as i16),
            // bytecode 17: SSPUSH
            bytecode::opcode_sspush => vm_spush(&mut executionContext, fetcher.fetchS().unwrap() as i16),
            // bytecode 18: BIPUSH
            bytecode::opcode_bipush => vm_ipush(&mut executionContext, fetcher.fetchB().unwrap() as i32),
            // bytecode 19: SIPUSH
            bytecode::opcode_sipush => vm_ipush(&mut executionContext, fetcher.fetchS().unwrap() as i32),
            // bytecode 20: IIPUSH
            bytecode::opcode_iipush => vm_ipush(&mut executionContext, fetcher.fetchI().unwrap() as i32),

            _ => {break}
        }
    }
}

fn vm_spush(context: &mut Context, value: i16)
{
    context.variables_stack.push(value);
}

fn vm_ipush(context: &mut Context, value: i32)
{
    context.variables_stack.push((value & 0xFFFF) as i16);
    context.variables_stack.push((value >> 16) as i16);
}