use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
};

use crate::runtime::{state::State};

use super::{
    function::Function,
    primitive::Primitive,
    table::Table,
    utilities::{boolean, float, int, nil},
};

#[derive(Debug, Clone)]
pub enum ObjectValue {
    Primitive(Primitive),
    Function(Arc<Function>),
    Table(Table),
}

#[derive(Debug, Clone)]
pub struct ObjectInner {
    pub value: Option<ObjectValue>,
    #[allow(unused)]
    pub metatable: Option<Object>,
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
    pub inner: Arc<Mutex<ObjectInner>>,
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

    pub fn as_bool(&self) -> Option<bool> {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Primitive(Primitive::Boolean(x))) => Some(*x),
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
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner.lock().unwrap().value {
            Some(ObjectValue::Primitive(p)) => write!(f, "{}", p.to_string()),
            Some(ObjectValue::Function(function)) => write!(f, "{}", function),
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
