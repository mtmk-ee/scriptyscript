use crate::runtime::{
    executor::execute_source,
    state::State,
    types::{
        function::Function,
        object::{greater_than, less_than, ObjectValue},
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
}

pub fn to_string(state: &mut State, n: usize) -> usize {
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
    n
}

pub fn print(state: &mut State, n: usize) -> usize {
    for _ in 0..n {
        let pushed = to_string(state, 1);
        assert_eq!(pushed, 1);
        let primitive = state.pop().unwrap().as_primitive();
        match primitive {
            Some(Primitive::String(s)) => print!("{} ", s),
            _ => panic!("expected string primitive"),
        }
    }
    if n != 0 {
        println!();
    }
    0
}

pub fn max(state: &mut State, n: usize) -> usize {
    assert_ne!(n, 0);

    let mut max = state.pop().unwrap();
    for _ in 1..n {
        let current = state.pop().unwrap();
        greater_than(state, &current, &max);

        match state.pop().unwrap().as_bool() {
            Some(true) => max = current,
            Some(false) => (),
            None => panic!("expected boolean"),
        }
    }
    state.push(&max);
    1
}

pub fn min(state: &mut State, n: usize) -> usize {
    assert_ne!(n, 0);

    let mut min = state.pop().unwrap();
    for _ in 1..n {
        let current = state.pop().unwrap();
        less_than(state, &current, &min);

        match state.pop().unwrap().as_bool() {
            Some(true) => min = current,
            Some(false) => (),
            None => panic!("expected boolean"),
        }
    }
    state.push(&min);
    1
}

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
            Primitive::Boolean(x) => int(*x as i64),
            _ => nil(),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}

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
            Primitive::Boolean(x) => int(*x as i64),
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

pub fn to_float(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::Integer(x) => float(*x as f64),
            Primitive::Float(x) => float(*x as f64),
            Primitive::Boolean(x) => float(*x as u8 as f64),
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

pub fn exec(state: &mut State, n: usize) -> usize {
    assert_eq!(n, 1);

    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => match x {
            Primitive::String(source) => {
                let result = execute_source(state, &source);
                match result {
                    Ok(_) => state.pop().unwrap_or_else(nil),
                    Err(e) => string(e.to_string()),
                }
            }
            _ => panic!("expected string"),
        },
        _ => panic!("expected primitive"),
    };
    state.push(&result);
    1
}

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
