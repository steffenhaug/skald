use crate::value::{Value, Applicative};
use crate::error::EvalResult;
use crate::env::Env;
use crate::pattern::Pattern;
use crate::strintern::Symbol;

use std::sync::Arc;


// The AST is composed of nested expressions.
// Various types of expressions exists.

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(Value),
    Variable(Symbol),
    TupleConstructor(Vec<Expression>),
    Application(Vec<Expression>),
    Abstraction { params: Vec<Symbol>, body: Box<Expression> },
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
            Constant(val) => Ok(val.clone()),
            // Variables evaluate to the value associated with it.
            Variable(sym) => {
                env.get(sym).ok_or(
                    Undefined { identifier: *sym }
                )
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
                let mut argv = Vec::new();

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


#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::ast::Expression::*;
    use crate::value::Value;
    use crate::value::Value::*;
    use crate::value::Applicative;
    use crate::env::Env;
    use crate::error::RuntimeError::*;
    use crate::error::EvalResult;
    use crate::env::*;
    use crate::pattern::Pattern::*;
    use crate::strintern::{Interner,Symbol};

    #[test]
    fn pattern_matching_expression() {
        // # A (xor a b) expression
        //
        // match (x, y) with
        //   (true,  false) -> true
        //   (true,  true)  -> false
        //   (false, false) -> false
        //   (false, true)  -> true
        let mut identifiers = Interner::new();
        let x: Symbol = identifiers.intern("x");
        let y: Symbol = identifiers.intern("y");

        let program = Match {
            term: Box::new(Expression::TupleConstructor(vec![
                Expression::Variable(x),
                Expression::Variable(y)
            ])),
            clauses: vec![
                (PTupleConstructor(vec![
                    PConstant(Boolean(true)),
                    PConstant(Boolean(false))
                ]), Constant(Boolean(true))),
                (PTupleConstructor(vec![
                    PConstant(Boolean(true)),
                    PConstant(Boolean(true))
                ]), Constant(Boolean(false))),
                (PTupleConstructor(vec![
                    PConstant(Boolean(false)),
                    PConstant(Boolean(false))
                ]), Constant(Boolean(false))),
                (PTupleConstructor(vec![
                    PConstant(Boolean(false)),
                    PConstant(Boolean(true))
                ]), Constant(Boolean(true))),
            ]
        };

        // let x = false
        // let y = true
        let env = Env::inside(&GLOBAL)
            .bind_one(x, Boolean(false))
            .bind_one(y, Boolean(true))
            .build();

        // As expected, x (xor) y is true.
        let result = program.eval(&env).unwrap();
        assert_eq!(result, Boolean(true));
    }

    #[test]
    fn apply_abstraction() {
        // Tests all the basic shit by evaluating an AST
                       
        // with constants, identifiers, abstractions and
        // function application of abstractions as well
        // as primitive operations.
        fn bin_op_and(argv: &[Value]) -> EvalResult {
            match argv {
                [Boolean(x), Boolean(y)] => Ok(Boolean(*x && *y)),
                [Boolean(_), _]          =>
                    Err(TypeMismatch { argn: 1 }),
                [_,_]                    =>
                    Err(TypeMismatch { argn: 0 }),
                _ => Err(NumArgs { expected: 2, found: argv.len() })
            }
        }

        let mut identifiers = Interner::new();
        let x: Symbol = identifiers.intern("x");
        let y: Symbol = identifiers.intern("y");
        let and: Symbol = identifiers.intern("and");

        // let x = false
        // let and = extern and
        let scope = Env::inside(&GLOBAL)
            .bind_one(x, Boolean(false))
            .bind_one(and, Function(Applicative::Primitive(&bin_op_and)))
            .build();

        // ((lambda (x y) (and x y)) x true)
        let program = Application(vec![
            Abstraction {
                params: vec![x, y],
                body: Box::new(Application(vec![
                    Variable(and),
                    Variable(x),
                    Variable(y),
                ]))
            },
            Variable(x),
            Constant(Boolean(true)),
        ]);

        let result: Value = program.eval(&scope).unwrap();
        assert_eq!(result, Boolean(false));
    }
}
