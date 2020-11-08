use crate::value::{Value, Applicative};
use crate::error::EvalResult;
use crate::env::Env;
use Value::*;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Value),
    Identifier(String),
    Application(Vec<Expression>),
    Abstraction { params: Vec<String>, body: Box<Expression> }, // (lambda (params) body)
}

impl Expression {
    pub fn eval(&self, env: &Arc<Env>) -> EvalResult {
        use Expression::*;
        use crate::error::RuntimeError::*;
        match self {
            Constant(val) => {
                // Constants evaluate to their contained value.
                Ok(val.clone())
            },
            Identifier(name) => {
                // Identifiers evaluate to the value stored in
                // the innermost 
                env.get(name).ok_or(Undefined { identifier: String::from(name) })
            },
            Application(list) => {
                // Obtain the function (as an applicative) by
                // evaluating the head of the list.
                let func = list[0].eval(env)?;

                // Extract the stored procedure.
                let proc =
                    if let Some(applicative) = func.get_applicative() {
                        applicative
                    } else {
                        return Err(NotApplicative);
                    };

                // Evaluate the arguments.
                let mut argv: Vec<Value> = Vec::new();

                // Note the edge case where list has lengt
                // one. Then list[1..] still works fine,
                // and is an empty slice. No error!
                for expr in list[1..].iter() {
                    let arg = expr.eval(env)?;
                    argv.push(arg);
                }

                // Finally apply the procedure to the argument vector.
                proc.apply(argv)
            },
            Abstraction { params, body } => {
                // Create a new environment that includes
                // references to the scope in which the function
                // was defined in its parent.
                let closure = Arc::clone(env);

                // Construct the function.
                Ok(Function(Applicative::Lambda {
                    params: params.to_vec(),
                    body: body.clone(),
                    closure
                }))
            }
            
        }
    }
}
