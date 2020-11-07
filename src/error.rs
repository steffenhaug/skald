use crate::value::Value;

#[derive(Debug)]
pub enum RuntimeError {
    Undefined { identifier: String },
    NotApplicative,
    NumArgs { expected: usize, found: usize },
    TypeMismatch { argn: usize }
}

pub type EvalResult = Result<Value, RuntimeError>;
