use std::cmp::Ordering;
use std::collections::HashMap;

use crate::byte_code::ByteCode;
use crate::parse::ParseProto;
use crate::value::Value;

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

fn fn_print(state: &mut ExeState) -> i32 {
    let v = &state.stack[1];
    match v {
        Value::String(s) => println!("{}", s),
        _ => println!("{:?}", v),
    };
    0
}

impl ExeState {
    pub fn new() -> Self {
        let mut g = HashMap::new();
        g.insert("print".into(), Value::Function(fn_print));
        ExeState {
            globals: g,
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, proto: &ParseProto) {
        for bc in proto.byte_codes.iter() {
            match *bc {
                ByteCode::GetGlobal(idx, x) => {
                    let name = &proto.constants[x as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(idx, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    };
                }
                ByteCode::LoadConst(idx, x) => {
                    let v = proto.constants[x as usize].clone();
                    self.set_stack(idx, v);
                }
                ByteCode::Call(func, _) => {
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}")
                    };
                }
            }
        }
    }

    fn set_stack(&mut self, idx: u8, v: Value) {
        let idx = idx as usize;
        match idx.cmp(&self.stack.len()) {
            Ordering::Less => self.stack[idx] = v,
            Ordering::Equal => self.stack.push(v),
            Ordering::Greater => panic!("invalid stack index: {idx}"),
        };
    }
}