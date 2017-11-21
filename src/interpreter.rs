use bytecodes::bytecode;
use context::Context;
use stack::StackEntry;
use jcvmerrors::InterpreterError;
use constants;
use exceptions;
use traits::{HasType, DataReader};

pub type BytecodeType = i8;
pub type BytecodeData = Vec<BytecodeType>;


// macro allowing to simplify null reference check
macro_rules! check_null_reference {
    ($variable:ident, $ctx:ident) => (
        if !$variable.is_of_type(constants::PrimitiveType::REFERENCE) || $variable.value == constants::NULL_HANDLE {
            try!(exceptions::throw_exception($ctx, exceptions::InterpreterException::NullPointerException));
        }
    )
}


pub fn interpreter(execution_context: &mut Context) -> Result<(), InterpreterError> {
    loop {
        let current_opcode = execution_context.bytecode_fetcher.fetch_bytecode()?;

        //println!("Found bytecode : {:02X}", current_opcode.unwrap() as u8);
        match current_opcode {
            // bytecode 0 : NOP
            bytecode::nop => {
                println!(
                    "NOP bytecode reached at offset 0x{:X}",
                    execution_context.bytecode_fetcher.current_offset()
                );
                panic!("Unexpected NOP bytecode !");
            }
            // bytecode 1 : ACONST_NULL
            bytecode::aconst_null => {
                try!(
                    execution_context
                        .operand_stack
                        .apush(constants::NULL_HANDLE)
                );
            }

            // bytecode 2 : SCONST_M1
            bytecode::sconst_m1 => {
                execution_context.operand_stack.spush(-1 as i16)?;
            }

            // bytecode 3: SCONST_0
            bytecode::sconst_0 => execution_context.operand_stack.spush(0)?,
            // bytecode 4: SCONST_1
            bytecode::sconst_1 => execution_context.operand_stack.spush(1)?,
            // bytecode 5: SCONST_2
            bytecode::sconst_2 => execution_context.operand_stack.spush(2)?,
            // bytecode 6: SCONST_3
            bytecode::sconst_3 => execution_context.operand_stack.spush(3)?,
            // bytecode 7: SCONST_4
            bytecode::sconst_4 => execution_context.operand_stack.spush(4)?,
            // bytecode 8: SCONST_5
            bytecode::sconst_5 => execution_context.operand_stack.spush(5)?,
            // bytecode 9: ICONST_M1
            bytecode::iconst_m1 => execution_context.operand_stack.ipush(-1)?,
            // bytecode 10: ICONST_0
            bytecode::iconst_0 => execution_context.operand_stack.ipush(0)?,
            // bytecode 11: ICONST_1
            bytecode::iconst_1 => execution_context.operand_stack.ipush(1)?,
            // bytecode 12: ICONST_2
            bytecode::iconst_2 => execution_context.operand_stack.ipush(2)?,
            // bytecode 13: ICONST_3
            bytecode::iconst_3 => execution_context.operand_stack.ipush(3)?,
            // bytecode 14: ICONST_4
            bytecode::iconst_4 => execution_context.operand_stack.ipush(4)?,
            // bytecode 15: ICONST_5
            bytecode::iconst_5 => execution_context.operand_stack.ipush(5)?,
            // bytecode 16: BSPUSH
            bytecode::bspush => execution_context
                .operand_stack
                .spush(i16::from(execution_context.bytecode_fetcher.fetch_b()?))?,
            // bytecode 17: SSPUSH
            bytecode::sspush => execution_context
                .operand_stack
                .spush(execution_context.bytecode_fetcher.fetch_s()?)?,
            // bytecode 18: BIPUSH
            bytecode::bipush => execution_context
                .operand_stack
                .ipush(execution_context.bytecode_fetcher.fetch_b()? as i32)?,
            // bytecode 19: SIPUSH
            bytecode::sipush => execution_context
                .operand_stack
                .ipush(execution_context.bytecode_fetcher.fetch_s()? as i32)?,
            // bytecode 20: IIPUSH
            bytecode::iipush => execution_context
                .operand_stack
                .ipush(execution_context.bytecode_fetcher.fetch_i()? as i32)?,
            // bytecode 21: ALOAD
            bytecode::aload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // read local from current frame
                let current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(i16::from(index), constants::PrimitiveType::REFERENCE)?;

                execution_context.operand_stack.push(current_local)?;
            }

            // bytecode 22: SLOAD
            bytecode::sload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // read local from current frame
                let current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(i16::from(index), constants::PrimitiveType::SHORT)?;

                execution_context.operand_stack.push(current_local)?;
            }

            // bytecode 23: ILOAD
            bytecode::iload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // note: the int takes 2 slots in the VM
                // we have to read those 2 variables and put them on the stack
                // read local from current frame
                let current_local1 = execution_context
                    .current_frame()?
                    .get_local_check_type(i16::from(index), constants::PrimitiveType::INTEGER)?;
                let current_local2 = execution_context
                    .current_frame()?
                    .get_local_check_type(i16::from(index + 1), constants::PrimitiveType::INTEGER)?;

                // push variables in reverse order to keep the original order
                execution_context.operand_stack.push(current_local2)?;
                execution_context.operand_stack.push(current_local1)?;
            }

            // bytecode 24...27: ALOAD_0...ALOAD_3
            bytecode::aload_0 |
            bytecode::aload_1 |
            bytecode::aload_2 |
            bytecode::aload_3 => {
                // read local from current frame
                let current_local = execution_context.current_frame()?.get_local_check_type(
                    i16::from(current_opcode as u8 - bytecode::aload_0 as u8),
                    constants::PrimitiveType::REFERENCE,
                )?;

                execution_context.operand_stack.push(current_local)?;
            }

            // bytecode 28...31: SLOAD_0...SLOAD_3
            bytecode::sload_0 |
            bytecode::sload_1 |
            bytecode::sload_2 |
            bytecode::sload_3 => {
                // read local from current frame
                let current_local = execution_context.current_frame()?.get_local_check_type(
                    i16::from(current_opcode as u8 - bytecode::sload_0 as u8),
                    constants::PrimitiveType::SHORT,
                )?;

                execution_context.operand_stack.push(current_local)?;
            }

            // bytecode 32...35: ILOAD_0...ILOAD_3
            bytecode::iload_0 |
            bytecode::iload_1 |
            bytecode::iload_2 |
            bytecode::iload_3 => {
                // read local from current frame
                let current_val = i16::from(current_opcode as u8 - bytecode::iload_0 as u8);
                let mut current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(current_val + 1, constants::PrimitiveType::INTEGER)?;

                execution_context.operand_stack.push(current_local)?;

                current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(current_val, constants::PrimitiveType::INTEGER)?;

                execution_context.operand_stack.push(current_local)?;
            }

            bytecode::aaload => {
                let arrayref = execution_context.operand_stack.pop()?;
                let index = execution_context.operand_stack.pop()?;

                check_null_reference!(arrayref, execution_context);

                let associated_reference = execution_context
                    .object_manager
                    .get_object(index.value as usize);

                if let Ok(e) = associated_reference {
                    // consistency check to make sure it is an array
                    assert!(e.is_array());

                    assert!(e.is_of_type(constants::PrimitiveType::REFERENCE));

                    // retrieve value of the reference of the array
                    if let Ok(reference) = e.read_s((index.value * 2) as usize) {
                        execution_context.operand_stack.apush(reference)?;
                    }
                    else {
                        exceptions::throw_exception(
                            execution_context,
                            associated_reference.err().unwrap())?;
                    }

                    
                } else {
                    exceptions::throw_exception(
                        execution_context,
                        associated_reference.err().unwrap(),
                    )?;
                }
            }
            //bytecode::baload,          // 37
            //bytecode::saload,          // 38
            //bytecode::iaload,          // 39
            //bytecode::astore,          // 40
            //bytecode::sstore,          // 41
            //bytecode::istore,          // 42
            //bytecode::astore_0,        // 43
            //bytecode::astore_1,        // 44
            //bytecode::astore_2,        // 45
            //bytecode::astore_3,        // 46
            //bytecode::sstore_0,        // 47
            //bytecode::sstore_1,        // 48
            //bytecode::sstore_2,        // 49
            //bytecode::sstore_3,        // 50
            //bytecode::istore_0,        // 51
            //bytecode::istore_1,        // 52
            //bytecode::istore_2,        // 53
            //bytecode::istore_3,        // 54
            //bytecode::aastore,         // 55
            //bytecode::bastore,         // 56
            //bytecode::sastore,         // 57
            //bytecode::iastore,         // 58
            //bytecode::pop,             // 59
            //bytecode::pop2,            // 60
            //bytecode::dup,             // 61
            //bytecode::dup2,            // 62
            //bytecode::dup_x,           // 63
            //bytecode::swap_x,          // 64
            bytecode::sadd => {
                let value1 = execution_context.operand_stack.pop()?;
                let value2 = execution_context.operand_stack.pop()?;
                let res = value1.value + value2.value;
                execution_context.operand_stack.push(StackEntry::from_values(res,constants::PrimitiveType::SHORT))?;
            }
            // bytecode::iadd,            // 66
            // bytecode::ssub,            // 67
            // bytecode::isub,            // 68
            // bytecode::smul,            // 69
            // bytecode::imul,            // 70
            // bytecode::sdiv,            // 71
            // bytecode::idiv,            // 72
            // bytecode::srem,            // 73
            // bytecode::irem,            // 74
            // bytecode::sneg,            // 75
            // bytecode::ineg,            // 76
            // bytecode::sshl,            // 77
            // bytecode::ishl,            // 78
            // bytecode::sshr,            // 79
            // bytecode::ishr,            // 80
            // bytecode::sushr,           // 81
            // bytecode::iushr,           // 82
            // bytecode::sand,            // 83
            // bytecode::iand,            // 84
            // bytecode::sor,             // 85
            // bytecode::ior,             // 86
            // bytecode::sxor,            // 87
            // bytecode::ixor,            // 88
            // bytecode::sinc,            // 89
            // bytecode::iinc,            // 90
            // bytecode::s2b,             // 91
            // bytecode::s2i,             // 92
            // bytecode::i2b,             // 93
            // bytecode::i2s,             // 94
            // bytecode::icmp,            // 95
            // bytecode::ifeq,            // 96
            // bytecode::ifne,            // 97
            // bytecode::iflt,            // 98
            // bytecode::ifge,            // 99
            // bytecode::ifgt,            // 100
            // bytecode::ifle,            // 101
            // bytecode::ifnull,          // 102
            // bytecode::ifnonnull,       // 103
            // bytecode::if_acmpeq,       // 104
            // bytecode::if_acmpne,       // 105
            // bytecode::if_scmpeq,       // 106
            // bytecode::if_scmpne,       // 107
            // bytecode::if_scmplt,       // 108
            // bytecode::if_scmpge,       // 109
            // bytecode::if_scmpgt,       // 110
            // bytecode::if_scmple,       // 111
            // bytecode::goto,            // 112
            // bytecode::jsr,             // 113
            // bytecode::ret,             // 114
            // bytecode::stableswitch,    // 115
            // bytecode::itableswitch,    // 116
            // bytecode::slookupswitch,   // 117
            // bytecode::ilookupswitch,   // 118
            // bytecode::areturn,         // 119
            // bytecode::sreturn,         // 120
            // bytecode::ireturn,         // 121
            // bytecode::return_,         // 122
            // bytecode::getstatic_a,     // 123
            // bytecode::getstatic_b,     // 124
            // bytecode::getstatic_s,     // 125
            // bytecode::getstatic_i,     // 126
            // bytecode::putstatic_a,     // 127
            // bytecode::putstatic_b,     // 128
            // bytecode::putstatic_s,     // 129
            // bytecode::putstatic_i,     // 130
            // bytecode::getfield_a,      // 131
            // bytecode::getfield_b,      // 132
            // bytecode::getfield_s,      // 133
            // bytecode::getfield_i,      // 134
            // bytecode::putfield_a,      // 135
            // bytecode::putfield_b,      // 136
            // bytecode::putfield_s,      // 137
            // bytecode::putfield_i,      // 138
            // bytecode::invokevirtual,   // 139
            // bytecode::invokespecial,   // 140
            // bytecode::invokestatic,    // 141
            // bytecode::invokeinterface, // 142
            // bytecode::new,             // 143
            // bytecode::newarray,        // 144
            // bytecode::anewarray,       // 145
            // bytecode::arraylength,     // 146
            // bytecode::athrow,          // 147
            // bytecode::checkcast,       // 148
            // bytecode::instanceof,      // 149
            // bytecode::sinc_w,          // 150
            // bytecode::iinc_w,          // 151
            // bytecode::ifeq_w,          // 152
            // bytecode::ifne_w,          // 153
            // bytecode::iflt_w,          // 154
            // bytecode::ifge_w,          // 155
            // bytecode::ifgt_w,          // 156
            // bytecode::ifle_w,          // 157
            // bytecode::ifnull_w,        // 158
            // bytecode::ifnonnull_w,     // 159
            // bytecode::if_acmpeq_w,     // 160
            // bytecode::if_acmpne_w,     // 161
            // bytecode::if_scmpeq_w,     // 162
            // bytecode::if_scmpne_w,     // 163
            // bytecode::if_scmplt_w,     // 164
            // bytecode::if_scmpge_w,     // 165
            // bytecode::if_scmpgt_w,     // 166
            // bytecode::if_scmple_w,     // 167
            // bytecode::goto_w,          // 168
            // bytecode::getfield_a_w,    // 169
            // bytecode::getfield_b_w,    // 170
            // bytecode::getfield_s_w,    // 171
            // bytecode::getfield_i_w,    // 172
            // bytecode::getfield_a_this, // 173
            // bytecode::getfield_b_this, // 174
            // bytecode::getfield_s_this, // 175
            // bytecode::getfield_i_this, // 176
            // bytecode::putfield_a_w,    // 177
            // bytecode::putfield_b_w,    // 178
            // bytecode::putfield_s_w,    // 179
            // bytecode::putfield_i_w,    // 180
            // bytecode::putfield_a_this, // 181
            // bytecode::putfield_b_this, // 182
            // bytecode::putfield_s_this, // 183
            // bytecode::putfield_i_this, // 184

            _ => break,
        }
    }

    Ok(())
}
