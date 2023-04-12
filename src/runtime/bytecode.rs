//! Module containing [`OpCode`]s and the [`Bytecode`] container.

use serde::{Deserialize, Serialize};

use crate::compiler::{BinaryOperationKind, UnaryOperationKind};

/// Container for bytecode.
///
/// Currently this is simply a wrapper around a `Vec<OpCode>`. This type
/// should be used rather than `Vec<OpCode>` for forward-compatibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bytecode {
    inner: Vec<OpCode>,
}

impl Bytecode {
    /// Create an empty bytecode container.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Fetch the inner vector of opcodes.
    pub fn inner(&self) -> &Vec<OpCode> {
        &self.inner
    }

    /// Fetch the inner vector of opcodes mutably.
    pub fn inner_mut(&mut self) -> &mut Vec<OpCode> {
        &mut self.inner
    }

    /// Consume `self` and return the inner vector of opcodes.
    pub fn into_inner(self) -> Vec<OpCode> {
        self.inner
    }

    /// Returns an iterator over the opcodes.
    pub fn iter(&self) -> std::slice::Iter<OpCode> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the opcodes.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<OpCode> {
        self.inner.iter_mut()
    }

    /// Extends the bytecode with the opcodes from another bytecode.
    pub fn extend(&mut self, other: &mut Bytecode) {
        self.inner.append(&mut other.inner);
    }

    /// Pushes a single opcode into the bytecode container.
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

/// Opcodes representing instructions which the executor can apply to a [`State`](crate::runtime::state::State).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpCode {
    // ====================== Scope Operations ======================
    /// Load a value with the given name from the current (or parent) scope onto the stack.
    ///
    /// Stack: `[] -> [value]`
    Load(String),
    /// Store a value with the given name in the current scope.
    ///
    /// Stack: `[value] -> []`
    Store(String),
    /// Load a value from a table
    ///
    /// Stack: `[object] -> [value]`
    GetKey(String),
    /// Store a value into a table.
    ///
    /// Stack: `[object, value] -> []`
    SetKey(String),

    // ====================== Push Operations ======================
    /// Push a nil value onto the stack.
    ///
    /// Stack: `[] -> [nil]`
    PushNil,
    /// Push a string onto the stack.
    ///
    /// Stack: `[] -> [string]`
    PushString(String),
    /// Push an integer onto the stack.
    ///
    /// Stack: `[] -> [integer]`
    PushInteger(i64),
    /// Push a float onto the stack.
    ///
    /// Stack: `[] -> [float]`
    PushFloat(f64),
    /// Push a boolean onto the stack.
    ///
    /// Stack: `[] -> [boolean]`
    PushBool(bool),
    /// Push a function with the given bytecode onto the stack.
    ///
    /// Stack: `[] -> [function]`
    PushFunction(Bytecode),

    // ====================== Expressions  ======================
    /// Perform a binary operation on the top two values on the stack.
    ///
    /// Stack: `[rhs, rhs] -> [result]`
    BinaryOperation(BinaryOperationKind),
    /// Perform a unary operation on the top value on the stack.
    ///
    /// Stack: `[value] -> [result]`
    UnaryOperation(UnaryOperationKind),
    /// Call a function with the given number of arguments.
    ///
    /// Stack: `[arg n-1, arg n-2, ..., arg0, function] -> [result n-1, result n-2, ..., result0]`
    Call(usize),

    // ====================== Control Flow ======================
    /// Break out of the current loop.
    Break,
    /// Continue to the next iteration of the current loop.
    Continue,
    /// Return from the current function.
    ///
    /// The given number of values will be popped from the stack and pushed onto the
    /// parent frame's stack.
    Return(usize),
    /// An if statement.
    If {
        /// Condition to check. The bytecode is executed and is checked by popping the result
        /// from the stack.
        condition: Bytecode,
        /// Body to execute when the condition is `true`.
        body: Bytecode,
        /// Body to execute when the condition is `false`.
        else_body: Option<Bytecode>,
    },
    /// A for loop.
    For {
        /// Initialization code. This is executed once before the loop starts.
        initialization: Option<Bytecode>,
        /// Condition to check. The bytecode is executed before each iteration, and
        /// is checked by popping the result from the stack.
        condition: Option<Bytecode>,
        /// Increment code. This is executed after each iteration.
        increment: Option<Bytecode>,
        /// Body to execute.
        body: Bytecode,
    },
    /// While loop.
    While {
        /// Condition to check. The bytecode is executed before each iteration, and
        /// is checked by popping the result from the stack.
        condition: Bytecode,
        /// Body to execute.
        body: Bytecode,
    },
    /// Infinite-ish loop. This can still be exited through `break` and `return` statements.
    Loop {
        /// Body to execute.
        body: Bytecode,
    },
}
