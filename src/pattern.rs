use crate::value::Value;
// The pattern language is made up of
// syntax to destructure the built-in
// types of the interpreter.
//
// They need to be their own magical little
// separate language because down the line
// i will add pattern guards.
//
// Naming is a bit weird to not confuse
// patterns with expressions. Might change.
#[derive(Clone, Debug)]
pub enum Pattern {
    PConstant(Value),
    PVariable(String),
    PTupleConstructor(Vec<Pattern>)
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
        match (self, value) {
            (PConstant(pval), val) => pval == val,
            (PVariable(sym),  val) => {
                bindings.push((String::from(sym), val.clone()));
                true
            },
            (PTupleConstructor(nested_patterns), Value::Tuple(nested_values)) => {
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
