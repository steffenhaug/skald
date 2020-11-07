use crate::value::{Value, Applicative, Env};
use crate::error::EvalResult;
use Value::*;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Value),
    Identifier(String),
    Application(Vec<Expression>),
    Abstraction { params: Vec<String>, body: Box<Expression> }, // (lambda (params) body)
}

impl Expression {
    pub fn eval(&self, env: &Rc<Env>) -> EvalResult {
        use Expression::*;
        use crate::error::RuntimeError::*;
        match self {
            Constant(val) => Ok(val.clone()),
            Identifier(name) => {
                // Get the value from the environment.
                env.get(name).ok_or(Undefined { identifier: String::from(name) })
            },
            Application(list) => {
                // Obtain the function (as an applicative) by
                // evaluating the head of the list.
                let func = list[0].eval(env)?;

                // Extract the stored procedure.
                let proc =
                    if let Some(applicative) = func.get_proc() {
                        applicative
                    } else {
                        return Err(NotApplicative);
                    };

                // Evaluate the arguments.
                // Each argument is an expression, which
                // needs to be evaluated before we can apply
                // the function.
                // Note the edge case where the application
                // takes no arguments. If list has only one
                // element, list[1..] is actually an empty
                // slice.
                let mut argv: Vec<Value> = Vec::new();
                for expr in list[1..].iter() {
                    let arg = expr.eval(env)?;
                    argv.push(arg);
                }

                proc.apply(&argv)
            },
            Abstraction { params, body } => {
                let closure = Env::inside(env);
                Ok(Function(Applicative::Lambda {
                    params: params.to_vec(),
                    body: body.clone(),
                    closure: closure
                }))
            }
            
        }
    }
}
