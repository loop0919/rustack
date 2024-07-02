use std::collections::HashMap;

use crate::rustack::value::{Value, NativeOp};
use crate::rustack::functions::*;

pub struct Vm {
    pub stack: Vec<Value>,
    pub vars: Vec<HashMap<String, Value>>,
    pub blocks: Vec<Vec<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        let functions: [(&str, fn(&mut Vm)); 16] = [
            ("+", add),
            ("-", sub),
            ("*", mul),
            ("/", div),
            ("<", lt),
            ("<=", le),
            (">", gt),
            (">=", ge),
            ("==", eq),
            ("!=", neq),
            ("if", op_if),
            ("def", op_def),
            ("puts", puts),
            ("dup", dup),
            ("exch", exch),
            ("index", index),
        ];
        Self {
            stack: vec![],
            vars: vec![
                functions.into_iter()
                    .map(|(name, fun)| {
                        (name.to_owned(), Value::Native(NativeOp(fun)))
                    })
                    .collect()
            ],
            blocks: vec![],
        }
    }

    pub fn find_var(&self, name: &str) -> Option<Value> {
        self.vars
            .iter()
            .rev()
            .find_map(|vars| vars.get(name).map(|var| var.to_owned()))
    }
}
