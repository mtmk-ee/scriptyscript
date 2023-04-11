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
    PushFunction(Vec<OpCode>),

    // Binary operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    // Unary operations
    Negate,

    Call(usize),
    Return(usize),

    If {
        condition: Vec<OpCode>,
        body: Vec<OpCode>,
        else_body: Option<Vec<OpCode>>,
    },
    For {
        initialization: Option<Vec<OpCode>>,
        condition: Option<Vec<OpCode>>,
        increment: Option<Vec<OpCode>>,
        body: Vec<OpCode>,
    },
    While {
        condition: Vec<OpCode>,
        body: Vec<OpCode>,
    },
    Loop {
        body: Vec<OpCode>,
    },
}
