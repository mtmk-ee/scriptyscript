//! The crate for the `scriptyscript` interpreter.
//!
//! This crate contains:
//! - A [compiler] which compiles a source string into bytecode.
//!     - A [parser](compiler::parser) which parses a source string into an AST (Abstract Syntax Tree).
//!     - A [translator](compiler::translator) which translates an AST into bytecode.
//! - A [runtime] which executes bytecode.
//! - A [standard library](stdlib) which contains built-in functions and types that are available to scripts.
pub mod compiler;
pub mod runtime;
pub mod stdlib;
