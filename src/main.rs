mod value;
mod ast;
mod error;
mod env;

use crate::value::Value::*;
use crate::value::Applicative;
use crate::ast::Expression::*;
use crate::env::Env;

fn main() {
    let scope = Env::inside(&env::GLOBAL)
        .bind_one("x", Boolean(false))
        .bind_one("and", Function(Applicative::Primitive(&value::and)))
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

    println!("{:?}", program.eval(&scope));
}
