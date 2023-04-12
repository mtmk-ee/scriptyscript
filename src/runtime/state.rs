use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::stdlib;

use super::types::{object::Object, utilities::nil};

pub struct CallFrame {
    pub parent: Option<Arc<Mutex<CallFrame>>>,
    pub operands: Vec<Object>,
    pub locals: HashMap<String, Object>,
}

impl CallFrame {
    pub fn new(parent: Option<Arc<Mutex<CallFrame>>>) -> CallFrame {
        CallFrame {
            parent,
            operands: Vec::new(),
            locals: HashMap::new(),
        }
    }

    pub fn push(&mut self, object: &Object) {
        self.operands.push(object.clone());
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.operands.pop()
    }

    pub fn peek(&self) -> Option<Object> {
        self.operands.last().cloned()
    }

    pub fn stack_size(&self) -> usize {
        self.operands.len()
    }

    pub fn load(&mut self, name: &str) {
        let local_value = self.locals.get(name).cloned();
        if let Some(x) = local_value {
            self.push(&x);
        } else if self.parent.is_some() {
            let parent = self.parent.clone().unwrap();
            let mut parent = parent.lock().unwrap();
            parent.load(name);
            self.push(&parent.pop().unwrap());
        } else {
            self.push(&nil());
        }
    }

    pub fn load_local(&self, name: &str) -> Option<&Object> {
        self.locals.get(name)
    }

    pub fn store_local(&mut self, name: &str) {
        let value = self.pop().unwrap();
        self.locals.insert(name.to_string(), value);
    }
}

pub struct State {
    stack: Vec<Arc<Mutex<CallFrame>>>,
}

impl State {
    pub fn new() -> State {
        let mut result = State { stack: Vec::new() };
        result.push_frame();
        stdlib::register(&mut result);
        result
    }

    pub fn push_frame(&mut self) {
        let parent = self.current_frame();
        self.stack
            .push(Arc::new(Mutex::new(CallFrame::new(parent))));
    }

    pub fn pop_frame(&mut self) {
        self.stack.pop().expect("no call frame to pop");
    }

    pub fn current_frame(&mut self) -> Option<Arc<Mutex<CallFrame>>> {
        self.stack.last().cloned()
    }

    pub fn push(&mut self, object: &Object) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .push(object);
    }

    pub fn push_all(&mut self, objects: &[Object]) {
        objects.iter().for_each(|object| self.push(object));
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .pop()
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<Object> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.pop().unwrap());
        }
        result
    }

    pub fn peek(&mut self) -> Option<Object> {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .peek()
    }

    pub fn set_global(&mut self, name: &str, obj: Object) {
        self.stack
            .get(0)
            .expect("no global frame")
            .lock()
            .unwrap()
            .locals
            .insert(name.to_string(), obj);
    }

    pub fn store_local(&mut self, name: &str) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .store_local(name);
    }

    pub fn load(&mut self, name: &str) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .load(name);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
