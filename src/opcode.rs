use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Call(usize),
}

impl PartialEq for OpCode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OpCode::Load(a), OpCode::Load(b)) => a == b,
            (OpCode::Store(a), OpCode::Store(b)) => a == b,
            (OpCode::GetKey(a), OpCode::GetKey(b)) => a == b,
            (OpCode::SetKey(a), OpCode::SetKey(b)) => a == b,
            (OpCode::PushString(a), OpCode::PushString(b)) => a == b,
            (OpCode::PushInteger(a), OpCode::PushInteger(b)) => a == b,
            (OpCode::PushFloat(a), OpCode::PushFloat(b)) => a == b,
            (OpCode::Call(a), OpCode::Call(b)) => a == b,
            _ => false,
        }
    }
}
