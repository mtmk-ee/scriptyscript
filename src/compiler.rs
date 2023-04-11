use crate::{ast::{AstNode, Number}, opcode::OpCode};




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
        AstNode::BinaryOperation { kind, left, right } => {
            bytecode.extend(compile_node(right).unwrap());
            bytecode.extend(compile_node(left).unwrap());
            bytecode.push(OpCode::Duplicate);
            bytecode.push(OpCode::GetKey(kind.dunder()));
            bytecode.push(OpCode::Call(2));
        }
        AstNode::UnaryOperation { kind, operand } => {
            bytecode.extend(compile_node(operand).unwrap());
            bytecode.push(OpCode::GetKey(kind.dunder()));
            bytecode.push(OpCode::Call(1));
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
