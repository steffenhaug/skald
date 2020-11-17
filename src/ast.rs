use crate::value::{Primitive, Value, Applicative};
use crate::error::EvalResult;
use crate::env::Env;
use crate::pattern::Pattern;

use std::sync::Arc;


// The AST is composed of nested expressions.
// Various types of expressions exists.

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(Primitive),
    Variable(String),
    TupleConstructor(Vec<Expression>),
    Application { fun: Box<Expression>, expr_argv: Vec<Expression> },
    Abstraction { params: Vec<String>, body: Box<Expression> },
    Match { term: Box<Expression>, clauses: Vec<(Pattern, Expression)> },
}

impl Expression {
    pub fn eval(&self, env: &Arc<Env>) -> EvalResult {

        use Expression::*;
        use crate::error::RuntimeError::{
            Undefined,
            NotApplicative,
            UnmatchedPattern
        };

        match self {
            // Constants evaluate to their contained value.
            Constant(primitive) => Ok(Value::Simple(primitive.clone())),
            // Variables evaluate to the value associated with it.
            Variable(sym) => {
                env.get(sym).ok_or(
                    Undefined { identifier: String::from(sym) }
                )
            },
            Application { fun, expr_argv } => {
                // Obtain the function (as an applicative) by
                // evaluating the head of the list.
                ////let func = list[0].eval(env)?;
                let applicative = fun.eval(env)?;

                // Extract the stored procedure.
                let proc =
                    if let Some(proc) = applicative.get_applicative() {
                        proc
                    } else {
                        return Err(NotApplicative);
                    };

                // Evaluate the arguments.
                let mut argv = Vec::new();

                // Note the edge case where list has lengt
                // one. Then list[1..] still works fine,
                // and is an empty slice. No error!
                for expr in expr_argv {
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
                // Note that the environment is exactly the same,
                // so evaluation will reflect changes to the
                // enclosing environment after definition.

                // Construct the function.
                Ok(Value::Function(Applicative::Lambda {
                    params: params.to_vec(),
                    body: body.clone(),
                    closure
                }))
            },
            TupleConstructor(exprs) => {
                let mut elements = Vec::new();

                for expr in exprs {
                    let element = expr.eval(env)?;
                    elements.push(element);
                }

                Ok(Value::Tuple(elements))
            },
            Match { term, clauses } => {
                // Evaluate the term to match.
                let value = term.eval(&env)?;
                
                for (pattern, expr) in clauses {
                    // Test the pattern.
                    if let Some(bindings) = pattern.matches(&value) {
                        // If the pattern matches the value, create
                        // a new scope with the the bindings the
                        // match produced.
                        let scope = Env::inside(&env)
                            .bind(bindings)
                            .build();

                        // Evaluate the corresponding match arm in
                        // the new scope.
                        return expr.eval(&scope);
                    }
                }

                // No pattern matched.
                Err(UnmatchedPattern { found: value.clone() })
            }
        }
    }
}
