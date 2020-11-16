// A `Value` is either primitive in which
// case it is just a simple type, or it is
// a tuple, which is a compound type.

use std::sync::Arc;

use crate::ast::Expression;
use crate::error::EvalResult;
use crate::env::Env;

#[derive(Clone, Debug)]
pub enum Value {
    Simple(Primitive),
    Function(Applicative),
    Tuple(Vec<Value>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Primitive {
    Boolean(bool),
}

impl Value {
    pub fn get_applicative(&self) -> Option<&Applicative> {
        // Extract the stored applicative procedure.
        //
        // An interesting idea here, is that other values
        // could be given applicatives, for example the
        // applicative of an array could be the indexing
        // function.
        match self {
            Value::Function(applicative) => Some(applicative),
            _ => None
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Simple(x), Simple(y)) => x == y,
            (Tuple(p), Tuple(q))   => p == q,
            _ => false
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
    Primitive(&'static (dyn Fn(&[Value]) -> EvalResult + Send + Sync))
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
                    return Err(NumArgs {
                        expected: nparams, found: nargs
                    });
                }

                let bindings: Vec<(String, Value)> = params.iter()
                    .map(|sym|  String::from(sym))
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
