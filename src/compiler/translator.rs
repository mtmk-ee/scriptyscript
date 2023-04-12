//! Module for translating an AST into bytecode for use by the [`executor`](crate::runtime::executor).
//!
//! There's a single public function, [`translate_node`], which can be used to translate any
//! node in an AST (including the root node) into its bytecode representation.

use std::borrow::Borrow;

use super::ast::{AstNode, Number};
use crate::runtime::bytecode::{Bytecode, OpCode};

impl<T: Borrow<AstNode>> From<T> for Bytecode {
    fn from(node: T) -> Self {
        translate_node(node.borrow())
    }
}

/// Translates an AST node into a list of opcodes which can be executed on a state.
///
/// # Errors
/// Returns an error if the AST node could not be compiled.
pub fn translate_node(ast: &AstNode) -> Bytecode {
    let mut result = Bytecode::new();
    let inner = result.inner_mut();

    match ast {
        AstNode::Block(nodes) => {
            nodes.iter().for_each(|node| {
                inner.extend(translate_node(node));
            });
        }
        AstNode::Assignment { identifier, value } => {
            inner.extend(translate_node(value));
            inner.push(OpCode::Store(identifier.clone()));
        }
        AstNode::FunctionCall { identifier, args } => {
            args.iter().for_each(|arg| {
                inner.extend(translate_node(arg));
            });
            inner.push(OpCode::Load(identifier.clone()));
            inner.push(OpCode::Call(args.len()));
        }
        AstNode::FunctionDef { args, body } => {
            let mut translated_body = Bytecode::new();
            for name in args {
                translated_body
                    .inner_mut()
                    .push(OpCode::Store(name.clone()))
            }
            translated_body.inner_mut().extend(translate_node(body));
            inner.push(OpCode::PushFunction(translated_body));
        }
        AstNode::Return { value } => {
            // Return can be empty, or can return the result of an expression.
            let mut n = 0;
            if let Some(value) = value {
                inner.extend(translate_node(value));
                n = 1;
            }
            inner.push(OpCode::Return(n));
        }
        AstNode::Break => {
            inner.push(OpCode::Break);
        }
        AstNode::Continue => {
            inner.push(OpCode::Continue);
        }
        AstNode::If {
            condition,
            body,
            else_body,
        } => {
            inner.push(OpCode::If {
                condition: translate_node(condition),
                body: translate_node(body),
                else_body: else_body
                    .as_ref()
                    .map(|else_body| translate_node(else_body)),
            });
        }
        AstNode::For {
            initialization,
            condition,
            increment,
            body,
        } => {
            inner.push(OpCode::For {
                initialization: initialization.as_ref().map(|node| translate_node(node)),
                condition: condition.as_ref().map(|node| translate_node(node)),
                increment: increment.as_ref().map(|node| translate_node(node)),
                body: translate_node(body),
            });
        }
        AstNode::While { condition, body } => {
            inner.push(OpCode::While {
                condition: translate_node(condition),
                body: translate_node(body),
            });
        }
        AstNode::Loop { body } => {
            inner.push(OpCode::Loop {
                body: translate_node(body),
            });
        }
        AstNode::BinaryOperation { kind, left, right } => {
            inner.extend(translate_node(left));
            inner.extend(translate_node(right));
            inner.push(OpCode::BinaryOperation(*kind));
        }
        AstNode::UnaryOperation { kind, operand } => {
            inner.extend(translate_node(operand));
            inner.push(OpCode::UnaryOperation(*kind));
        }
        AstNode::Identifier(identifier) => {
            inner.push(OpCode::Load(identifier.clone()));
        }
        AstNode::NumberLiteral(number) => match number {
            Number::Integer(x) => inner.push(OpCode::PushInteger(*x)),
            Number::Float(x) => inner.push(OpCode::PushFloat(*x)),
        },
        AstNode::StringLiteral(string) => {
            inner.push(OpCode::PushString(string.clone()));
        }
        AstNode::BooleanLiteral(boolean) => {
            inner.push(OpCode::PushBool(*boolean));
        }
        AstNode::NilLiteral => {
            inner.push(OpCode::PushNil);
        }
    }
    result
}
