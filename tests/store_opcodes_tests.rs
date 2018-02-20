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
/// Test all xstore  codes opcode from standard specification
///
fn opcode_xstore_test(curbytecode: bytecode, type_: constants::PrimitiveType, idx: u8) {
    // prepare data for xstore
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let datatoexecute: &BytecodeData = &[curbytecode as BytecodeType, idx as BytecodeType];
    // create context
    let mut ctx = context::Context::new(datatoexecute);
    // prepare expected value
    let exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType, type_);

    ctx.operand_stack.push(exp_value);
    ctx.operand_stack.push(exp_value);

    ctx.frame_stack.push(frame::Frame::new(idx + 3));

    // execute code
    execute_with_context(&mut ctx);
    // check that we have the right local in the stack (first, check it is a reference)
    let mut result = ctx.frame_stack
        .top()
        .unwrap()
        .get_local_check_type(idx as i16, type_)
        .unwrap();

    assert_eq!(result.value, exp_value.value);
    assert!(result.is_of_type(type_));

    if type_ == constants::PrimitiveType::INTEGER {
        // special case for integers, we have to check the next index also
        result = ctx.frame_stack
            .top()
            .unwrap()
            .get_local_check_type((idx + 1) as i16, type_)
            .unwrap();

        assert_eq!(result.value, exp_value.value);
        assert!(result.is_of_type(type_));
    }
}

///
/// Test sload opcode from standard specification
///
#[test]
fn opcode_astore_test_nominal() {
    opcode_xstore_test(bytecode::astore, constants::PrimitiveType::REFERENCE, 1);
}

#[should_panic]
#[test]
fn opcode_astore_test_bad_type() {
    // bad entry types in operand stack should raise an error
    opcode_xstore_test(bytecode::astore, constants::PrimitiveType::SHORT, 1);
}

#[test]
fn opcode_sstore_test_nominal() {
    opcode_xstore_test(bytecode::sstore, constants::PrimitiveType::SHORT, 1);
}

#[test]
#[should_panic]
fn opcode_sstore_test_bad_type() {
    // bad entry types in operand stack should raise an error
    opcode_xstore_test(bytecode::sstore, constants::PrimitiveType::REFERENCE, 1);
}

#[test]
fn opcode_istore_test_nominal() {
    opcode_xstore_test(bytecode::istore, constants::PrimitiveType::INTEGER, 1);
}

#[test]
#[should_panic]
fn opcode_istore_test_bad_type() {
    // bad entry types in operand stack should raise an error
    opcode_xstore_test(bytecode::istore, constants::PrimitiveType::SHORT, 1);
}

///
/// Test all xstore  codes opcode from standard specification
///
fn opcode_xstore_x_test(curbytecode: bytecode, type_: constants::PrimitiveType, idx: u8) {
    // prepare data for xstore
    // we voluntarily don't  use offset 0 to make sure we pick the right value
    let datatoexecute: &BytecodeData = &[curbytecode as BytecodeType];
    // create context
    let mut ctx = context::Context::new(datatoexecute);
    // prepare expected value
    let exp_value = StackEntry::from_values((0xA55A as u16) as StackElementType, type_);

    ctx.operand_stack.push(exp_value);
    ctx.operand_stack.push(exp_value);

    ctx.frame_stack.push(frame::Frame::new(idx + 3));

    // execute code
    execute_with_context(&mut ctx);
    // check that we have the right local in the stack (first, check it is a reference)
    let mut result = ctx.frame_stack
        .top()
        .unwrap()
        .get_local_check_type(idx as i16, type_)
        .unwrap();

    assert_eq!(result.value, exp_value.value);
    assert!(result.is_of_type(type_));

    if type_ == constants::PrimitiveType::INTEGER {
        // special case for integers, we have to check the next index also
        result = ctx.frame_stack
            .top()
            .unwrap()
            .get_local_check_type((idx + 1) as i16, type_)
            .unwrap();

        assert_eq!(result.value, exp_value.value);
        assert!(result.is_of_type(type_));
    }
}

#[test]
fn opcode_astore_x_test() {
    for x in bytecode::astore_0 as u8..bytecode::astore_3 as u8 {
        opcode_xstore_x_test(
            bytecode::from(x).unwrap(),
            constants::PrimitiveType::REFERENCE,
            x - (bytecode::astore_0 as u8),
        );
    }
}

#[test]
fn opcode_sstore_x_test() {
    for x in bytecode::sstore_0 as u8..bytecode::sstore_3 as u8 {
        opcode_xstore_x_test(
            bytecode::from(x).unwrap(),
            constants::PrimitiveType::SHORT,
            x - (bytecode::sstore_0 as u8),
        );
    }
}

#[test]
fn opcode_istore_x_test() {
    for x in bytecode::istore_0 as u8..bytecode::istore_3 as u8 {
        opcode_xstore_x_test(
            bytecode::from(x).unwrap(),
            constants::PrimitiveType::INTEGER,
            x - (bytecode::istore_0 as u8),
        );
    }
}
