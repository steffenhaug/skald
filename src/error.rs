use crate::value::Value;
use crate::strintern::Symbol;

#[derive(Debug)]
pub enum RuntimeError {
    Undefined { identifier: Symbol },
    NotApplicative,
    NumArgs { expected: usize, found: usize },
    TypeMismatch { argn: usize },
    UnmatchedPattern { found: Value },
}

pub type EvalResult = Result<Value, RuntimeError>;
