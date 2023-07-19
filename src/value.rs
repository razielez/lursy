use crate::vm::ExeState;

pub enum Value {
    Nil,
    String(String),
    Function(fn (&mut ExeState) -> i32)

}
