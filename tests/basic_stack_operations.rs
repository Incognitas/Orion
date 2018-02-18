extern crate interpreterlib;

use interpreterlib::{constants, context, interpreter};
use interpreterlib::bytecodes::*;

use interpreter::{BytecodeData, BytecodeType};

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
