//! Contains the standard library for the `ScriptyScript` language.
//!
//! These functions may be bound to a [`State`] and called from within a script.

use std::io::Write;

use crate::runtime::{
    executor::execute_source,
    state::State,
    types::{
        function::Function,
        object::ObjectValue,
        operations,
        primitive::Primitive,
        utilities::{float, int, nil, string, wrapped_function},
    },
};

pub fn register(state: &mut State) {
    state.set_global("print", wrapped_function(print));
    state.set_global("string", wrapped_function(to_string));
    state.set_global("max", wrapped_function(max));
    state.set_global("min", wrapped_function(min));
    state.set_global("int", wrapped_function(to_int));
    state.set_global("float", wrapped_function(to_float));
    state.set_global("round", wrapped_function(round));
    state.set_global("abs", wrapped_function(abs));
    state.set_global("exec", wrapped_function(exec));
    state.set_global("exit", wrapped_function(exit));
    state.set_global("input", wrapped_function(input));
}

/// Convert an object to its string representation.
///
/// Pops 1 argument, the object.
/// Pushes 1 result, the string representation of the object.
pub fn to_string(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);
    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => string(x.to_string()),
        Some(ObjectValue::Function(x)) => match x.as_ref() {
            Function::Scripted(x) => string(format!("scripted function: {:?}", x.bytecode())),
            Function::Wrapped(_) => string("wrapped function"),
        },
        Some(ObjectValue::Table(_)) => {
            todo!(); // need to invoke __str__
        }
        None => string("nil"),
    };
    state.push(&result);
    1
}

/// Print the string representation for one or more objects.
///
/// Pops `n` arguments, the objects to print.
/// Pushes no results.
pub fn print(state: &mut State, n: usize) -> usize {
    for _ in 0..n {
        let pushed = to_string(state, 1);
        assert_eq!(pushed, 1);
        let primitive = state.pop().unwrap().as_primitive();
        match primitive {
            Some(Primitive::String(s)) => print!("{s}"),
            _ => panic!("unsupported type"),
        }
    }
    // Add the final newline character
    if n != 0 {
        println!();
    }
    0
}

/// Compute the maximum of two or more numbers.
///
/// Pops `n` arguments, the numbers to compare. Takes at least two args.
/// Pushes 1 result, the maximum of the numbers.
pub fn max(state: &mut State, n: usize) -> usize {
    assert!(n >= 2);

    let mut max = state.pop().unwrap();
    for _ in 1..n {
        let current = state.pop().unwrap();
        operations::greater_than(state, &current, &max);

        match state.pop().unwrap().as_bool() {
            Some(true) => max = current,
            Some(false) => (),
            None => panic!("unsupported type"),
        }
    }
    state.push(&max);
    1
}

/// Compute the minimum of two or more numbers.
///
/// Pops `n` arguments, the numbers to compare. Takes at least two args.
/// Pushes 1 result, the minimum of the numbers.
pub fn min(state: &mut State, n: usize) -> usize {
    assert!(n >= 2);

    let mut min = state.pop().unwrap();
    for _ in 1..n {
        let current = state.pop().unwrap();
        operations::less_than(state, &current, &min);

        match state.pop().unwrap().as_bool() {
            Some(true) => min = current,
            Some(false) => (),
            None => panic!("unsupported type"),
        }
    }
    state.push(&min);
    1
}

/// Rounds a number to the nearest integer.
///
/// Pops 1 argument, the number to round.
/// Pushes 1 result, the rounded number.
pub fn round(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => int(*x),
            Primitive::Float(x) => int(x.round() as i64),
            Primitive::Boolean(x) => int(i64::from(*x)),
            _ => panic!("unsupported type"),
        },
        _ => panic!("unsupported type"),
    };
    state.push(&result);
    1
}

/// Convert a primitive value to an integer.
///
/// Parses strings to integers.
///
/// Pops 1 argument, the primitive value to convert.
/// Pushes 1 result, the integer value.
pub fn to_int(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => int(*x),
            Primitive::Float(x) => int(*x as i64),
            Primitive::Boolean(x) => int(i64::from(*x)),
            Primitive::String(x) => match x.parse::<u64>() {
                Ok(x) => int(x),
                Err(_) => nil(),
            },
            Primitive::Nil => nil(),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}

/// Convert a primitive value to a float.
///
/// Parses strings to floats.
///
/// Pops 1 argument, the primitive value to convert.
/// Pushes 1 result, the float value.
pub fn to_float(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => float(*x as f64),
            Primitive::Float(x) => float(*x),
            Primitive::Boolean(x) => float(f64::from(u8::from(*x))),
            Primitive::String(x) => match x.parse::<f64>() {
                Ok(x) => float(x),
                Err(_) => nil(),
            },
            Primitive::Nil => nil(),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}

/// Compute the absolute value of a number.
///
/// Pops 1 argument, the number to compute the absolute value of.
/// Pushes 1 result, the absolute value.
pub fn abs(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => int(x.abs()),
            Primitive::Float(x) => float(x.abs()),
            _ => nil(),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}

/// Executes a string as source code.
///
/// This will compile and execute the source code on the
/// current call frame.
///
/// Pops 1 argument, the string to execute.
/// Pushes 1 result, the result of the execution.
pub fn exec(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(Primitive::String(source))) => {
            let result = execute_source(state, source);
            match result {
                Ok(_) => state.pop().unwrap_or_else(nil),
                Err(e) => string(e.to_string()),
            }
        }
        _ => panic!("unsupported type"),
    };
    state.push(&result);
    1
}

/// Exits the program with the given status code.
///
/// Pops 1 argument, the status code.
/// Pushes 0 results.
pub fn exit(state: &mut State, n: usize) -> usize {
    assert!(n <= 1);

    let object = state.pop().unwrap_or_else(|| int(0));
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => {
                std::process::exit(*x as i32);
            }
            _ => panic!("expected integer"),
        },
        _ => panic!("expected primitive"),
    };
}

/// Read a line from stdin.
///
/// Pops 0 to 1 arguments, the prompt string or nothing.
/// Pushes 1 result, the line read from stdin.
pub fn input(state: &mut State, n: usize) -> usize {
    assert!(n <= 1);

    let object = state.pop().unwrap_or_else(|| string(""));
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::String(x) => {
                print!("{x}");
                let _ = std::io::stdout().lock().flush();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                // remove \n and \r
                string(&input[..input.len() - 2])
            }
            _ => panic!("expected string"),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}
