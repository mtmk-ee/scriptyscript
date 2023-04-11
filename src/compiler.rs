use crate::{ast::{AstNode, Number, BinaryOperationKind, UnaryOperationKind}, opcode::OpCode};




pub fn compile_node(ast: &AstNode) -> Result<Vec<OpCode>, anyhow::Error> {
    let mut bytecode = Vec::new();

    match ast {
        AstNode::Block(nodes) => {
            nodes.iter().for_each(|node| {
                bytecode.extend(compile_node(node).unwrap());
            });
        }
        AstNode::Assignment { identifier, value } => {
            bytecode.extend(compile_node(value).unwrap());
            bytecode.push(OpCode::Store(identifier.clone()));
        }
        AstNode::FunctionCall { identifier, args } => {
            args.iter().for_each(|arg| {
                bytecode.extend(compile_node(arg).unwrap());
            });
            bytecode.push(OpCode::Load(identifier.clone()));
            bytecode.push(OpCode::Call(args.len()));
        }
        AstNode::BinaryOperation { kind, left, right } => {
            bytecode.extend(compile_node(right).unwrap());
            bytecode.extend(compile_node(left).unwrap());
            bytecode.push((*kind).into());
        }
        AstNode::UnaryOperation { kind, operand } => {
            bytecode.extend(compile_node(operand).unwrap());
            bytecode.push((*kind).into());
        }
        AstNode::Identifier(identifier) => {
            bytecode.push(OpCode::Load(identifier.clone()));
        }
        AstNode::NumberLiteral(number) => {
            match number {
                Number::Integer(x) => bytecode.push(OpCode::PushInteger(*x)),
                Number::Float(x) => bytecode.push(OpCode::PushFloat(*x)),
            }
        }
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
    fn from(kind: BinaryOperationKind) -> Self {
        match kind {
            BinaryOperationKind::Add => OpCode::Add,
            BinaryOperationKind::Subtract => OpCode::Subtract,
            BinaryOperationKind::Multiply => OpCode::Multiply,
            BinaryOperationKind::Divide => OpCode::Divide,
            BinaryOperationKind::Modulus => OpCode::Modulus,
            _ => todo!(),
        }
    }
}

impl From<UnaryOperationKind> for OpCode {
    fn from(kind: UnaryOperationKind) -> Self {
        match kind {
            // UnaryOperationKind::Negate => OpCode::Negate,
            // UnaryOperationKind::Not => OpCode::Not,
            _ => todo!(),
        }
    }
}
