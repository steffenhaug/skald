

use std::collections::HashMap;

pub struct Interner {
    names: Vec<*const str>,
    dictionary: HashMap<String, usize>
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Symbol(usize);

impl Interner {
    pub fn new() -> Interner {
        Interner {
            names: Vec::new(),
            dictionary: HashMap::new()
        }
    }
    
    pub fn intern(&mut self, sym: &str) -> Symbol {
        if let Some(&idx) = self.dictionary.get(sym) {
            return Symbol(idx);
        }

        let idx = self.names.len();
        let key = String::from(sym);
        // Danger!
        let ptr: *const str = &(*key);

        self.dictionary.insert(key, idx);
        self.names.push(ptr);

        Symbol(idx)
    }

    pub fn lookup(&self, sym: Symbol) -> &str {
        unsafe { self.names[sym.0] }
    }
}

#[cfg(test)]
mod tests {
    use crate::strintern::{Interner, Symbol};

    #[test]
    fn intern_equal_strings_are_equal() {
        let mut interner = Interner::new();
        let x1: Symbol = interner.intern(&String::from("x"));
        let x2: Symbol = interner.intern("x");
        assert_eq!(x1, x2);
        // Symbols reference *the exact same string*.
        assert_eq!(interner.lookup(x1).as_ptr(), interner.lookup(x2).as_ptr());
    }
}
