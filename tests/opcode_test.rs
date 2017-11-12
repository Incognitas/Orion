extern crate interpreterlib;

use interpreterlib::{bytecodes,interpreter,context,stack,constants};

use interpreter::{BytecodeData, BytecodeType};

pub fn execute(data: BytecodeData) -> context::Context  {
    let mut ctx = context::Context::new(data);
    // silence result as we don't care here
    let _result = interpreter::interpreter(&mut ctx);
    ctx
}

pub fn execute_bytecode(bc: u8) -> context::Context {
    let opcodes: BytecodeData = vec![bc as i8];
    execute(opcodes)
}

#[test]
#[should_panic]
fn nop_test() {
     execute_bytecode(bytecodes::bytecode::nop as u8);
}


#[test]
/// make sure the aconst_null pushes a null reference on the stack
fn opcode_aconst_null_test() {
    let ctx = execute_bytecode(bytecodes::bytecode::aconst_null as u8);
    let top_entry = ctx.variables_stack.top().unwrap();
    assert!(top_entry.value == constants::NULL_HANDLE && top_entry.is_of_type(stack::StackEntryType::Reference));
}


/// make sure the sconst_x pushes a null reference on the stack
#[test]
fn opcode_sconst_x_test() {
    for x in bytecodes::bytecode::sconst_m1 as u8..bytecodes::bytecode::sconst_5  as u8 {
        let expected_value = (x as i16) - (bytecodes::bytecode::sconst_0 as i16);
        let ctx = execute_bytecode(x);
        let top_entry = ctx.variables_stack.top().unwrap();
        assert!((top_entry.value == expected_value) && (top_entry.is_of_type(stack::StackEntryType::Short)));
    }
}

#[test]
fn opcode_iconst_x_test() {
    for x in bytecodes::bytecode::iconst_m1 as u8..bytecodes::bytecode::iconst_5  as u8 {
        let expected_value1 = ((x as i32) - (bytecodes::bytecode::iconst_0 as i32) >> 16) as i16;
        let expected_value2 = ((x as i32) - (bytecodes::bytecode::iconst_0 as i32) & 0xFFFF) as i16;
        let ctx = execute_bytecode(x);
        let top_entry1 = ctx.variables_stack.peek_index(0).unwrap();
        let top_entry2 = ctx.variables_stack.peek_index(1).unwrap();
        // 2 MSB of value are at the top of the stack
        // next two bytes are in the next index of the stack
        assert!((top_entry2.value == expected_value2) 
                && (top_entry2.is_of_type(stack::StackEntryType::Int))
                && (top_entry1.value == expected_value1)
                && (top_entry1.is_of_type(stack::StackEntryType::Int)));
    }
}


#[test]
fn opcode_bspush_test() {
    let datatoexecute: BytecodeData = vec![bytecodes::bytecode::bspush as BytecodeType,  -91/*0xA5*/];
    let ctx = execute(datatoexecute);
    let top_entry = ctx.variables_stack.top().unwrap();
    println!("Found value : {:04X}", top_entry.value);
    assert!(top_entry.value == -91 && top_entry.is_of_type(stack::StackEntryType::Short));
}


#[test]
fn opcode_sspush_test() {
    let datatoexecute: BytecodeData = vec![bytecodes::bytecode::sspush as BytecodeType, -91, 90]; /*0xA55A*/
    let ctx = execute(datatoexecute);
    let top_entry = ctx.variables_stack.top().unwrap();
    println!("Found value : {:04X}", (top_entry.value) as u16);
    assert!(top_entry.value as u16 == 0xA55A as u16 && top_entry.is_of_type(stack::StackEntryType::Short));
}
