use crate::runtime::state::State;

use super::{object::{Object, ObjectValue}, primitive::Primitive, utilities::{nil, float, int, boolean}};

fn binary_op(
    state: &mut State,
    lhs: &Object,
    rhs: &Object,
    primitive_op: fn(Primitive, Primitive) -> Option<Primitive>,
) {
    match (lhs.as_primitive(), rhs.as_primitive()) {
        (Some(a), Some(b)) => {
            let result = if let Some(result) = primitive_op(a, b) {
                Object::new(Some(ObjectValue::Primitive(result)), None)
            } else {
                nil()
            };
            state.push(&result);
        }
        _ => todo!(),
    }
}

pub fn add(state: &mut State, lhs: &Object, rhs: &Object) {
    binary_op(state, lhs, rhs, std::ops::Add::add);
}

pub fn subtract(state: &mut State, lhs: &Object, rhs: &Object) {
    binary_op(state, lhs, rhs, std::ops::Sub::sub);
}

pub fn multiply(state: &mut State, lhs: &Object, rhs: &Object) {
    binary_op(state, lhs, rhs, std::ops::Mul::mul);
}

pub fn divide(state: &mut State, lhs: &Object, rhs: &Object) {
    binary_op(state, lhs, rhs, std::ops::Div::div);
}

pub fn remainder(state: &mut State, lhs: &Object, rhs: &Object) {
    binary_op(state, lhs, rhs, std::ops::Rem::rem);
}

pub fn negate(state: &mut State, obj: &Object) {
    match obj.as_primitive() {
        Some(Primitive::Integer(i)) => state.push(&int(-i)),
        Some(Primitive::Float(f)) => state.push(&float(-f)),
        _ => state.push(&nil()),
    }
}

pub fn equals(state: &mut State, a: &Object, b: &Object) {
    let a = a.inner.lock().unwrap();
    let b = b.inner.lock().unwrap();
    match (&a.value, &b.value) {
        (Some(ObjectValue::Primitive(a)), Some(ObjectValue::Primitive(b))) => {
            state.push(&boolean(a == b))
        }
        (Some(ObjectValue::Table(a)), Some(ObjectValue::Table(b))) => state.push(&boolean(a == b)),
        (Some(ObjectValue::Function(a)), Some(ObjectValue::Function(b))) => {
            state.push(&boolean(a == b))
        }
        _ => state.push(&boolean(false)),
    }
}

pub fn not_equals(state: &mut State, a: &Object, b: &Object) {
    let a = a.inner.lock().unwrap();
    let b = b.inner.lock().unwrap();
    match (&a.value, &b.value) {
        (Some(ObjectValue::Primitive(a)), Some(ObjectValue::Primitive(b))) => {
            state.push(&boolean(a != b))
        }
        (Some(ObjectValue::Table(a)), Some(ObjectValue::Table(b))) => state.push(&boolean(a != b)),
        (Some(ObjectValue::Function(a)), Some(ObjectValue::Function(b))) => {
            state.push(&boolean(a != b))
        }
        _ => state.push(&boolean(true)),
    }
}

pub fn greater_than(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_primitive(), rhs.as_primitive()) {
        (Some(Primitive::Integer(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs > rhs))
        }
        (Some(Primitive::Integer(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs as f64 > rhs))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs > rhs as f64))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs > rhs))
        }
        _ => todo!("error handling"),
    }
}

pub fn less_than(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_primitive(), rhs.as_primitive()) {
        (Some(Primitive::Integer(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs < rhs))
        }
        (Some(Primitive::Integer(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean((lhs as f64) < rhs))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs < rhs as f64))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs < rhs))
        }
        _ => todo!("error handling"),
    }
}

pub fn greater_than_or_equal(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_primitive(), rhs.as_primitive()) {
        (Some(Primitive::Integer(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs >= rhs))
        }
        (Some(Primitive::Integer(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs as f64 >= rhs))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs >= rhs as f64))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs >= rhs))
        }
        _ => todo!("error handling"),
    }
}

pub fn less_than_or_equal(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_primitive(), rhs.as_primitive()) {
        (Some(Primitive::Integer(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs <= rhs))
        }
        (Some(Primitive::Integer(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs as f64 <= rhs))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Integer(rhs))) => {
            state.push(&boolean(lhs <= rhs as f64))
        }
        (Some(Primitive::Float(lhs)), Some(Primitive::Float(rhs))) => {
            state.push(&boolean(lhs <= rhs))
        }
        _ => todo!("error handling"),
    }
}

pub fn and(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_bool(), rhs.as_bool()) {
        (Some(a), Some(b)) => state.push(&boolean(a && b)),
        _ => todo!("error handling"),
    }
}

pub fn or(state: &mut State, lhs: &Object, rhs: &Object) {
    match (lhs.as_bool(), rhs.as_bool()) {
        (Some(a), Some(b)) => state.push(&boolean(a || b)),
        _ => todo!("error handling"),
    }
}
