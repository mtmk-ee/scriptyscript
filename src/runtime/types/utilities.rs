//! Utilities for creating objects from Rust types.

use std::sync::Arc;

use super::{
    function::{Function, ScriptedFunction, WrappedFunction},
    object::{Object, ObjectValue},
    primitive::Primitive,
};
use crate::runtime::bytecode::Bytecode;

/// Creates an integer object from an integral value.
pub fn int<T: num_traits::PrimInt>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::Integer(
            value.to_i64().unwrap(),
        ))),
        None,
    )
}

/// Creates a float object from a floating point value.
pub fn float<T: num_traits::Float>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::Float(
            value.to_f64().unwrap(),
        ))),
        None,
    )
}

/// Creates a string object from a [`String`] or [`str`] slice.
pub fn string<T: AsRef<str>>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::String(
            value.as_ref().to_string(),
        ))),
        None,
    )
}

/// Creates a nil object.
pub fn nil() -> Object {
    Object::new(Some(ObjectValue::Primitive(Primitive::Nil)), None)
}

/// Creates a function object wrapping the given Rust-side function.
pub fn wrapped_function(func: WrappedFunction) -> Object {
    Object::new(
        Some(ObjectValue::Function(Arc::new(Function::Wrapped(func)))),
        None,
    )
}

/// Creates a function object from the given bytecode.
pub fn scripted_function(bytecode: Bytecode) -> Object {
    Object::new(
        Some(ObjectValue::Function(Arc::new(Function::Scripted(
            ScriptedFunction::new(bytecode),
        )))),
        None,
    )
}

/// Creates a table object.
pub fn table() -> Object {
    todo!("tables are unsupported");
}

/// Creates a boolean object from the given value.
pub fn boolean(x: bool) -> Object {
    Object::new(Some(ObjectValue::Primitive(Primitive::Boolean(x))), None)
}
