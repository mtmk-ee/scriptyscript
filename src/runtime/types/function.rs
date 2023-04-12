/// Module containing the [`Function`] enum, which is used to represent a callable function.
/// The function may either be a scripted or a wrapped (Rust-side).
use std::fmt::{Debug, Display};

use crate::runtime::{bytecode::Bytecode, state::State};

/// A function pointer to a native function.
///
/// The first argument is the state the function was called by.
/// The second argument is the number of arguments passed to the function
/// which the native function may pop from the state.
/// The return value is the number of values pushed back onto the stack.
///
/// Currently, the wrapped function is in charge of keeping the stack balanced
/// to ensure stability. This may change in the future.
pub type WrappedFunction = fn(state: &mut State, n_args: usize) -> usize;

/// An enum wrapping either a scripted function (containing bytecode) or a wrapped function
/// (a function pointer to a native function)
#[derive(Clone)]
pub enum Function {
    /// A scripted function.
    Scripted(ScriptedFunction),
    /// A wrapped function.
    Wrapped(WrappedFunction),
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scripted(func) => f
                .debug_tuple("scripted function")
                .field(func.bytecode())
                .finish(),
            Self::Wrapped(func) => f
                .debug_tuple("wrapped function")
                .field(&(*func as usize))
                .finish(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Scripted(a), Self::Scripted(b)) => a.bytecode() == b.bytecode(),
            (Self::Wrapped(a), Self::Wrapped(b)) => *a as usize == *b as usize,
            _ => false,
        }
    }
}

/// A scripted function containing its bytecode.
#[derive(Debug, Clone)]
pub struct ScriptedFunction {
    /// The bytecode of the function.
    bytecode: Bytecode,
}

impl ScriptedFunction {
    /// Creates a new scripted function from the given bytecode.
    #[must_use]
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    /// Returns the bytecode of the function.
    #[must_use]
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }
}
