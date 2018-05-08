use bytecodes::bytecode;
use context::Context;
use stack::StackEntry;
use constants;
use exceptions::{throw_exception, throw_exception_from_interpretererror, InterpreterException};
use traits::{BufferAccessor, HasType};

// macro allowing to simplify null reference check
#[macro_export]
macro_rules! check_null_reference {
    ($variable: ident, $ctx: ident) => {
        if !$variable.is_of_type(constants::PrimitiveType::REFERENCE)
            || $variable.value == constants::NULL_HANDLE
        {
            return throw_exception($ctx, InterpreterException::NullPointerException);
        }
    };
}

///
/// Manages aaload, baload, saload, iaload
///
pub fn xaload(
    execution_context: &mut Context,
    type_: constants::PrimitiveType,
) -> Result<(), InterpreterException> {
    let arrayref = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::REFERENCE)
        .unwrap();
    let index = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::SHORT)
        .unwrap();

    check_null_reference!(arrayref, execution_context);

    let associated_reference = execution_context
        .object_manager
        .get_object(arrayref.value as usize);

    if let Ok(e) = associated_reference {
        // consistency check to make sure it is an array
        assert!(e.is_array());

        // in case of arrays, the primitive type represents the type of its elements
        assert!(e.is_of_type(type_));

        match type_ {
            // for short and references, we perform thee same type of checks and
            // fetch the array identically (2 by 2)
            constants::PrimitiveType::SHORT | constants::PrimitiveType::REFERENCE => {
                let size_one_entry = constants::REFERENCE_SIZE;
                match e.read_s((index.value as usize) * size_one_entry) {
                    // REFERENCE_SIZE == SHORT_SIZE here
                    Ok(res) => {
                        execution_context
                            .operand_stack
                            .push(StackEntry::from_values(res, type_));
                    }
                    Err(e) => {
                        return throw_exception_from_interpretererror(execution_context, e);
                    }
                }
            }
            // for bytes, each entry is one byte long
            constants::PrimitiveType::BYTE => {
                let size_one_entry = constants::BYTE_SIZE;
                match e.read_b((index.value as usize) * size_one_entry) {
                    // retrieve v(alue of the reference of the array
                    Ok(res) => {
                        execution_context.operand_stack.bpush(res);
                    }
                    Err(e) => return throw_exception_from_interpretererror(execution_context, e),
                }
            }

            // or integers readings are performed 4 bytes by 4 bytes
            constants::PrimitiveType::INTEGER => {
                let size_one_entry = constants::INTEGER_SIZE;
                match e.read_i((index.value as usize) * size_one_entry) {
                    // retrieve value of the reference of the array
                    Ok(res) => {
                        execution_context.operand_stack.ipush(res);
                    }
                    Err(e) => return throw_exception_from_interpretererror(execution_context, e),
                }
            }

            constants::PrimitiveType::UNKNOWN => {
                panic!("Unknown type !");
            }
        }
    } else {
        return throw_exception(execution_context, associated_reference.err().unwrap());
    }

    Ok(())
}

///
/// Manages astore, sstore, istore and assoiated xstore_x (because index is passed as parameter)
///
pub fn xstore(execution_context: &mut Context, index: u8, type_: constants::PrimitiveType) {
    match type_ {
        // storing shorts and references follow the same pattern
        constants::PrimitiveType::SHORT | constants::PrimitiveType::REFERENCE => {
            // pop and check the type loaded from stack
            let value_to_put = execution_context
                .operand_stack
                .pop_check_type(type_)
                .unwrap();
            //update local variable
            execution_context
                .current_frame_mut()
                .unwrap()
                .set_local(index as i16, value_to_put)
                .unwrap();
        }
        // for integers, we pop and check 2 times on the stack
        constants::PrimitiveType::INTEGER => {
            let value_to_put1 = execution_context
                .operand_stack
                .pop_check_type(type_)
                .unwrap();

            let value_to_put2 = execution_context
                .operand_stack
                .pop_check_type(type_)
                .unwrap();
            // ... and we update 2 indexes in local variables stack
            execution_context
                .current_frame_mut()
                .unwrap()
                .set_local(index as i16, value_to_put1)
                .unwrap();

            execution_context
                .current_frame_mut()
                .unwrap()
                .set_local((index + 1) as i16, value_to_put2)
                .unwrap();
        }

        _ => panic!("Unknown type"),
    };
}

///
/// Manages astore, sstore, istore and assoiated xstore_x (because index is passed as parameter)
/// Note: for aaload, some supplementary checks are performed to ensure consistency of the operaton
/// See chapter 7.5.2 from JCVM specification for more details
///
pub fn xastore(
    execution_context: &mut Context,
    type_: constants::PrimitiveType,
) -> Result<(), InterpreterException> {
    // in stack:
    // array ref
    // index
    // value
    // first, pop the array reference and check it is not null
    let array_ref = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::REFERENCE)
        .unwrap();

    check_null_reference!(array_ref, execution_context);

    // make sure it is an array of the correct type
    let array = execution_context
        .object_manager
        .get_object(array_ref.value as usize)
        .unwrap();

    // check that is is really an array
    if !array.is_array() || !array.is_of_type(type_) {
        return Err(InterpreterException::SecurityException);
    }

    let index = execution_context
        .operand_stack
        .pop_check_type(constants::PrimitiveType::SHORT)
        .unwrap();

    /*let value = execution_context
        .operand_stack
        .pop_check_type(type_)
        .unwrap_or(return Err(InterpreterException::SecurityException));*/

    Ok(())
}
