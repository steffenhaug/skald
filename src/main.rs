extern crate pest;
#[macro_use]
extern crate pest_derive;

mod value;
mod error;
mod ast;
mod env;
mod pattern;
mod parsing;

fn main() {
    parsing::parse("test-identifier");
}
