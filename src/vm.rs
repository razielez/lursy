use std::collections::HashMap;
use crate::value::Value;

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl ExeState {
    pub fn new() -> Self {
        todo!()
    }

    pub fn execute(&mut self, proto: &ParseProto){
        todo!()
    }
}