// A `Value` is either primitive in which
// case it is just a simple type, or it is
// a tuple, which is a compound type.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::Expression;
use crate::error::EvalResult;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Atom(String),
    Tuple(Compound),
    Function(Applicative),
}

impl Value {
    pub fn get_proc(&self) -> Option<Applicative> {
        // Extract the stored applicative procedure
        // if the value is a function type.
        match self {
            Value::Function(applicative) => Some(applicative.clone()),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Compound {
    values: Vec<Value>
}

impl Compound {
    fn arity(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    bindings: RefCell<HashMap<String, Value>>,
    parent: Option<Rc<Env>>
}

impl Env {
    pub fn inside(outer: &Rc<Env>) -> Rc<Env> {
        Rc::new(Env {
            bindings: RefCell::new(HashMap::new()),
            parent: Some(Rc::clone(outer))
        })
    }

    pub fn global() -> Rc<Env> {
        Rc::new(Env {
            bindings: RefCell::new(HashMap::new()),
            parent: None
        })
    }

    pub fn get(&self, identifier: &str) -> Option<Value> {
        match self.bindings.borrow().get(identifier) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                None => None,
                Some(parent) => parent.get(identifier)
            }
        }
    }

    pub fn bind(&self, bindings: Vec<(&str, Value)>) {
        for (identifier, value) in bindings {
            self.bind_one(identifier, value);
        }
    }

    pub fn bind_one(&self, identifier: &str, value: Value) {
        self.bindings
            .borrow_mut()
            .insert(identifier.to_string(), value);
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
        closure: Rc<Env>
    },
    // Built-in call to rust function.
    Primitive(&'static dyn Fn(&[Value]) -> EvalResult)
}

impl Applicative {
    pub fn apply(&self, args: &[Value]) -> EvalResult {
        use crate::error::RuntimeError::*;
        match self {
            Applicative::Primitive(proc) => {
                // The easy case B)
                proc(args)
            },
            Applicative::Lambda { params, body, closure } => {
                // Count the # of parameters and arguments.
                let nparams = params.len();
                let nargs = args.len();

                // Make sure they match.
                if nparams != nargs {
                    return Err(NumArgs { expected: nparams, found: nargs });
                }

                // yikes
                let bindings: Vec<(&str, Value)> = params.iter()
                    .map(|strref| &strref[..])
                    .zip(args.iter().map(|vref| vref.clone())).collect();

                closure.bind(bindings);
                body.eval(&closure)
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
