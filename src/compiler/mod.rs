
use crate::runtime::opcode::OpCode;

use self::translator::translate_node;

pub mod ast;
pub mod parser;
pub mod translator;

/// Compile a source string into a list of opcodes.
///
/// # Errors
/// Returns an error if the source string could not be compiled.
pub fn compile(source: impl AsRef<str>) -> Result<Vec<OpCode>, anyhow::Error> {
    translate_node(&parser::parse(source)?)
}
