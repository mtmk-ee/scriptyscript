use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpCode {
    Load(String),
    Store(String),
    GetKey(String),
    SetKey(String),
    Duplicate,

    PushString(String),
    PushInteger(i64),
    PushFloat(f64),
    PushBool(bool),

    // Binary operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    Call(usize),
}
