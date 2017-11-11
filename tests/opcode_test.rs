extern crate interpreterlib;

use interpreterlib::{bytecodes,interpreter,context,stack,constants};


#[test]
#[should_panic]
fn nop_test() {
    let opcodes: Vec<u8> = vec![bytecodes::bytecode::opcode_nop as u8];
    let mut ctx = context::Context::new();
    ctx.bytecode_fetcher.bc_array = opcodes;
    // silence result as we don't care here
    let _result = interpreter::interpreter(&mut ctx);
}


#[test]
/// make sure the aconst_null pushes a null reference on the stack
fn opcode_aconst_null_test() {
    let opcodes: Vec<u8> = vec![bytecodes::bytecode::opcode_aconst_null as u8];
    let mut ctx = context::Context::new();
    ctx.bytecode_fetcher.bc_array = opcodes;
    // silence result as we don't care here
    let _result = interpreter::interpreter(&mut ctx);
    let top_entry = ctx.variables_stack.top().unwrap();

    assert!(top_entry.value == constants::NULL_HANDLE && top_entry.is_of_type(stack::StackEntryType::reference));
}
