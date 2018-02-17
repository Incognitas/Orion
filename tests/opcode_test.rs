extern crate interpreterlib;

use interpreterlib::{constants, context, frame, interpreter, objects, stack, traits};
use interpreterlib::bytecodes::*;

use interpreter::{BytecodeData, BytecodeType};
use stack::{StackElementType, StackEntry};
use objects::JCVMObject;
use traits::BufferAccessor;

pub fn execute_with_context(mut ctx: &mut context::Context) {
    let _result = interpreter::interpreter(&mut ctx);
}

pub fn execute_bytecode<'a>(data: &'a BytecodeData) -> context::Context<'a> {
    let mut ctx = context::Context::new(data);
    // silence result as we don't care here
    execute_with_context(&mut ctx);
    ctx
}

/// nop bytecode shall fail (at least for now)
#[test]
#[should_panic]
fn nop_test() {
    execute_bytecode(&[bytecode::nop as i8]);
}

/// make sure the aconst_null pushes a null reference on the stack
#[test]
fn opcode_aconst_null_test() {
    let ctx = execute_bytecode(&[bytecode::aconst_null as i8]);
    // let ctx = execute_opcode(bytecode::aconst_null as i8);
    let top_entry = ctx.operand_stack.top().unwrap();
    assert_eq!(top_entry.value, constants::NULL_HANDLE);
    assert!(top_entry.is_of_type(constants::PrimitiveType::REFERENCE));
}

/// make sure the sconst_x pushes a null reference on the stack
#[test]
fn opcode_sconst_x_test() {
    for x in bytecode::sconst_m1 as u8..bytecode::sconst_5 as u8 {
        let expected_value = (x as i16) - (bytecode::sconst_0 as i16);
        // let ctx = execute_opcode(x as i8);
        let dat = &[x as i8];
        let ctx = execute_bytecode(dat);
        let top_entry = ctx.operand_stack.top().unwrap();
        assert_eq!(top_entry.value, expected_value);
        assert!(top_entry.is_of_type(constants::PrimitiveType::SHORT));
    }
}

///
/// Test all iconst_x opcodes from standard specification
///
#[test]
fn opcode_iconst_x_test() {
    for x in bytecode::iconst_m1 as u8..bytecode::iconst_5 as u8 {
        let expected_value1 = ((x as i32) - (bytecode::iconst_0 as i32) >> 16) as i16;
        let expected_value2 = ((x as i32) - (bytecode::iconst_0 as i32) & 0xFFFF) as i16;
        let dat = &[x as i8];
        let ctx = execute_bytecode(dat);

        let top_entry1 = ctx.operand_stack
            .peek_index_check_type(0, constants::PrimitiveType::INTEGER)
            .unwrap();
        let top_entry2 = ctx.operand_stack
            .peek_index_check_type(1, constants::PrimitiveType::INTEGER)
            .unwrap();
        // 2 MSB of value are at the top of the stack
        // next two bytes are in the next index of the stack
        assert_eq!(top_entry2.value, expected_value2);
        assert_eq!(top_entry1.value, expected_value1);
    }
}

///
/// Test bspush opcode from standard specification
///
#[test]
fn opcode_bspush_test() {
    let exp_value: u16 = 0xFFA5; /*0xA5*/
    let datatoexecute: &BytecodeData =
        &[bytecode::bspush as BytecodeType, exp_value as BytecodeType];
    let ctx = execute_bytecode(datatoexecute);
    let top_entry = ctx.operand_stack.top().unwrap();
    println!(
        "Found value : {:04X} instead of {:04X}",
        top_entry.value, exp_value
    );
    assert_eq!(top_entry.value as u16, exp_value);
    assert!(top_entry.is_of_type(constants::PrimitiveType::SHORT));
}

///
/// Test sspush opcode from standard specification
///
#[test]
fn opcode_sspush_test() {
    let exp_value: u16 = 0xA55A; /*0xA5*/
    let datatoexecute: &BytecodeData = &[
                                        bytecode::sspush as BytecodeType,
                                        (exp_value >> 8) as BytecodeType,
                                        exp_value as BytecodeType,
                                        ];
    let ctx = execute_bytecode(datatoexecute);
    let top_entry = ctx.operand_stack.top().unwrap();
    println!(
        "Found value : {:04X} instead of {:04X}",
        top_entry.value, exp_value
    );
    assert_eq!(top_entry.value as u16, exp_value);
    assert!(top_entry.is_of_type(constants::PrimitiveType::SHORT));
}

///
/// Test bipush opcode from standard specification
///
#[test]
fn opcode_bipush_test() {
    let exp_value: u32 = 0xFFFFFFA5; /*0xA5 + sign extension*/
    let datatoexecute: &BytecodeData =
        &[bytecode::bipush as BytecodeType, exp_value as BytecodeType];
    let ctx = execute_bytecode(datatoexecute);

    let entry1 = ctx.operand_stack
        .peek_index_check_type(0, constants::PrimitiveType::INTEGER)
        .unwrap();
    let entry2 = ctx.operand_stack
        .peek_index_check_type(1, constants::PrimitiveType::INTEGER)
        .unwrap();
    let result: u32 = (entry1.value as u32) << 16 | (entry2.value as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert_eq!(result, exp_value);
}

///
/// Test sipush opcode from standard specification
///
#[test]
fn opcode_sipush_test() {
    let exp_value: u32 = 0xFFFFA55A; /*0xA55A + sign extension*/
    let datatoexecute: &BytecodeData = &[
        bytecode::sipush as BytecodeType,
        (exp_value >> 8) as BytecodeType,
        exp_value as BytecodeType,
    ];
    let ctx = execute_bytecode(datatoexecute);

    let entry1 = ctx.operand_stack
        .peek_index_check_type(0, constants::PrimitiveType::INTEGER)
        .unwrap();
    let entry2 = ctx.operand_stack
        .peek_index_check_type(1, constants::PrimitiveType::INTEGER)
        .unwrap();
    let result: u32 = (entry1.value as u32) << 16 | (entry2.value as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert_eq!(result, exp_value);
}

///
/// Test iipush opcode from standard specification
///
#[test]
fn opcode_iipush_test() {
    let exp_value: u32 = 0xA55AA55A;
    let datatoexecute: &BytecodeData = &[
        bytecode::iipush as BytecodeType,
        (exp_value >> 24) as BytecodeType,
        (exp_value >> 16) as BytecodeType,
        (exp_value >> 8) as BytecodeType,
        exp_value as BytecodeType,
    ];
    let mut ctx = execute_bytecode(datatoexecute);

    let entry1 = ctx.operand_stack.pop().unwrap();
    let entry2 = ctx.operand_stack.pop().unwrap();

    let result: u32 = ((entry1.value as u16) as u32) << 16 | ((entry2.value as u16) as u32);

    println!("Found value : {:08X} instead of {:08X}", result, exp_value);
    assert_eq!(result, exp_value);
    assert!(entry1.is_of_type(constants::PrimitiveType::INTEGER));
    assert!(entry2.is_of_type(constants::PrimitiveType::INTEGER));
}

///
/// Test aload opcode from standard specification
///
#[test]
fn opcode_aload_test() {
    // prepare data for aload (a local variable of type reference in local variables at index 1)
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let idx: BytecodeType = 1;
    let datatoexecute: &BytecodeData = &[bytecode::aload as BytecodeType, idx];
    let mut ctx = context::Context::new(datatoexecute);
    let exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType,
                                            constants::PrimitiveType::REFERENCE);

    ctx.frame_stack.push(frame::Frame::new(2));
    {
        let top_frame = ctx.frame_stack.top_mut().unwrap();
        top_frame.set_local(1, exp_value).unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result = ctx.operand_stack.top().unwrap();

    assert_eq!(result.value, exp_value.value);
    assert!(result.is_of_type(constants::PrimitiveType::REFERENCE));
}

///
/// Test sload opcode from standard specification
///
#[test]
fn opcode_sload_test() {
    // prepare data for aload (a local variable of type reference in local variables at index 1)
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let idx: BytecodeType = 1;
    let datatoexecute: &BytecodeData = &[bytecode::sload as BytecodeType, idx];
    let mut ctx = context::Context::new(datatoexecute);
    let exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType,
                                            constants::PrimitiveType::SHORT);

    ctx.frame_stack.push(frame::Frame::new(2));
    {
        let top_frame = ctx.frame_stack.top_mut().unwrap();
        top_frame.set_local(idx as i16, exp_value).unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result = ctx.operand_stack.top().unwrap();

    assert_eq!(result.value, exp_value.value);
    assert!(result.is_of_type(constants::PrimitiveType::SHORT));
}

///
/// Test iload opcode from standard specification
///
#[test]
fn opcode_iload_test() {
    // prepare data for aload (a local variable of type reference in local variables at index 1)
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let idx: BytecodeType = 1;
    let datatoexecute: &BytecodeData = &[bytecode::iload as BytecodeType, idx];
    let mut ctx = context::Context::new(datatoexecute);
    let exp_value: u32 = 0xA55A5AA5;

    ctx.frame_stack.push(frame::Frame::new((idx + 2) as u8));
    {
        let top_frame = ctx.frame_stack.top_mut().unwrap();
        top_frame.set_local(idx as i16,
                       StackEntry::from_values((exp_value >> 16) as StackElementType,
                                               constants::PrimitiveType::INTEGER))
            .unwrap();
        top_frame.set_local((idx + 1) as i16,
                       StackEntry::from_values((exp_value & 0xFFFF) as StackElementType,
                                               constants::PrimitiveType::INTEGER))
            .unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result1 =
        ctx.operand_stack.peek_index_check_type(0, constants::PrimitiveType::INTEGER).unwrap();
    let result2 =
        ctx.operand_stack.peek_index_check_type(1, constants::PrimitiveType::INTEGER).unwrap();

    assert_eq!(result1.value, (exp_value >> 16) as i16);
    assert_eq!(result2.value, (exp_value & 0xFFFF) as i16);
}

///
/// utility function provided for ease of testing
///
fn opcode_xload_x_unittest(bc: bytecode, idx: u8, type_: constants::PrimitiveType) {
    // prepare data for aload (a local variable of type reference in local variables at index 1)
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let datatoexecute: &BytecodeData = &[bc as BytecodeType];
    let mut ctx = context::Context::new(datatoexecute);
    let exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType, type_);

    // create a new frame
    ctx.frame_stack.push(frame::Frame::new(idx + 1));
    {
        // push a value on the frame
        let top_frame = ctx.frame_stack.top_mut().unwrap();
        top_frame.set_local(idx as i16, exp_value).unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result = ctx.operand_stack.top().unwrap();

    assert_eq!(result.value, exp_value.value);
    assert!(result.is_of_type(type_));
}

///
/// Tests all aload_x opcodes (from standard specification)
///
#[test]
fn opcode_aload_x_tests() {
    for curbc in bytecode::aload_0 as u8..bytecode::aload_3 as u8 {
        opcode_xload_x_unittest(bytecode::from(curbc).unwrap(),
                                curbc - (bytecode::aload_0 as u8),
                                constants::PrimitiveType::REFERENCE);
    }
}

///
/// Tests all sload_x opcodes (from standard specification)
///
#[test]
fn opcode_sload_x_tests() {
    for curbc in bytecode::sload_0 as u8..bytecode::sload_3 as u8 {
        opcode_xload_x_unittest(bytecode::from(curbc).unwrap(),
                                curbc - (bytecode::sload_0 as u8),
                                constants::PrimitiveType::SHORT);
    }
}

///
/// Tests all iload_x opcodes (from standard specification)
///
#[test]
fn opcode_iload_x_tests() {
    for curbc in bytecode::iload_0 as u8..bytecode::iload_3 as u8 {
        // prepare data for aload (a local variable of type reference in local variables at index 1)
        // we voluntarily don't  use offset 0 to make sure we pick the right value
        let datatoexecute: &BytecodeData = &[curbc as BytecodeType];
        let mut ctx = context::Context::new(datatoexecute);
        let mut exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType,
                                                    constants::PrimitiveType::INTEGER);
        let idx = curbc - (bytecode::iload_0 as u8);
        // create a new frame
        ctx.frame_stack.push(frame::Frame::new(idx + 2));
        {
            // push a value on the frame
            let top_frame = ctx.frame_stack.top_mut().unwrap();
            top_frame.set_local(idx as i16, exp_value).unwrap();
            exp_value.value = !exp_value.value;
            top_frame.set_local((idx + 1) as i16, exp_value).unwrap();
        }

        // execute code
        execute_with_context(&mut ctx);

        // check that we have the right local in the stack (first, check it is a reference)
        let result1 = ctx.operand_stack.pop_check_type(constants::PrimitiveType::INTEGER).unwrap();
        let result2 = ctx.operand_stack.pop_check_type(constants::PrimitiveType::INTEGER).unwrap();

        assert_eq!(result1.value, !exp_value.value);
        assert_eq!(result2.value, exp_value.value);
    }
}

///
/// Tests all iload_x opcodes (from standard specification)
///
fn opcode_xaload_x_tests(bc: bytecode, type_: constants::PrimitiveType) {
    let datatoexecute: &BytecodeData = &[bc as BytecodeType];
    let mut ctx = context::Context::new(datatoexecute);

    // first, push the index in the array
    let exp_value: u16 = 0xA55A;
    let is_persistent = true;
    let size_array: i16 = 10;
    let owner: i16 = 0;
    let flags: u8 = constants::ObjectFlags::ARRAY as u8;
    let idx: i16 = 2;
    // create the actual array
    let mut created_array = JCVMObject::new_array(owner, flags, type_, size_array, is_persistent);
    // add a specific value that we will be able to check afterwards
    created_array.write_s((idx * constants::REFERENCE_SIZE) as usize, exp_value as i16).unwrap();
    // register the array to the objects manager
    let refidx = ctx.object_manager.add_object(created_array);

    // now prepare the stack with approriate content
    // first, the index
    ctx.operand_stack.push(StackEntry::from_values(idx, constants::PrimitiveType::SHORT)).unwrap();
    // next, the arrayref
    ctx.operand_stack
        .push(StackEntry::from_values(refidx as i16, constants::PrimitiveType::REFERENCE))
        .unwrap();

    // execute code
    execute_with_context(&mut ctx);

    // now check that we have an entry with the given value on the stack
    let result = ctx.operand_stack.pop_check_type(type_).unwrap();
    println!("obtained:{:04X}, expected: {:04X}",
             result.value as u16,
             exp_value);
    assert_eq!(result.value as u16, exp_value);
}

#[test]
fn opcode_aaload_test() {
    opcode_xaload_x_tests(bytecode::aaload,
                          constants::PrimitiveType::REFERENCE);
}
