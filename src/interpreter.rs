use bcutils;
use bytecodes::bytecode;
use context::Context;
use stack::{StackEntry, StackEntryType};
use jcvmerrors::InterpreterError;
use constants;
use exceptions;

// macro allowing to simplify null reference check
macro_rules! check_null_reference {
    ($variable:ident, $ctx:ident) => (
        if !$variable.is_of_type(StackEntryType::reference) || $variable.value == constants::NULL_HANDLE {
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
            bytecode::opcode_nop => {
                println!(
                    "NOP bytecode reached at offset 0x{:X}",
                    execution_context.bytecode_fetcher.current_offset()
                );
                panic!("Unexpected NOP bytecode !");
            }
            // bytecode 1 : ACONST_NULL
            bytecode::opcode_aconst_null => {
                try!(
                    execution_context
                        .variables_stack
                        .apush(constants::NULL_HANDLE)
                );
            }

            // bytecode 2 : SCONST_M1
            bytecode::opcode_sconst_m1 => {
                execution_context.variables_stack.spush(-1 as i16)?;
            }

            // bytecode 3: SCONST_0
            bytecode::opcode_sconst_0 => execution_context.variables_stack.spush(0)?,
            // bytecode 4: SCONST_1
            bytecode::opcode_sconst_1 => execution_context.variables_stack.spush(1)?,
            // bytecode 5: SCONST_2
            bytecode::opcode_sconst_2 => execution_context.variables_stack.spush(2)?,
            // bytecode 6: SCONST_3
            bytecode::opcode_sconst_3 => execution_context.variables_stack.spush(3)?,
            // bytecode 7: SCONST_4
            bytecode::opcode_sconst_4 => execution_context.variables_stack.spush(4)?,
            // bytecode 8: SCONST_5
            bytecode::opcode_sconst_5 => execution_context.variables_stack.spush(5)?,
            // bytecode 9: ICONST_M1
            bytecode::opcode_iconst_m1 => execution_context.variables_stack.ipush(-1)?,
            // bytecode 10: ICONST_0
            bytecode::opcode_iconst_0 => execution_context.variables_stack.ipush(0)?,
            // bytecode 11: ICONST_1
            bytecode::opcode_iconst_1 => execution_context.variables_stack.ipush(1)?,
            // bytecode 12: ICONST_2
            bytecode::opcode_iconst_2 => execution_context.variables_stack.ipush(2)?,
            // bytecode 13: ICONST_3
            bytecode::opcode_iconst_3 => execution_context.variables_stack.ipush(3)?,
            // bytecode 14: ICONST_4
            bytecode::opcode_iconst_4 => execution_context.variables_stack.ipush(4)?,
            // bytecode 15: ICONST_5
            bytecode::opcode_iconst_5 => execution_context.variables_stack.ipush(5)?,
            // bytecode 16: BSPUSH
            bytecode::opcode_bspush => execution_context
                .variables_stack
                .spush(execution_context.bytecode_fetcher.fetch_b()? as i16)?,
            // bytecode 17: SSPUSH
            bytecode::opcode_sspush => execution_context
                .variables_stack
                .spush(execution_context.bytecode_fetcher.fetch_s()? as i16)?,
            // bytecode 18: BIPUSH
            bytecode::opcode_bipush => execution_context
                .variables_stack
                .ipush(execution_context.bytecode_fetcher.fetch_b()? as i32)?,
            // bytecode 19: SIPUSH
            bytecode::opcode_sipush => execution_context
                .variables_stack
                .ipush(execution_context.bytecode_fetcher.fetch_s()? as i32)?,
            // bytecode 20: IIPUSH
            bytecode::opcode_iipush => execution_context
                .variables_stack
                .ipush(execution_context.bytecode_fetcher.fetch_i()? as i32)?,
            // bytecode 21: ALOAD
            bytecode::opcode_aload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // read local from current frame
                let current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(index, StackEntryType::reference)?;

                execution_context.variables_stack.push(current_local)?;
            }

            // bytecode 22: SLOAD
            bytecode::opcode_sload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // read local from current frame
                let current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(index, StackEntryType::short)?;

                execution_context.variables_stack.push(current_local)?;
            }

            // bytecode 23: ILOAD
            bytecode::opcode_iload => {
                let index = execution_context.bytecode_fetcher.fetch_b()?;
                // note: the int takes 2 slots in the VM
                // we have to read those 2 variables and put them on the stack
                // read local from current frame
                let current_local1 = execution_context
                    .current_frame()?
                    .get_local_check_type(index, StackEntryType::int)?;
                let current_local2 = execution_context
                    .current_frame()?
                    .get_local_check_type(index + 1, StackEntryType::int)?;

                // push variables in reverse order to keep the original order
                execution_context.variables_stack.push(current_local2)?;
                execution_context.variables_stack.push(current_local1)?;
            }

            // bytecode 24...27: ALOAD_0...ALOAD_3
            bytecode::opcode_aload_0 |
            bytecode::opcode_aload_1 |
            bytecode::opcode_aload_2 |
            bytecode::opcode_aload_3 => {
                // read local from current frame
                let current_local = execution_context.current_frame()?.get_local_check_type(
                    current_opcode as u8 - bytecode::opcode_aload_0 as u8,
                    StackEntryType::reference,
                )?;

                execution_context.variables_stack.push(current_local)?;
            }

            // bytecode 28...31: SLOAD_0...SLOAD_3
            bytecode::opcode_sload_0 |
            bytecode::opcode_sload_1 |
            bytecode::opcode_sload_2 |
            bytecode::opcode_sload_3 => {
                // read local from current frame
                let current_local = execution_context.current_frame()?.get_local_check_type(
                    current_opcode as u8 - bytecode::opcode_aload_0 as u8,
                    StackEntryType::short,
                )?;

                execution_context.variables_stack.push(current_local)?;
            }

            // bytecode 32...35: ILOAD_0...ILOAD_3
            bytecode::opcode_iload_0 |
            bytecode::opcode_iload_1 |
            bytecode::opcode_iload_2 |
            bytecode::opcode_iload_3 => {
                // read local from current frame
                let current_opcode_val = current_opcode as u8 - bytecode::opcode_iload_0 as u8;
                let mut current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(current_opcode_val, StackEntryType::int)?;

                execution_context.variables_stack.push(current_local)?;

                current_local = execution_context
                    .current_frame()?
                    .get_local_check_type(current_opcode_val + 1, StackEntryType::int)?;

                execution_context.variables_stack.push(current_local)?;
            }

            bytecode::opcode_aaload => {
                let arrayref = execution_context.variables_stack.pop()?;
                let index = execution_context.variables_stack.pop()?;

                check_null_reference!(arrayref, execution_context);

                let associatedReference = execution_context
                    .object_manager
                    .get_object(index.value as usize);

                if let Ok(e) = associatedReference {
                    // consistency check to make sure it is an array
                    assert_eq!(
                        e.flags() & (constants::ObjectFlags::ARRAY as u8),
                        constants::ObjectFlags::ARRAY as u8
                    );

                    // retrieve value of the reference of the array
                    // TODO
                    panic!("implementation not finished !")
                } else {
                    exceptions::throw_exception(
                        execution_context,
                        associatedReference.err().unwrap(),
                    )?;
                }


                // TODO: implement handle table ? heap ?
            }

            _ => break,
        }
    }

    Ok(())
}
