//! The brains of the operation. This module contains the code that executes the bytecode
//! on a [`State`](`crate::runtime::state::State`).
//!
//! The executor is structured as many nested function calls. Each function call represents
//! an "execution layer". For example, when a function is called, a new execution layer is
//! run on the function body. When an if statement is encountered, a new execution layer is
//! run on either the `if` or the `else` bodies. This structure is simple, but extremely
//! powerful when used correctly. It allows for safe recursion and complex control flow.
//!
//! Note that the documentation for some functions in this module may show information on
//! how they modify the stack. This information is shown as:
//!
//! ```
//! stack: [first pop, second pop, ...] -> [first push, second push, ...]
//! ```
//!
//! When the stack modification is ambiguous, the documentation will show the stack as the "nose and glasses":
//!
//! Stack: `[*] -> [*]`

use self::{
    control_flow::ControlFlow,
    expressions::{execute_binary_operation, execute_function_call, execute_unary_operation},
};
use super::{
    bytecode::{Bytecode, OpCode},
    state::State,
    types::utilities::{boolean, float, int, nil, scripted_function, string},
};
use crate::{
    compiler::compile,
    runtime::executor::control_flow::{
        execute_for_loop, execute_if_statement, execute_infinite_loop, execute_while_loop,
        function_layer_control_flow,
    },
};

/// Whether or not to print debug information when executing.
///
/// Use only for debugging.
const STACK_DEBUG: bool = false;

/// Parse, compile, and run the input string on the given state.
///
/// Returns the number of objects pushed onto the stack.
///
/// # Errors
/// anyhow::Error if there is a problem parsing or compiling the input.
pub fn execute_source(state: &mut State, input: &str) -> Result<usize, anyhow::Error> {
    let bytecode = compile(input)?;
    let pushed_amt = execute(state, &bytecode);
    Ok(pushed_amt)
}

/// Execute the given bytecode on the given state.
///
/// Returns the number of objects pushed onto the stack.
pub(crate) fn execute(state: &mut State, bytecode: &Bytecode) -> usize {
    match run_execution_layer(state, bytecode) {
        ControlFlow::Return(n) => n,
        _ => 0,
    }
}

/// Run the given bytecode on the given state.
///
/// This serves as running a new execution layer.
/// See the [`module`](self) documentation for more information.
///
/// Stack: `[*] -> [*]`
fn run_execution_layer(state: &mut State, bytecode: &Bytecode) -> ControlFlow {
    for opcode in bytecode.iter() {
        if STACK_DEBUG {
            println!("=================================");
            println!("stack: {:?}", state.operand_stack_size());
            println!("executing opcode: {:?}", opcode);
        }

        // This may exit the current execution layer early.
        function_layer_control_flow!(execute_operation(state, opcode));
    }

    ControlFlow::None
}

/// Execute a single operation on the given state.
///
/// Returns a [`ControlFlow`] enum which may indicate that the current execution layer
/// needs to exit early.
///
/// Stack: `[*] -> [*]`
fn execute_operation(state: &mut State, opcode: &OpCode) -> ControlFlow {
    match opcode {
        // ======================== Stack Operations ========================
        OpCode::Store(identifier) => state.store_local(identifier),
        OpCode::Load(identifier) => state.load(identifier),
        OpCode::SetKey(key) => {
            let value = state.pop().unwrap();
            let mut table_obj = state.pop().unwrap();
            table_obj.set_key(key, value);
        }
        OpCode::GetKey(key) => {
            let table = state.pop().unwrap();
            let value = table.get_key(key).unwrap_or_else(nil);
            state.push(&value);
        }

        // ======================== Push Operations ========================
        OpCode::PushInteger(x) => state.push(&int(*x)),
        OpCode::PushFloat(x) => state.push(&float(*x)),
        OpCode::PushString(x) => state.push(&string(x)),
        OpCode::PushBool(x) => state.push(&boolean(*x)),
        OpCode::PushFunction(x) => state.push(&scripted_function(x.clone())),
        OpCode::PushNil => state.push(&nil()),

        // ======================== Expressions ========================
        OpCode::BinaryOperation(op) => execute_binary_operation(state, *op),
        OpCode::UnaryOperation(op) => execute_unary_operation(state, *op),
        OpCode::Call(n) => execute_function_call(state, *n),

        // ======================== Control Flow ========================
        OpCode::Return(n) => return ControlFlow::Return(*n),
        OpCode::Break => return ControlFlow::Break,
        OpCode::Continue => return ControlFlow::Continue,
        opcode @ OpCode::If { .. } => {
            function_layer_control_flow!(execute_if_statement(state, opcode));
        }
        opcode @ OpCode::For { .. } => {
            function_layer_control_flow!(execute_for_loop(state, opcode));
        }
        opcode @ OpCode::While { .. } => {
            function_layer_control_flow!(execute_while_loop(state, opcode));
        }
        opcode @ OpCode::Loop { .. } => {
            function_layer_control_flow!(execute_infinite_loop(state, opcode));
        }
    };
    ControlFlow::None
}

/// Executors for more complex expression operations.
pub(self) mod expressions {
    use std::borrow::Borrow;

    use crate::{
        compiler::{BinaryOperationKind, UnaryOperationKind},
        runtime::{
            executor::execute,
            state::State,
            types::{function::Function, object::ObjectValue, operations},
        },
    };

    /// Execute a binary operation on the given state. The type of operation
    /// is indicated by the [`BinaryOperationKind`].
    ///
    /// Stack: `[rhs, lhs] -> result`
    pub fn execute_binary_operation(state: &mut State, kind: BinaryOperationKind) {
        let right = state.pop().unwrap();
        let left = state.pop().unwrap();
        match kind {
            BinaryOperationKind::Add => operations::add(state, &left, &right),
            BinaryOperationKind::Subtract => operations::subtract(state, &left, &right),
            BinaryOperationKind::Multiply => operations::multiply(state, &left, &right),
            BinaryOperationKind::Divide => operations::divide(state, &left, &right),
            BinaryOperationKind::Remainder => operations::remainder(state, &left, &right),
            BinaryOperationKind::Equal => operations::equals(state, &left, &right),
            BinaryOperationKind::NotEqual => operations::not_equals(state, &left, &right),
            BinaryOperationKind::GreaterThan => operations::greater_than(state, &left, &right),
            BinaryOperationKind::GreaterThanOrEqual => {
                operations::greater_than_or_equal(state, &left, &right)
            }
            BinaryOperationKind::LessThan => operations::less_than(state, &left, &right),
            BinaryOperationKind::LessThanOrEqual => {
                operations::less_than_or_equal(state, &left, &right)
            }
            BinaryOperationKind::And => operations::and(state, &left, &right),
            BinaryOperationKind::Or => operations::or(state, &left, &right),
            _ => unimplemented!("binary operation is unimplemented: {:?}", kind),
        };
    }

    /// Execute a unary operation on the given state. The type of operation
    /// is indicated by the [`UnaryOperationKind`].
    ///
    /// Stack: `operand -> result`
    pub fn execute_unary_operation(state: &mut State, kind: UnaryOperationKind) {
        let operand = state.pop().unwrap();
        match kind {
            UnaryOperationKind::Negate => operations::negate(state, &operand),
            _ => unimplemented!("unary operation is unimplemented: {:?}", kind),
        };
    }

    /// Execute a function call on the given state.
    ///
    /// For scripted functions this will run a new execution layer on the function body.
    /// For wrapped functions this will call the function directly.
    ///
    /// Stack: `[arg n-1, arg n-2, ... arg 0] -> [return n-1, return n-2, return 0]`
    pub fn execute_function_call(state: &mut State, n: usize) {
        let function = {
            let function = state.pop().unwrap();
            let function = function.inner.lock().unwrap();
            match &function.value {
                Some(ObjectValue::Function(f)) => f.clone(),
                _ => panic!("Cannot call non-function object"),
            }
        };

        let args = state.pop_n(n);
        state.push_frame();
        state.push_all(&args);
        let push_amt = match function.borrow() {
            Function::Wrapped(f) => f(state, n),
            Function::Scripted(f) => execute(state, f.bytecode()),
        };
        let returns = state.pop_n(push_amt);
        state.pop_frame();
        state.push_all(&returns);
    }
}

/// Executors for control flow operations.
pub(self) mod control_flow {
    use crate::runtime::{
        bytecode::OpCode,
        executor::{execute, run_execution_layer},
        state::State,
    };

    /// Executes an if statement, conditionally executing the "then" body or the "else" body.
    /// Note that else-if is implemented as an if statement nested under an else body.
    ///
    /// Stack: `[] -> []`
    pub fn execute_if_statement(state: &mut State, opcode: &OpCode) -> ControlFlow {
        let (condition, body, else_body) = match opcode {
            OpCode::If {
                condition,
                body,
                else_body,
            } => (condition, body, else_body),
            _ => unreachable!(),
        };
        execute(state, condition);
        let condition = state.pop().expect("no condition");
        if let Some(condition) = condition.as_bool() {
            if condition {
                function_layer_control_flow!(run_execution_layer(state, body));
            } else if let Some(else_body) = else_body {
                function_layer_control_flow!(run_execution_layer(state, else_body));
            }
        } else {
            // TODO: exception handling
            panic!("expected boolean condition");
        }
        ControlFlow::None
    }

    /// Executes a for loop.
    ///
    /// Stack: `[] -> []`
    pub fn execute_for_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
        let (initialization, condition, increment, body) = match op_code {
            OpCode::For {
                initialization,
                condition,
                increment,
                body,
            } => (initialization, condition, increment, body),
            _ => unreachable!(),
        };
        if let Some(initialization) = initialization {
            execute(state, initialization);
        }
        loop {
            let condition_result = match condition {
                Some(condition) => {
                    execute(state, condition);
                    let result = state.pop().expect("no condition");
                    result.as_bool().expect("expected boolean condition")
                }
                None => true,
            };
            if condition_result {
                loop_layer_control_flow!(run_execution_layer(state, body));
                if let Some(increment) = increment {
                    execute(state, increment);
                }
            } else {
                break;
            }
        }
        ControlFlow::None
    }

    /// Executes a while loop.
    ///
    /// Stack: `[] -> []`
    pub fn execute_while_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
        let (condition, body) = match op_code {
            OpCode::While { condition, body } => (condition, body),
            _ => unreachable!(),
        };
        loop {
            execute(state, condition);
            let condition_result = state.pop().expect("no condition");
            if let Some(condition_result) = condition_result.as_bool() {
                if condition_result {
                    loop_layer_control_flow!(run_execution_layer(state, body));
                } else {
                    break;
                }
            }
        }
        ControlFlow::None
    }

    /// Executes an infinite loop.
    ///
    /// Stack: `[] -> []`
    pub fn execute_infinite_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
        let body = match op_code {
            OpCode::Loop { body } => body,
            _ => unreachable!(),
        };
        loop {
            loop_layer_control_flow!(run_execution_layer(state, body));
        }
        ControlFlow::None
    }

    /// A macro to propagate control flow out of nested execution layers.
    /// This macro is used when executing nested layers in a function body
    ///
    /// This will immediately return control to the appropriate layer.
    macro_rules! function_layer_control_flow {
        ($cf:expr) => {
            match $cf {
                ControlFlow::Return(n) => return ControlFlow::Return(n),
                ControlFlow::Break => return ControlFlow::Break,
                ControlFlow::Continue => return ControlFlow::Continue,
                ControlFlow::None => {}
            }
        };
    }

    /// A macro to perform a loop control flow operation inside of an actual Rust loop,
    /// or to propagate return control flow out of nested execution layers.
    /// This macro is used when executing within a loop body
    ///
    /// This will immediately break or continue a loop, or return control out of the loop layer.
    macro_rules! loop_layer_control_flow {
        ($cf:expr) => {
            match $cf {
                ControlFlow::Return(n) => return ControlFlow::Return(n),
                ControlFlow::Break => break,
                ControlFlow::Continue => continue,
                ControlFlow::None => {}
            }
        };
    }

    /// An enum representing the different types of control flow operations.
    /// This is used to jump out of nested execution layers to the appropriate layer,
    /// where further action may be taken.
    pub enum ControlFlow {
        /// Causes the control flow to be propagated up to the current function call execution layer.
        Return(usize),
        /// Causes the control flow to return to the loop execution layer, and break out of the loop.
        Break,
        /// Causes the control flow to return to the loop execution layer, and continue the loop.
        Continue,
        /// No-op.
        None,
    }

    pub(crate) use function_layer_control_flow;
    pub(crate) use loop_layer_control_flow;
}
