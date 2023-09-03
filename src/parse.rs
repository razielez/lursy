use std::fs::File;

use crate::byte_code::ByteCode;
use crate::lex::{Lex, Token};
use crate::value::Value;

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}

impl ParseProto {
    pub fn load(input: File) -> ParseProto {
        let mut constants = Vec::new();
        let mut byte_codes = Vec::new();
        let mut lex = Lex::new(input);
        loop {
            match lex.next() {
                Token::Name(n) => {
                    constants.push(Value::String(n));
                    byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));
                    if let Token::String(x) = lex.next() {
                        constants.push(Value::String(x));
                        byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                        byte_codes.push(ByteCode::Call(0, 1))
                    } else {
                        panic!("expected string");
                    };
                }
                Token::Eos => break,
                it => panic!("unexpected token {it:?}"),
            };
        }
        dbg!(&constants);
        dbg!(&byte_codes);
        ParseProto {
            constants,
            byte_codes,
        }
    }

    fn add_const(&mut self, value: Value) -> usize {
        self.constants.iter().position(|x| x == &value)
            .unwrap_or_else(|| {
                self.constants.push(value);
                self.constants.len() - 1
            })
    }

    fn load_const(&mut self, value: Value) {
        let idx = self.add_const(value);
        self.byte_codes.push(ByteCode::LoadConst(0, idx as u8));
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;

    use crate::parse;

    #[test]
    fn test_load() {
        let dir = env::current_dir().unwrap();
        let file = File::open(dir.join("lua/hello-world.lua")).unwrap();
        println!("{:?}", file);
        let proto = parse::load(file);
        dbg!(proto);
    }
}
