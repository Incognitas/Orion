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
    let exp_value = StackEntry::from_values(
        (0xA55A as u16) as StackElementType,
        constants::PrimitiveType::REFERENCE,
    );

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
    let exp_value = StackEntry::from_values(
        (0xA55A as u16) as StackElementType,
        constants::PrimitiveType::SHORT,
    );

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
        top_frame
            .set_local(
                idx as i16,
                StackEntry::from_values(
                    (exp_value >> 16) as StackElementType,
                    constants::PrimitiveType::INTEGER,
                ),
            )
            .unwrap();
        top_frame
            .set_local(
                (idx + 1) as i16,
                StackEntry::from_values(
                    (exp_value & 0xFFFF) as StackElementType,
                    constants::PrimitiveType::INTEGER,
                ),
            )
            .unwrap();
    }
    // execute code
    execute_with_context(&mut ctx);

    // check that we have the right local in the stack (first, check it is a reference)
    let result1 = ctx.operand_stack
        .peek_index_check_type(0, constants::PrimitiveType::INTEGER)
        .unwrap();
    let result2 = ctx.operand_stack
        .peek_index_check_type(1, constants::PrimitiveType::INTEGER)
        .unwrap();

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
        opcode_xload_x_unittest(
            bytecode::from(curbc).unwrap(),
            curbc - (bytecode::aload_0 as u8),
            constants::PrimitiveType::REFERENCE,
        );
    }
}

///
/// Tests all sload_x opcodes (from standard specification)
///
#[test]
fn opcode_sload_x_tests() {
    for curbc in bytecode::sload_0 as u8..bytecode::sload_3 as u8 {
        opcode_xload_x_unittest(
            bytecode::from(curbc).unwrap(),
            curbc - (bytecode::sload_0 as u8),
            constants::PrimitiveType::SHORT,
        );
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
        let mut exp_value = StackEntry::from_values(
            (0xA55A as u16) as StackElementType,
            constants::PrimitiveType::INTEGER,
        );
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
        let result1 = ctx.operand_stack
            .pop_check_type(constants::PrimitiveType::INTEGER)
            .unwrap();
        let result2 = ctx.operand_stack
            .pop_check_type(constants::PrimitiveType::INTEGER)
            .unwrap();

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
    let exp_value: u32 = 0xA55AA55A;
    let is_persistent = true;
    let size_array: i16 = 20;
    let owner: i16 = 0;
    let flags: u8 = constants::ObjectFlags::ARRAY as u8;
    let idx: i16 = 2;

    // create the actual array
    let mut created_array = JCVMObject::new_array(owner, flags, type_, size_array, is_persistent);

    match type_ {
        constants::PrimitiveType::SHORT | constants::PrimitiveType::REFERENCE => {
            let size_one_entry = constants::SHORT_SIZE;
            // add a specific value that we will be able to check afterwards
            created_array
                .write_s(((idx as usize) * size_one_entry), exp_value as i16)
                .unwrap();
        }

        constants::PrimitiveType::BYTE => {
            // add a specific value that we will be able to check afterwards
            created_array
                .write_b(idx as usize, exp_value as i8)
                .unwrap();
        }

        constants::PrimitiveType::INTEGER => {
            let size_one_entry = constants::INTEGER_SIZE;

            created_array
                .write_i((idx as usize) * size_one_entry, exp_value as i32)
                .unwrap();
        }

        _ => {
            // this should never happen
            panic!("Error case");
        }
    };

    // register the array to the objects manager
    let refidx = ctx.object_manager.add_object(created_array);

    // now prepare the stack with approriate content
    // first, the index
    ctx.operand_stack.push(StackEntry::from_values(
        idx,
        constants::PrimitiveType::SHORT,
    ));
    // next, the arrayref
    ctx.operand_stack.push(StackEntry::from_values(
        refidx as i16,
        constants::PrimitiveType::REFERENCE,
    ));
    // execute code
    execute_with_context(&mut ctx);

    match type_ {
        constants::PrimitiveType::SHORT | constants::PrimitiveType::REFERENCE => {
            let result = ctx.operand_stack.pop_check_type(type_).unwrap();
            // now check that we have an entry with the given value on the stack
            println!(
                "obtained:{:04X}, expected: {:04X}",
                result.value as u16, exp_value
            );
            assert_eq!(result.value as u16, exp_value as u16);
        }

        constants::PrimitiveType::BYTE => {
            // now check that we have an entry with the given value on the stack
            let result = ctx.operand_stack.pop_check_type(type_).unwrap();

            println!(
                "obtained:{:04X}, expected: {:04X}",
                result.value as u8, exp_value as u8
            );
            assert_eq!(result.value as u8, exp_value as u8);
        }

        constants::PrimitiveType::INTEGER => {
            let mut result = ctx.operand_stack.pop_check_type(type_).unwrap();
            // now check that we have an entry with the given value on the stack
            println!(
                "obtained:{:04X}, expected: {:04X}",
                result.value as u16, exp_value as u16
            );
            assert_eq!(result.value as u16, (exp_value >> 16) as u16);

            result = ctx.operand_stack.pop_check_type(type_).unwrap();
            // check with the next entry pushed on stack that we have an entry with the given value on the stack
            println!(
                "obtained:{:04X}, expected: {:04X}",
                result.value as u16, exp_value as u16
            );
            assert_eq!(result.value as u16, exp_value as u16);
        }
        _ => { /*already handled before*/ }
    }
}

#[test]
fn opcode_aaload_test() {
    opcode_xaload_x_tests(bytecode::aaload, constants::PrimitiveType::REFERENCE);
}

#[test]
fn opcode_baload_test() {
    opcode_xaload_x_tests(bytecode::baload, constants::PrimitiveType::BYTE);
}

#[test]
fn opcode_saload_test() {
    opcode_xaload_x_tests(bytecode::saload, constants::PrimitiveType::SHORT);
}

#[test]
fn opcode_iaload_test() {
    opcode_xaload_x_tests(bytecode::iaload, constants::PrimitiveType::INTEGER);
}
