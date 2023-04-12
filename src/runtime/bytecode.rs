use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bytecode {
    inner: Vec<OpCode>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn inner(&self) -> &Vec<OpCode> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<OpCode> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Vec<OpCode> {
        self.inner
    }

    pub fn iter(&self) -> std::slice::Iter<OpCode> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<OpCode> {
        self.inner.iter_mut()
    }

    pub fn extend(&mut self, other: &mut Bytecode) {
        self.inner.append(&mut other.inner);
    }

    pub fn push(&mut self, op: OpCode) {
        self.inner.push(op);
    }
}

impl IntoIterator for Bytecode {
    type Item = OpCode;
    type IntoIter = std::vec::IntoIter<OpCode>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpCode {
    Load(String),
    Store(String),
    GetKey(String),
    SetKey(String),
    Duplicate,

    PushNil,
    PushString(String),
    PushInteger(i64),
    PushFloat(f64),
    PushBool(bool),
    PushFunction(Bytecode),

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
    Break,
    Continue,

    If {
        condition: Bytecode,
        body: Bytecode,
        else_body: Option<Bytecode>,
    },
    For {
        initialization: Option<Bytecode>,
        condition: Option<Bytecode>,
        increment: Option<Bytecode>,
        body: Bytecode,
    },
    While {
        condition: Bytecode,
        body: Bytecode,
    },
    Loop {
        body: Bytecode,
    },
}
