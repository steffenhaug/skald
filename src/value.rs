// A `Value` is either primitive in which
// case it is just a simple type, or it is
// a tuple, which is a compound type.

use std::sync::Arc;

use crate::ast::Expression;
use crate::error::EvalResult;
use crate::env::Env;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Function(Applicative),
}

impl Value {
    pub fn get_applicative(&self) -> Option<Applicative> {
        // Extract the stored applicative procedure
        // if the value is a function type.
        //
        // An interesting idea here, is that other values
        // could be given applicatives, for example the
        // applicative of an array could be the indexing
        // function.
        match self {
            Value::Function(applicative) => Some(applicative.clone()),
            _ => None
        }
    }
}

// An Applicative is in this case simply something which
// can be applied to arguments, it has nothing to do with
// Haskell.
#[derive(Clone)]
pub enum Applicative {
    // Lambda abstraction.
    Lambda {
        params: Vec<String>,
        body: Box<Expression>,
        closure: Arc<Env>
    },
    // Built-in call to rust function.
    Primitive(&'static (dyn Fn(&[Value]) -> EvalResult + Sync))
}

impl Applicative {
    pub fn apply(&self, args: Vec<Value>) -> EvalResult {
        use crate::error::RuntimeError::*;
        match self {
            Applicative::Primitive(proc) => {
                // The easy case B)
                proc(&args)
            },
            Applicative::Lambda { params, body, closure } => {
                // Count the # of parameters and arguments.
                let nparams = params.len();
                let nargs = args.len();

                // Make sure they match.
                if nparams != nargs {
                    return Err(NumArgs { expected: nparams, found: nargs });
                }

                let bindings: Vec<(&String, Value)> = params.iter()
                    .zip(args.into_iter()).collect();

                let env = Env::inside(closure)
                    .bind(bindings)
                    .build();

                body.eval(&env)
            }
        }
    }
}

impl std::fmt::Debug for Applicative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Applicative::Lambda { params: _, body: _, closure: _ } => {
                f.write_str("lambda (...) ...")?;
            },
            Applicative::Primitive (_) => {
                f.write_str("<primitive operator>")?;
            }
        }
        Ok(())
    }
}

pub fn and(argv: &[Value]) -> EvalResult {
    use crate::error::RuntimeError::*;
    match argv {
        [Value::Boolean(x), Value::Boolean(y)] =>
            Ok(Value::Boolean(*x && *y)),
        [Value::Boolean(_), _] =>
            Err(TypeMismatch { argn: 1 }),
        [_,_] =>
            Err(TypeMismatch { argn: 0 }),
        _ =>
            Err(NumArgs { expected: 2, found: argv.len() })
    }
}
