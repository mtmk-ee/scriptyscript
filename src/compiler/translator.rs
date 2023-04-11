use crate::runtime::opcode::OpCode;

use super::ast::{AstNode, BinaryOperationKind, Number, UnaryOperationKind};

/// Translates an AST node into a list of opcodes which can be executed on a state.
///
/// # Errors
/// Returns an error if the AST node could not be compiled.
pub fn translate_node(ast: &AstNode) -> Result<Vec<OpCode>, anyhow::Error> {
    let mut bytecode = Vec::new();

    match ast {
        AstNode::Block(nodes) => {
            nodes.iter().for_each(|node| {
                bytecode.extend(translate_node(node).unwrap());
            });
        }
        AstNode::Assignment { identifier, value } => {
            bytecode.extend(translate_node(value).unwrap());
            bytecode.push(OpCode::Store(identifier.clone()));
        }
        AstNode::FunctionCall { identifier, args } => {
            args.iter().for_each(|arg| {
                bytecode.extend(translate_node(arg).unwrap());
            });
            bytecode.push(OpCode::Load(identifier.clone()));
            bytecode.push(OpCode::Call(args.len()));
        }
        AstNode::FunctionDef { args, body } => {
            let mut translated_body = Vec::new();
            for name in args {
                translated_body.push(OpCode::Store(name.clone()))
            }
            translated_body.extend(translate_node(body).unwrap());

            bytecode.push(OpCode::PushFunction(translated_body));
        }
        AstNode::Return { value } => {
            let mut n = 0;
            if let Some(value) = value {
                bytecode.extend(translate_node(value).unwrap());
                n = 1;
            }
            bytecode.push(OpCode::Return(n));
        }
        AstNode::If {
            condition,
            body,
            else_body,
        } => {
            bytecode.push(OpCode::If {
                condition: translate_node(condition).unwrap(),
                body: translate_node(body).unwrap(),
                else_body: else_body
                    .as_ref()
                    .map(|else_body| translate_node(else_body).unwrap()),
            });
        }
        AstNode::For {
            initialization,
            condition,
            increment,
            body,
        } => {
            bytecode.push(OpCode::For {
                initialization: initialization
                    .as_ref()
                    .map(|node| translate_node(node).unwrap()),
                condition: condition.as_ref().map(|node| translate_node(node).unwrap()),
                increment: increment.as_ref().map(|node| translate_node(node).unwrap()),
                body: translate_node(body).unwrap(),
            });
        }
        AstNode::BinaryOperation { kind, left, right } => {
            bytecode.extend(translate_node(left).unwrap());
            bytecode.extend(translate_node(right).unwrap());
            bytecode.push((*kind).into());
        }
        AstNode::UnaryOperation { kind, operand } => {
            bytecode.extend(translate_node(operand).unwrap());
            bytecode.push((*kind).into());
        }
        AstNode::Identifier(identifier) => {
            bytecode.push(OpCode::Load(identifier.clone()));
        }
        AstNode::NumberLiteral(number) => match number {
            Number::Integer(x) => bytecode.push(OpCode::PushInteger(*x)),
            Number::Float(x) => bytecode.push(OpCode::PushFloat(*x)),
        },
        AstNode::StringLiteral(string) => {
            bytecode.push(OpCode::PushString(string.clone()));
        }
        AstNode::BooleanLiteral(boolean) => {
            bytecode.push(OpCode::PushBool(*boolean));
        }
    }

    Ok(bytecode)
}

impl From<BinaryOperationKind> for OpCode {
    /// Convert a [`BinaryOperationKind`] into its matching [`OpCode`].
    fn from(kind: BinaryOperationKind) -> Self {
        match kind {
            BinaryOperationKind::Add => OpCode::Add,
            BinaryOperationKind::Subtract => OpCode::Subtract,
            BinaryOperationKind::Multiply => OpCode::Multiply,
            BinaryOperationKind::Divide => OpCode::Divide,
            BinaryOperationKind::Remainder => OpCode::Remainder,
            BinaryOperationKind::Equal => OpCode::Equal,
            BinaryOperationKind::NotEqual => OpCode::NotEqual,
            BinaryOperationKind::LessThan => OpCode::LessThan,
            BinaryOperationKind::LessThanOrEqual => OpCode::LessThanOrEqual,
            BinaryOperationKind::GreaterThan => OpCode::GreaterThan,
            BinaryOperationKind::GreaterThanOrEqual => OpCode::GreaterThanOrEqual,
            BinaryOperationKind::And => OpCode::And,
            BinaryOperationKind::Or => OpCode::Or,
            _ => todo!(),
        }
    }
}

impl From<UnaryOperationKind> for OpCode {
    /// Convert a [`UnaryOperationKind`] into its matching [`OpCode`].
    fn from(_kind: UnaryOperationKind) -> Self {
        match _kind {
            UnaryOperationKind::Negate => OpCode::Negate,
            _ => todo!(),
        }
    }
}
