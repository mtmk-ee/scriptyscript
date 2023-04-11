use std::fmt::Debug;

use crate::runtime::{opcode::OpCode, state::State};

pub type WrappedFunction = fn(&mut State, usize) -> usize;

#[derive(Clone)]
pub enum Function {
    Scripted(ScriptedFunction),
    Wrapped(WrappedFunction),
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scripted(func) => f.debug_tuple("Scripted").field(func).finish(),
            Self::Wrapped(func) => f.debug_tuple("Wrapped").field(&(*func as usize)).finish(),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Function::Scripted(a), Function::Scripted(b)) => a.bytecode() == b.bytecode(),
            (Function::Wrapped(a), Function::Wrapped(b)) => *a as usize == *b as usize,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptedFunction {
    bytecode: Vec<OpCode>,
}

impl ScriptedFunction {
    pub fn new(bytecode: Vec<OpCode>) -> ScriptedFunction {
        ScriptedFunction { bytecode }
    }

    pub fn bytecode(&self) -> &Vec<OpCode> {
        &self.bytecode
    }
}
