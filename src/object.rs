use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::Add,
    sync::{Arc, Mutex},
};

use crate::{
    opcode::OpCode,
    state::{execute, State},
};

// ========================================================================================================================
// Table
// ========================================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    inner: HashMap<String, Object>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        self.inner.get(key)
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.inner.insert(key, value);
    }
}

// ========================================================================================================================
// Function
// ========================================================================================================================
pub type WrappedFunction = fn(&mut State, usize) -> usize;

#[derive(Clone)]
pub enum Function {
    Scripted(ScriptedFunction),
    Wrapped(WrappedFunction),
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scripted(func) => f.debug_tuple("Scripted").field(func).finish(),
            Self::Wrapped(func) => f.debug_tuple("Wrapped").field(&(*func as usize)).finish(),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Function::Scripted(a), Function::Scripted(b)) => a.bytecode() == b.bytecode(),
            (Function::Wrapped(a), Function::Wrapped(b)) => *a as usize == *b as usize,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptedFunction {
    bytecode: Vec<OpCode>,
}

impl ScriptedFunction {
    pub fn new(bytecode: Vec<OpCode>) -> ScriptedFunction {
        ScriptedFunction { bytecode }
    }

    pub fn bytecode(&self) -> &Vec<OpCode> {
        &self.bytecode
    }
}

// ========================================================================================================================
// Primitive
// ========================================================================================================================
#[derive(Debug, Clone)]
pub enum Primitive {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Eq for Primitive {}
impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Primitive::Nil, Primitive::Nil) => true,
            (Primitive::Integer(a), Primitive::Integer(b)) => a == b,
            (Primitive::Float(a), Primitive::Float(b)) => a == b,
            (Primitive::String(a), Primitive::String(b)) => a == b,
            (Primitive::Boolean(a), Primitive::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

impl Add for Primitive {
    type Output = Option<Primitive>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a + b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 + b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a + b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a + b)),
            (Primitive::String(a), Primitive::String(b)) => Some(Primitive::String(a + b.as_str())),
            _ => None,
        }
    }
}

impl std::ops::Sub for Primitive {
    type Output = Option<Primitive>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a - b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 - b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a - b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a - b)),
            _ => None,
        }
    }
}

impl std::ops::Mul for Primitive {
    type Output = Option<Primitive>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a * b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 * b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a * b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a * b)),
            _ => None,
        }
    }
}

impl std::ops::Div for Primitive {
    type Output = Option<Primitive>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a / b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 / b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a / b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a / b)),
            _ => None,
        }
    }
}

impl std::ops::Rem for Primitive {
    type Output = Option<Primitive>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a % b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 % b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a % b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a % b)),
            _ => None,
        }
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Nil => "nil".to_string(),
            Primitive::Integer(i) => i.to_string(),
            Primitive::Float(f) => f.to_string(),
            Primitive::String(s) => s.to_string(),
            Primitive::Boolean(b) => b.to_string(),
        }
    }
}

// ========================================================================================================================
// Object
// ========================================================================================================================

#[derive(Debug, Clone)]
pub enum ObjectValue {
    Primitive(Primitive),
    Function(Arc<Function>),
    Table(Table),
}

#[derive(Debug, Clone)]
pub struct ObjectInner {
    value: Option<ObjectValue>,
    #[allow(unused)]
    metatable: Option<Object>,
}

impl ObjectInner {
    pub fn new(value: Option<ObjectValue>, metatable: Option<Object>) -> ObjectInner {
        ObjectInner { value, metatable }
    }

    pub fn value(&self) -> &Option<ObjectValue> {
        &self.value
    }

    pub fn set_value(&mut self, value: Option<ObjectValue>) {
        self.value = value;
    }

    pub fn metatable(&self) -> &Option<Object> {
        &self.metatable
    }

    pub fn set_metatable(&mut self, metatable: Option<Object>) {
        self.metatable = metatable;
    }
}

#[derive(Clone)]
pub struct Object {
    inner: Arc<Mutex<ObjectInner>>,
}

impl Object {
    pub fn new(value: Option<ObjectValue>, metatable: Option<Object>) -> Object {
        Object {
            inner: Arc::new(Mutex::new(ObjectInner { value, metatable })),
        }
    }

    pub fn inner(&self) -> Arc<Mutex<ObjectInner>> {
        self.inner.clone()
    }

    pub fn as_primitive(&self) -> Option<Primitive> {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Primitive(p)) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn set_key(&mut self, key: &str, value: Object) {
        match &mut self.inner.lock().unwrap().value {
            Some(ObjectValue::Table(table)) => table.set(key.to_owned(), value),
            _ => panic!("Cannot set key on non-table object"),
        }
    }

    pub fn get_key(&self, key: &str) -> Option<Object> {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Table(table)) => table.get(key).cloned(),
            _ => panic!("Cannot get key on non-table object"),
        }
    }

    pub fn call(&self, state: &mut State, n_args: usize) -> usize {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Function(f)) => {
                let args = state.pop_n(n_args);
                state.push_frame();
                state.push_all(&args);
                let push_amt = match f.borrow() {
                    Function::Wrapped(f) => f(state, n_args),
                    Function::Scripted(f) => execute(state, f.bytecode().clone()),
                };
                let returns = state.pop_n(push_amt);
                state.pop_frame();
                state.push_all(&returns);
                push_amt
            }
            _ => panic!("Cannot call non-function object"),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Primitive(p)) => write!(f, "{}", p.to_string()),
            Some(ObjectValue::Function(_)) => write!(f, "function"),
            Some(ObjectValue::Table(t)) => write!(f, "table: {:?}", t),
            None => write!(f, "nil"),
        }
    }
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (
            &self.inner.lock().unwrap().value,
            &other.inner.lock().unwrap().value,
        ) {
            (Some(ObjectValue::Primitive(a)), Some(ObjectValue::Primitive(b))) => a == b,
            (Some(ObjectValue::Table(a)), Some(ObjectValue::Table(b))) => a == b,
            (Some(ObjectValue::Function(a)), Some(ObjectValue::Function(b))) => a == b,
            _ => false,
        }
    }
}

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

pub fn scripted_function(bytecode: Vec<OpCode>) -> Object {
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


fn binary_op(state: &mut State, lhs: &Object, rhs: &Object, primitive_op: fn(Primitive, Primitive) -> Option<Primitive>) {
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
        _ => state.push(&nil())
    }
}
