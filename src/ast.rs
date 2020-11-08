use crate::value::{Value, Applicative};
use crate::error::EvalResult;
use crate::env::Env;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(Value),
    Identifier(String),
    Application(Vec<Expression>),
    Abstraction { params: Vec<String>, body: Box<Expression> },
    TupleConstructor(Vec<Expression>),
}

impl Expression {
    pub fn eval(&self, env: &Arc<Env>) -> EvalResult {
        use Expression::*;
        use Value::*;
        use crate::error::RuntimeError::*;
        match self {
            Constant(val) => {
                // Constants evaluate to their contained value.
                Ok(val.clone())
            },
            Identifier(name) => {
                // Identifiers evaluate to the value stored in
                // the innermost 
                env.get(name)
                    .ok_or(Undefined { identifier: String::from(name) })
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
                Ok(Function(Applicative::Lambda {
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
            }
        }
    }
}

// The pattern language is a limited
// version of the real language.
pub enum Pattern {
    PConstant(Value),
    PVariable(String),
    PTuple(Vec<Pattern>)
}

impl Pattern {
    pub fn matches(&self, value: &Value) -> Option<Vec<(String, Value)>> {
        let mut bindings = Vec::new();

        if !self.find_bindings(value, &mut bindings) {
            return None;
        }

        Some(bindings)
    }

    fn find_bindings(&self, value: &Value, bindings: &mut Vec<(String, Value)>) -> bool {
        use Pattern::*;
        use Value::*;
        match (self, value) {
            (PConstant(pval), val) => pval == val,
            (PVariable(name), val) => {
                bindings.push((name.to_string(), val.clone()));
                true
            },
            (PTuple(nested_patterns), Tuple(nested_values)) => {
                if nested_patterns.len() != nested_values.len() {
                    return false;
                }

                for (p, v) in nested_patterns.iter().zip(nested_values.iter()) {
                    if !p.find_bindings(v, bindings) {
                        return false;
                    }
                }

                true
            }
            _ => false
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::ast::Expression::*;
    use crate::ast::Pattern::*;
    use crate::value::Applicative::*;
    use crate::value::Value::*;
    use crate::value::Value;
    use crate::env::Env;
    use crate::error::RuntimeError::*;
    use crate::error::EvalResult;
    use crate::env::*;

    #[test]
    fn variable_pattern_produces_binding() {
        let p = PVariable("x".to_string());
        let v = Boolean(true);

        let bindings = p.matches(&v);

        assert_eq!(bindings, Some(vec![("x".to_string(), Boolean(true))]));
    }

    #[test]
    fn constant_matches_equal_value() {
        let p = PConstant(Boolean(false));
        let v = Boolean(false);

        let bindings = p.matches(&v);

        assert_eq!(bindings, Some(vec![]));
    }

    #[test]
    fn constant_match_fails_diff_value() {
        let p = PConstant(Boolean(false));
        let v = Boolean(true);

        let bindings = p.matches(&v);

        assert_eq!(bindings, None);
    }

    #[test]
    fn apply_abstraction() {
        // Tests all the basic shit by evaluating an AST
                       
        // with constants, identifiers, abstractions and
        // function application of abstractions as well
        // as primitive operations.
        fn and(argv: &[Value]) -> EvalResult {
            match argv {
                [Boolean(x), Boolean(y)] => Ok(Boolean(*x && *y)),
                [Boolean(_), _]          =>
                    Err(TypeMismatch { argn: 1 }),
                [_,_]                    =>
                    Err(TypeMismatch { argn: 0 }),
                _ => Err(NumArgs { expected: 2, found: argv.len() })
            }
        }

        let scope = Env::inside(&GLOBAL)
            .bind_one("x", Boolean(false))
            .bind_one("and", Function(Primitive(&and)))
            .build();

        let program = Application(vec![
            Abstraction {
                params: vec!["x".to_string(), "y".to_string()],
                body: Box::new(Application(vec![
                    Identifier("and".to_string()),
                    Identifier("x".to_string()),
                    Identifier("y".to_string()),
                ]))
            },
            Identifier("x".to_string()),
            Constant(Boolean(true)),
        ]);

        let result: Value = program.eval(&scope).unwrap();
        assert_eq!(result, Boolean(false));
    }

    #[test]
    fn create_tuple() {
        let scope = Env::inside(&GLOBAL)
            .bind_one("x", Boolean(false))
            .bind_one("y", Boolean(true))
            .build();

        let program = TupleConstructor(vec![
            Identifier("x".to_string()),
            Identifier("y".to_string()),
        ]);

        let result: Value = program.eval(&scope).unwrap();
        assert_eq!(result, Tuple(vec![Boolean(false), Boolean(true)]));
    }
}
