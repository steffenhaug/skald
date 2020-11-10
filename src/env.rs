use std::collections::HashMap;
use std::sync::Arc;

use crate::value::Value;
use crate::strintern::Symbol;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Env {
    bindings: HashMap<Symbol, Value>,
    parent: Option<Arc<Env>>
}

impl Env {
    pub fn inside(outer: &Arc<Env>) -> EnvBuilder {
        let scope = Env {
            bindings: HashMap::new(),
            parent: Some(Arc::clone(outer))
        };

        EnvBuilder { env: scope }
    }

    pub fn global() -> EnvBuilder {
        let global_scope = Env {
            bindings: HashMap::new(),
            parent: None
        };

        EnvBuilder { env: global_scope }
    }

    pub fn get(&self, identifier: &Symbol) -> Option<Value> {
        match self.bindings.get(identifier) {
            Some(value) => Some(value.clone()),
            None =>
                if let Some(parent) = &self.parent {
                    parent.get(identifier)
                } else {
                    None
                }
        }
    }

}

lazy_static! {
    pub static ref GLOBAL: Arc<Env> = Env::global()
        .build();
}

pub struct EnvBuilder {
    env: Env
}

impl EnvBuilder {
    pub fn build(self) -> Arc<Env> {
        Arc::new(self.env)
    }

    pub fn bind(mut self, bindings: Vec<(Symbol, Value)>) -> Self {
        for (sym, value) in bindings {
            self.env.bindings.insert(sym, value);
        }
        self
    }

    pub fn bind_one(mut self, sym: Symbol, value: Value) -> Self {
        self.env.bindings.insert(sym, value);
        self
    }
}

