use crate::vm::analysis::read_only_checker::check_argument_count;
use crate::vm::analysis::type_checker::contexts::TypingContext;
use crate::vm::analysis::type_checker::{TypeChecker, TypeResult};
use crate::vm::analysis::CheckError;
use crate::vm::types::{BufferLength, SequenceSubtype, TypeSignature};
use crate::vm::SymbolicExpression;

pub fn check_special_to_consensus_buff(
    checker: &mut TypeChecker,
    args: &[SymbolicExpression],
    context: &TypingContext,
) -> TypeResult {
    check_argument_count(1, args)?;
    let input_type = checker.type_check(&args[0], context)?;
    let buffer_max_len = BufferLength::try_from(input_type.max_serialized_size()?)?;
    TypeSignature::new_option(TypeSignature::SequenceType(SequenceSubtype::BufferType(
        buffer_max_len,
    )))
    .map_err(CheckError::from)
}

pub fn check_special_from_consensus_buff(
    checker: &mut TypeChecker,
    args: &[SymbolicExpression],
    context: &TypingContext,
) -> TypeResult {
    check_argument_count(2, args)?;
    let result_type = TypeSignature::parse_type_repr(&args[0], checker)?;
    checker.type_check_expects(&args[1], context, &TypeSignature::max_buffer())?;
    Ok(result_type)
}
