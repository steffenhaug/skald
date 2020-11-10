use crate::value::Value;
use crate::strintern::Symbol;
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
    PVariable(Symbol),
    PTupleConstructor(Vec<Pattern>)
}

impl Pattern {
    pub fn matches(&self, value: &Value) -> Option<Vec<(Symbol, Value)>> {
        let mut bindings = Vec::new();

        if !self.find_bindings(value, &mut bindings) {
            return None;
        }

        Some(bindings)
    }

    fn find_bindings(&self, value: &Value, bindings: &mut Vec<(Symbol, Value)>) -> bool {
        use Pattern::*;
        match (self, value) {
            (PConstant(pval), val) => pval == val,
            (PVariable(sym),  val) => {
                bindings.push((*sym, val.clone()));
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

#[cfg(test)]
mod tests {
    use crate::value::Value::*;
    use crate::pattern::Pattern::*;
    use crate::strintern::Interner;

    #[test]
    fn match_tuple_pattern() {
        let mut interner = Interner::new();
        let x = interner.intern("x");
        
        let p = PTupleConstructor(vec![
            PConstant(Boolean(true)),
            PVariable(x)
        ]);

        let v = Tuple(vec![
            Boolean(true),
            Boolean(false),
        ]);

        let bindings = p.matches(&v);

        assert_eq!(bindings, Some(vec![(x, Boolean(false))]));
    }

    #[test]
    fn variable_pattern_produces_binding() {
        let mut interner = Interner::new();
        let x = interner.intern("x");
        
        let p = PVariable(x);
        let v = Boolean(true);

        let bindings = p.matches(&v);

        assert_eq!(bindings, Some(vec![(x, Boolean(true))]));
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
}
