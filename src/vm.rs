use std::collections::HashMap;
use crate::value::Value;

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}