use std::fs::File;
use crate::byte_code::ByteCode;
use crate::value::Value;

pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}

pub fn load(input: File) -> ParseProto {
    todo!()
}