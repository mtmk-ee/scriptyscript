//! Module containing the [`State`] type, which represents the memory portion of the program.
//!
//! The [`State`] type is acted upon by the [executor](crate::runtime::executor) (see documentation
//! for more details).
//!
//! A state may be passed around and mutated, with the same bytecode possibly resulting in different
//! outcomes based on the current state.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::types::{object::Object, utilities::nil};
use crate::stdlib;

/// Representation of the memory portion of the program;
/// this structure holds the call stack, including the global call frame.
///
/// # Warning
/// A script can only be executed with respect to one state at a time.
/// Swapping states partway through execution immediately causes
/// undefined behavior!
pub struct State {
    /// Call stack. The last element is the current frame, which the
    /// executor primarily operates on.
    stack: Vec<Arc<Mutex<CallFrame>>>,
}

impl State {
    /// Create a fresh state.
    ///
    /// The state will have a single call frame, the "global frame".
    /// The [`stdlib`](crate::stdlib) will be registered in the global frame.
    pub fn new() -> State {
        let mut result = State { stack: Vec::new() };
        result.push_frame();
        stdlib::register(&mut result);
        result
    }

    /// Push a new call frame onto the stack.
    ///
    /// The new frame will have no locals.
    pub fn push_frame(&mut self) {
        let frame = match self.current_frame() {
            Some(parent) => CallFrame::with_parent(parent),
            None => CallFrame::new(),
        };
        self.stack.push(Arc::new(Mutex::new(frame)));
    }

    /// Pop the current call frame off the stack.
    pub fn pop_frame(&mut self) {
        self.stack.pop().expect("no call frame to pop");
    }

    /// Get a mutable reference to the current call frame.
    fn current_frame(&self) -> Option<Arc<Mutex<CallFrame>>> {
        self.stack.last().cloned()
    }

    /// Push an object onto the current call frame's operand stack.
    pub fn push(&mut self, object: &Object) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .push(object);
    }

    /// Push multiple objects onto the current call frame's operand stack.
    ///
    /// Objects will be pushed in the same order as given, meaning the
    /// last element provided will be on top of the stack.
    pub fn push_all(&mut self, objects: &[Object]) {
        objects.iter().for_each(|object| self.push(object));
    }

    /// Pop an object off the current call frame's operand stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<Object> {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .pop()
    }

    /// Pop multiple objects off the current call frame's operand stack.
    ///
    /// The returned vector will contain the objects in the same order
    /// as they were popped off the stack, meaning the first element
    /// is the former top of the stack.
    pub fn pop_n(&mut self, n: usize) -> Vec<Object> {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push(self.pop().unwrap());
        }
        result
    }

    /// Peek at the top of the current call frame's operand stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn peek(&mut self) -> Option<Object> {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .peek()
    }

    /// Set a global variable.
    ///
    /// Internally this stores the object as a local in the
    /// bottom-most call frame.
    pub fn set_global(&mut self, name: &str, obj: Object) {
        self.stack
            .get(0)
            .expect("no global frame")
            .lock()
            .unwrap()
            .locals
            .insert(name.to_string(), obj);
    }

    /// Store a local variable into the current call frame.
    ///
    /// Stack: `[value] -> []`
    pub fn store_local(&mut self, name: &str) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .store_local(name);
    }

    /// Load a local variable from the current call frame.
    ///
    /// Stack: `[] -> [value]`
    pub fn load(&mut self, name: &str) {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .load(name);
    }

    /// Get the size of the operand stack of the current call frame.
    pub fn operand_stack_size(&self) -> usize {
        self.current_frame()
            .expect("no call frame")
            .lock()
            .unwrap()
            .operands
            .len()
    }
}

impl Default for State {
    /// Same as `State::new()`
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a single frame in the call stack.
///
/// A frame is a essentially context in which bytecode can be executed.
/// This means that a frame has an isolated stack of operands and a set of local variables
/// which can be accessed through the bytecode.
pub struct CallFrame {
    /// The parent frame, if any.
    pub parent: Option<Arc<Mutex<CallFrame>>>,
    /// The operand stack.
    pub operands: Vec<Object>,
    /// The local variables.
    pub locals: HashMap<String, Object>,
}

impl CallFrame {
    /// Create a new call frame with the given parent, if any.
    pub fn with_parent(parent: Arc<Mutex<CallFrame>>) -> Self {
        let mut result = Self::new();
        result.parent = Some(parent);
        result
    }

    /// Create a new call frame with no parent.
    pub fn new() -> Self {
        Self {
            parent: None,
            operands: Vec::new(),
            locals: HashMap::new(),
        }
    }

    /// Push an object onto the operand stack.
    pub fn push(&mut self, object: &Object) {
        self.operands.push(object.clone());
    }

    /// Pop an object off the operand stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<Object> {
        self.operands.pop()
    }

    /// Peek at the top of the operand stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn peek(&self) -> Option<Object> {
        self.operands.last().cloned()
    }

    /// Get the number of objects on the operand stack.
    pub fn stack_size(&self) -> usize {
        self.operands.len()
    }

    /// Load a local variable from the current frame. If the variable is not
    /// found in the current frame, the parent frames will be searched recursively.
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

    /// Load a local variable from the current frame (non-recursive).
    ///
    /// Returns `None` if the variable is not found.
    pub fn load_local(&self, name: &str) -> Option<&Object> {
        self.locals.get(name)
    }

    /// Store a local variable into the current frame.
    ///
    /// Stack: `[value] -> []`
    pub fn store_local(&mut self, name: &str) {
        let value = self.pop().unwrap();
        self.locals.insert(name.to_string(), value);
    }
}

impl Default for CallFrame {
    /// Same as `CallFrame::new()`
    fn default() -> Self {
        Self::new()
    }
}
