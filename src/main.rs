mod value;
mod ast;
mod error;

use crate::value::Env;
use crate::value::Value::*;
use crate::value::Applicative;
use crate::ast::Expression::*;

fn main() {
    let global_scope = Env::global();
    global_scope.bind(vec![
        ("x", Boolean(false)),
        ("and", Function(Applicative::Primitive(&value::and)))
    ]);

    // >> ((lambda (x y) (and x y)) x y)
    // false
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

    println!("{:?}", program.eval(&global_scope));
}
