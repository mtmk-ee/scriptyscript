use crate::runtime::{types::{utilities::{wrapped_function, string}, object::ObjectValue, primitive::Primitive}, state::State};




pub fn register(state: &mut State) {
    state.set_global("print", wrapped_function(print));
    state.set_global("to_string", wrapped_function(to_string));
}

pub fn to_string(state: &mut State, n: usize) -> usize {
    let object = state.pop().unwrap();
    let inner = object.inner();
    let value = inner.lock().unwrap();
    let value = value.value();
    let result = match value {
        Some(ObjectValue::Primitive(x)) => string(x.to_string()),
        Some(ObjectValue::Function(_)) => string("function"),
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
            Some(Primitive::String(s)) => println!("{}", s),
            _ => panic!("expected string primitive"),
        }
    }
    0
}
