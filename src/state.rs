use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    object::{float, int, nil, string, Function, Object},
    opcode::OpCode,
};

pub struct CallFrame {
    parent: Option<Arc<Mutex<CallFrame>>>,
    operands: Vec<Object>,
    locals: HashMap<String, Object>,
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
        } else if let Some(parent) = &self.parent {
            parent.lock().unwrap().load(name);
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
        State { stack: Vec::new() }
    }

    pub fn push_frame(&mut self) {
        let parent = self.current_frame();
        self.stack.push(Arc::new(Mutex::new(CallFrame::new(
            parent,
        ))));
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
}

pub fn execute(state: &mut State, bytecode: Vec<OpCode>) -> usize {
    let frame = state.current_frame().expect("no call frame");
    let mut pointer = 0;

    let stack_size = || frame.lock().unwrap().operands.len();
    let initial_stack_size = stack_size();

    while pointer < bytecode.len() {
        let opcode = &bytecode[pointer];
        pointer += 1;

        match opcode {
            OpCode::PushInteger(x) => {
                frame.lock().unwrap().push(&int(*x));
            }
            OpCode::PushFloat(x) => {
                frame.lock().unwrap().push(&float(*x));
            }
            OpCode::PushString(x) => {
                frame.lock().unwrap().push(&string(&x));
            }
            OpCode::Store(identifier) => {
                frame.lock().unwrap().store_local(identifier);
            }
            OpCode::Load(identifier) => {
                frame.lock().unwrap().load(identifier);
            }
            OpCode::SetKey(key) => {
                let value = frame.lock().unwrap().pop().unwrap();
                let mut table_obj = frame.lock().unwrap().pop().unwrap();
                table_obj.set_key(key, value);
            }
            OpCode::GetKey(key) => {
                let table = frame.lock().unwrap().pop().unwrap();
                let value = table.get_key(key).unwrap_or_else(|| nil());
                frame.lock().unwrap().push(&value);
            }
            OpCode::Call(n) => {
                let function = frame.lock().unwrap().pop();
                function.unwrap().call(state, *n);
            }
            OpCode::Duplicate => {
                let value = frame.lock().unwrap().peek().unwrap();
                frame.lock().unwrap().push(&value);
            }
            _ => todo!(),
        }
    }

    if stack_size() < initial_stack_size {
        panic!("stack corrupted");
    }

    stack_size() - initial_stack_size
}
