extern crate interpreterlib;

use interpreterlib::{bytecodes, constants, context, frame, interpreter, stack};

use interpreter::{BytecodeData, BytecodeType};
use stack::{StackElementType, StackEntry, StackEntryType};

pub fn execute_with_context(mut ctx: &mut context::Context) {
    let _result = interpreter::interpreter(&mut ctx);
}

pub fn execute(data: BytecodeData) -> context::Context {
    let mut ctx = context::Context::new(data);
    // silence result as we don't care here
    execute_with_context(&mut ctx);
    ctx
}

pub fn execute_bytecode(bc: u8) -> context::Context {
    let opcodes: BytecodeData = vec![bc as i8];
    execute(opcodes)
}

/// nop bytecode shall fail (at least for now)
#[test]
#[should_panic]
fn nop_test() {
    execute_bytecode(bytecodes::bytecode::nop as u8);
}


/// make sure the aconst_null pushes a null reference on the stack
#[test]
fn opcode_aconst_null_test() {
    let ctx = execute_bytecode(bytecodes::bytecode::aconst_null as u8);
    let top_entry = ctx.variables_stack.top().unwrap();
    assert!(
        top_entry.value == constants::NULL_HANDLE
            && top_entry.is_of_type(stack::StackEntryType::Reference)
    );
}


/// make sure the sconst_x pushes a null reference on the stack
#[test]
fn opcode_sconst_x_test() {
    for x in bytecodes::bytecode::sconst_m1 as u8..bytecodes::bytecode::sconst_5 as u8 {
        let expected_value = (x as i16) - (bytecodes::bytecode::sconst_0 as i16);
        let ctx = execute_bytecode(x);
        let top_entry = ctx.variables_stack.top().unwrap();
        assert!(
            (top_entry.value == expected_value)
                && (top_entry.is_of_type(stack::StackEntryType::Short))
        );
    }
}

#[test]
fn opcode_iconst_x_test() {
    for x in bytecodes::bytecode::iconst_m1 as u8..bytecodes::bytecode::iconst_5 as u8 {
        let expected_value1 = ((x as i32) - (bytecodes::bytecode::iconst_0 as i32) >> 16) as i16;
        let expected_value2 = ((x as i32) - (bytecodes::bytecode::iconst_0 as i32) & 0xFFFF) as i16;
        let ctx = execute_bytecode(x);
        let top_entry1 = ctx.variables_stack.peek_index(0).unwrap();
        let top_entry2 = ctx.variables_stack.peek_index(1).unwrap();
        // 2 MSB of value are at the top of the stack
        // next two bytes are in the next index of the stack
        assert!(
            (top_entry2.value == expected_value2)
                && (top_entry2.is_of_type(stack::StackEntryType::Int))
                && (top_entry1.value == expected_value1)
                && (top_entry1.is_of_type(stack::StackEntryType::Int))
        );
    }
}


#[test]
fn opcode_bspush_test() {
    let exp_value: u16 = 0xFFA5; /*0xA5*/
    let datatoexecute: BytecodeData = vec![
        bytecodes::bytecode::bspush as BytecodeType,
        exp_value as BytecodeType,
    ];
    let ctx = execute(datatoexecute);
    let top_entry = ctx.variables_stack.top().unwrap();
    println!(
        "Found value : {:04X} instead of {:04X}",
        top_entry.value,
        exp_value
    );
    assert!(
        top_entry.value as u16 == exp_value && top_entry.is_of_type(stack::StackEntryType::Short)
    );
}


#[test]
fn opcode_sspush_test() {
    let exp_value: u16 = 0xA55A; /*0xA5*/
    let datatoexecute: BytecodeData = vec![
        bytecodes::bytecode::sspush as BytecodeType,
        (exp_value >> 8) as BytecodeType,
        exp_value as BytecodeType,
    ];
    let ctx = execute(datatoexecute);
    let top_entry = ctx.variables_stack.top().unwrap();
    println!(
        "Found value : {:04X} instead of {:04X}",
        top_entry.value,
        exp_value
    );
    assert!(
        top_entry.value as u16 == exp_value && top_entry.is_of_type(stack::StackEntryType::Short)
    );
}



#[test]
fn opcode_bipush_test() {
    let exp_value: u32 = 0xFFFFFFA5; /*0xA5 + sign extension*/
    let datatoexecute: BytecodeData = vec![
        bytecodes::bytecode::bipush as BytecodeType,
        exp_value as BytecodeType,
    ];
    let ctx = execute(datatoexecute);

    let entry1 = ctx.variables_stack.peek_index(0).unwrap();
    let entry2 = ctx.variables_stack.peek_index(1).unwrap();
    let result: u32 = (entry1.value as u32) << 16 | (entry2.value as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert!(
        result == exp_value && entry1.is_of_type(stack::StackEntryType::Int)
            && entry2.is_of_type(stack::StackEntryType::Int)
    );
}


#[test]
fn opcode_sipush_test() {
    let exp_value: u32 = 0xFFFFA55A; /*0xA55A + sign extension*/
    let datatoexecute: BytecodeData = vec![
        bytecodes::bytecode::sipush as BytecodeType,
        (exp_value >> 8) as BytecodeType,
        exp_value as BytecodeType,
    ];
    let ctx = execute(datatoexecute);

    let entry1 = ctx.variables_stack.peek_index(0).unwrap();
    let entry2 = ctx.variables_stack.peek_index(1).unwrap();
    let result: u32 = (entry1.value as u32) << 16 | (entry2.value as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert!(
        result == exp_value && entry1.is_of_type(stack::StackEntryType::Int)
            && entry2.is_of_type(stack::StackEntryType::Int)
    );
}


#[test]
fn opcode_iipush_test() {
    let exp_value: u32 = 0xA55AA55A;
    let datatoexecute: BytecodeData = vec![
        bytecodes::bytecode::iipush as BytecodeType,
        (exp_value >> 24) as BytecodeType,
        (exp_value >> 16) as BytecodeType,
        (exp_value >> 8) as BytecodeType,
        exp_value as BytecodeType,
    ];
    let mut ctx = execute(datatoexecute);

    let entry1 = ctx.variables_stack.pop().unwrap();
    let entry2 = ctx.variables_stack.pop().unwrap();

    let result: u32 = ((entry1.value as u16) as u32) << 16 | ((entry2.value as u16) as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert!(
        result == exp_value && entry1.is_of_type(stack::StackEntryType::Int)
            && entry2.is_of_type(stack::StackEntryType::Int)
    );
}


#[test]
fn opcode_aload_test() {
    // prepare data for aload (a local variable of type reference in local variables at index 1)
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let idx: BytecodeType = 1;
    let datatoexecute: BytecodeData = vec![bytecodes::bytecode::aload as BytecodeType, idx];
    let mut ctx = context::Context::new(datatoexecute);
    let exp_value = StackEntry::from_values(
        (0xA55A as u16) as StackElementType,
        StackEntryType::Reference,
    );

    ctx.frame_stack.push(frame::Frame::new(2));
    {
        let top_frame = ctx.frame_stack.top_mut().unwrap();
        top_frame.set_local(1, &exp_value).unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result = ctx.frame_stack.top().unwrap()
        .get_local(idx as i16).unwrap();

    assert!(result.value == exp_value.value && result.is_of_type(stack::StackEntryType::Reference));
}
