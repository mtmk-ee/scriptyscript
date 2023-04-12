use std::sync::Arc;

use crate::runtime::bytecode::Bytecode;

use super::{
    function::{Function, ScriptedFunction, WrappedFunction},
    object::{Object, ObjectValue},
    primitive::Primitive,
    table::Table,
};

pub fn int<T: num_traits::PrimInt>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::Integer(
            value.to_i64().unwrap(),
        ))),
        None,
    )
}

pub fn float<T: num_traits::Float>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::Float(
            value.to_f64().unwrap(),
        ))),
        None,
    )
}

pub fn string<T: AsRef<str>>(value: T) -> Object {
    Object::new(
        Some(ObjectValue::Primitive(Primitive::String(
            value.as_ref().to_string(),
        ))),
        None,
    )
}

pub fn nil() -> Object {
    Object::new(Some(ObjectValue::Primitive(Primitive::Nil)), None)
}

pub fn wrapped_function(func: WrappedFunction) -> Object {
    Object::new(
        Some(ObjectValue::Function(Arc::new(Function::Wrapped(func)))),
        None,
    )
}

pub fn scripted_function(bytecode: Bytecode) -> Object {
    Object::new(
        Some(ObjectValue::Function(Arc::new(Function::Scripted(
            ScriptedFunction::new(bytecode),
        )))),
        None,
    )
}

pub fn table() -> Object {
    Object::new(Some(ObjectValue::Table(Table::new())), None)
}

pub fn boolean(x: bool) -> Object {
    Object::new(Some(ObjectValue::Primitive(Primitive::Boolean(x))), None)
}
