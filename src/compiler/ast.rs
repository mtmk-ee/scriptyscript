use serde::{Deserialize, Serialize};

/// A big enum of every possible type of node in the AST.
///
/// The root node of an AST is usually a [`Block`].
#[derive(Debug, Clone)]
pub enum AstNode {
    Identifier(String),
    NumberLiteral(Number),
    StringLiteral(String),
    BooleanLiteral(bool),
    FunctionCall {
        identifier: String,
        args: Vec<AstNode>,
    },
    FunctionDef {
        args: Vec<String>,
        body: Box<AstNode>,
    },
    UnaryOperation {
        kind: UnaryOperationKind,
        operand: Box<AstNode>,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Assignment {
        identifier: String,
        value: Box<AstNode>,
    },
    Return {
        value: Option<Box<AstNode>>,
    },
    Break,
    Continue,
    If {
        condition: Box<AstNode>,
        body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
    },
    For {
        initialization: Option<Box<AstNode>>,
        condition: Option<Box<AstNode>>,
        increment: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Loop {
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
}

/// The type of a unary operation.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnaryOperationKind {
    Negate,
    Not,
}

impl UnaryOperationKind {
    pub fn dunder(&self) -> String {
        match self {
            UnaryOperationKind::Negate => "__neg__",
            UnaryOperationKind::Not => "__not__",
        }
        .to_string()
    }
}

/// The type of a binary operation.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryOperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Power,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl BinaryOperationKind {
    pub fn dunder(&self) -> String {
        match self {
            BinaryOperationKind::Add => "__add__",
            BinaryOperationKind::Subtract => "__sub__",
            BinaryOperationKind::Multiply => "__mul__",
            BinaryOperationKind::Divide => "__div__",
            BinaryOperationKind::Remainder => "__rem__",
            BinaryOperationKind::Power => "__pow__",
            BinaryOperationKind::And => "__and__",
            BinaryOperationKind::Or => "__or__",
            BinaryOperationKind::Equal => "__eq__",
            BinaryOperationKind::NotEqual => "__ne__",
            BinaryOperationKind::GreaterThan => "__gt__",
            BinaryOperationKind::GreaterThanOrEqual => "__ge__",
            BinaryOperationKind::LessThan => "__lt__",
            BinaryOperationKind::LessThanOrEqual => "__le__",
        }
        .to_string()
    }
}

/// Holds either an integer or float value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
