mod value;
mod ast;
mod error;
mod env;

fn main() {
    use crate::value::Value::*;
    use crate::ast::Pattern::*;

    let p = PTuple(vec![
        PConstant(Boolean(true)),
        PVariable("x".to_string()),
        PConstant(Boolean(false)),
    ]);

    let v = Tuple(vec![
        Boolean(true),
        Boolean(true),
        Boolean(false),
    ]);

    let bindings = p.matches(&v);

    println!("{:?}", bindings);
                       
}

