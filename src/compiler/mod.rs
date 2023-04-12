//! The compiler module contains the compiler for the language.
//!
//! There are two main jobs the compiler performs when compiling a source string:
//! 1. Parse the source string into an AST (Abstract Syntax Tree).
//! 2. Translate the AST into a list of opcodes ("bytecode") which can be later executed.
//!
//! The compiler is split into three modules:
//! - [`ast`] - Contains data structures representing an AST.
//! - [`parser`] - Contains the parser, which parses a source string into an AST.
//! - [`translator`] - Contains the translator, which translates an AST into bytecode.

use crate::runtime::bytecode::Bytecode;

use self::translator::translate_node;

pub mod ast;
pub mod parser;
pub mod translator;

pub use ast::*;
pub use parser::*;

/// Compile a source string into bytecode.
///
/// This is a simple wrapper around the parser -> translator pipeline.
///
/// # Errors
/// Returns an error if the source string could not be compiled.
pub fn compile(source: impl AsRef<str>) -> Result<Bytecode, anyhow::Error> {
    Ok(translate_node(&parser::parse(source)?))
}
