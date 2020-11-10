mod value;
mod error;
mod ast;
mod env;
mod pattern;
mod strintern;

fn main() {
    use crate::strintern::{Interner, Symbol};
    let mut interner = Interner::new();
    let x: Symbol = interner.intern("x");
    println!("{:?}", interner.lookup(x));
}
