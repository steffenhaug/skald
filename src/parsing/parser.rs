use pest::Parser;

#[derive(Parser)]
#[grammar = "parsing/skald.pest"]
struct SkaldParser;

pub fn parse(src: &str) {
    let pairs = SkaldParser::parse(Rule::skald, src)
        .expect("failed parsing");
    println!("{:?}", pairs);
}
