use std::borrow::Borrow;

use crate::{
    compiler::compile,
    runtime::types::{function::Function, object::ObjectValue},
};

use super::{
    opcode::OpCode,
    state::State,
    types::{
        object::{
            add, and, divide, equals, greater_than, greater_than_or_equal, less_than,
            less_than_or_equal, multiply, negate, not_equals, or, remainder, subtract,
        },
        utilities::{boolean, float, int, nil, scripted_function, string},
    },
};


macro_rules! propagate_control_flow {
    ($cf:expr) => {
        match $cf {
            ControlFlow::Return(n) => return ControlFlow::Return(n),
            ControlFlow::Break => return ControlFlow::Break,
            ControlFlow::Continue => return ControlFlow::Continue,
            ControlFlow::None => {}
        }
    };
}

macro_rules! perform_loop_control_flow {
    ($cf:expr) => {
        match $cf {
            ControlFlow::Return(n) => return ControlFlow::Return(n),
            ControlFlow::Break => break,
            ControlFlow::Continue => continue,
            ControlFlow::None => {}
        }
    };
}

enum ControlFlow {
    Return(usize),
    Break,
    Continue,
    None,
}

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

pub fn execute(state: &mut State, bytecode: &Vec<OpCode>) -> usize {
    match execute_impl(state, bytecode) {
        ControlFlow::Return(n) => n,
        _ => 0,
    }
}

fn execute_impl(state: &mut State, bytecode: &Vec<OpCode>) -> ControlFlow {
    let frame = state.current_frame().expect("no call frame");
    let mut pointer = 0;

    while pointer < bytecode.len() {
        let opcode = &bytecode[pointer];
        pointer += 1;

        // println!("=================================");
        // println!("executing opcode: {:?}", opcode);

        match opcode {
            OpCode::PushInteger(x) => {
                frame.lock().unwrap().push(&int(*x));
            }
            OpCode::PushFloat(x) => {
                frame.lock().unwrap().push(&float(*x));
            }
            OpCode::PushString(x) => {
                frame.lock().unwrap().push(&string(x));
            }
            OpCode::PushBool(x) => {
                frame.lock().unwrap().push(&boolean(*x));
            }
            OpCode::PushFunction(x) => {
                frame.lock().unwrap().push(&scripted_function(x.clone()));
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
                let value = table.get_key(key).unwrap_or_else(nil);
                frame.lock().unwrap().push(&value);
            }
            OpCode::Call(n) => {
                call(state, *n);
            }
            OpCode::Return(n) => {
                return ControlFlow::Return(*n);
            }
            OpCode::Break => {
                return ControlFlow::Break;
            }
            OpCode::Continue => {
                return ControlFlow::Continue;
            }
            opcode @ OpCode::If { .. } => {
                propagate_control_flow!(if_statement(state, opcode));
            }
            opcode @ OpCode::For { .. } => {
                propagate_control_flow!(for_loop(state, opcode));
            }
            opcode @ OpCode::While { .. } => {
                propagate_control_flow!(while_loop(state, opcode));
            }
            opcode @ OpCode::Loop { .. } => {
                propagate_control_flow!(infinite_loop(state, opcode));
            }
            OpCode::Duplicate => {
                let value = frame.lock().unwrap().peek().unwrap();
                frame.lock().unwrap().push(&value);
            }
            opcode @ (OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::Remainder
            | OpCode::Equal
            | OpCode::NotEqual
            | OpCode::GreaterThan
            | OpCode::GreaterThanOrEqual
            | OpCode::LessThan
            | OpCode::LessThanOrEqual
            | OpCode::And
            | OpCode::Or) => binary_operation(state, opcode),
            opcode @ OpCode::Negate => {
                unary_operation(state, opcode);
            }
        };
        // println!(
        //     "stack: {:?}",
        //     state.current_frame().unwrap().lock().unwrap().operands
        // );
    }

    ControlFlow::None
}


fn binary_operation(state: &mut State, opcode: &OpCode) {
    let right = state.pop().unwrap();
    let left = state.pop().unwrap();
    match opcode {
        OpCode::Add => add(state, &left, &right),
        OpCode::Subtract => subtract(state, &left, &right),
        OpCode::Multiply => multiply(state, &left, &right),
        OpCode::Divide => divide(state, &left, &right),
        OpCode::Remainder => remainder(state, &left, &right),
        OpCode::Equal => equals(state, &left, &right),
        OpCode::NotEqual => not_equals(state, &left, &right),
        OpCode::GreaterThan => greater_than(state, &left, &right),
        OpCode::GreaterThanOrEqual => greater_than_or_equal(state, &left, &right),
        OpCode::LessThan => less_than(state, &left, &right),
        OpCode::LessThanOrEqual => less_than_or_equal(state, &left, &right),
        OpCode::And => and(state, &left, &right),
        OpCode::Or => or(state, &left, &right),
        _ => unreachable!(),
    };
}

fn unary_operation(state: &mut State, opcode: &OpCode) {
    let operand = state.pop().unwrap();
    match opcode {
        OpCode::Negate => negate(state, &operand),
        _ => unreachable!(),
    };
}

fn for_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
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
            perform_loop_control_flow!(execute_impl(state, body));
            if let Some(increment) = increment {
                execute(state, increment);
            }
        } else {
            break;
        }
    }
    ControlFlow::None
}

fn while_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
    let (condition, body) = match op_code {
        OpCode::While { condition, body } => (condition, body),
        _ => unreachable!(),
    };
    loop {
        execute(state, condition);
        let condition_result = state.pop().expect("no condition");
        if let Some(condition_result) = condition_result.as_bool() {
            if condition_result {
                perform_loop_control_flow!(execute_impl(state, body));
            } else {
                break;
            }
        }
    }
    ControlFlow::None
}

fn infinite_loop(state: &mut State, op_code: &OpCode) -> ControlFlow {
    let body = match op_code {
        OpCode::Loop { body } => body,
        _ => unreachable!(),
    };
    loop {
        perform_loop_control_flow!(execute_impl(state, body));
    }
    ControlFlow::None
}

fn if_statement(state: &mut State, opcode: &OpCode) -> ControlFlow {
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
            propagate_control_flow!(execute_impl(state, body));
        } else if let Some(else_body) = else_body {
            propagate_control_flow!(execute_impl(state, else_body));
        }
    } else {
        // TODO: exception handling
        panic!("expected boolean condition");
    }
    ControlFlow::None
}

fn call(state: &mut State, n: usize) {
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
