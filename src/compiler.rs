use crate::{ast::{AstNode, Number, BinaryOperationKind, UnaryOperationKind}, opcode::OpCode};



/// Compiles an AST node into a list of opcodes which can be executed on a state.
///
/// # Errors
/// Returns an error if the AST node could not be compiled.
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
            bytecode.extend(compile_node(left).unwrap());
            bytecode.extend(compile_node(right).unwrap());
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

    /// Convert a [`BinaryOperationKind`] into its matching [`OpCode`].
    fn from(kind: BinaryOperationKind) -> Self {
        match kind {
            BinaryOperationKind::Add => OpCode::Add,
            BinaryOperationKind::Subtract => OpCode::Subtract,
            BinaryOperationKind::Multiply => OpCode::Multiply,
            BinaryOperationKind::Divide => OpCode::Divide,
            BinaryOperationKind::Remainder => OpCode::Remainder,
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
