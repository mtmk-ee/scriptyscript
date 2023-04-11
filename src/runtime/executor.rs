use super::{
    opcode::OpCode,
    state::State,
    types::{
        object::{add, divide, multiply, negate, remainder, subtract},
        utilities::{boolean, float, int, nil, string},
    },
};

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
                frame.lock().unwrap().push(&string(x));
            }
            OpCode::PushBool(x) => {
                frame.lock().unwrap().push(&boolean(*x));
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
                let function = frame.lock().unwrap().pop();
                function.unwrap().call(state, *n);
            }
            OpCode::Duplicate => {
                let value = frame.lock().unwrap().peek().unwrap();
                frame.lock().unwrap().push(&value);
            }
            opcode @ (OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::Remainder) => {
                let right = frame.lock().unwrap().pop().unwrap();
                let left = frame.lock().unwrap().pop().unwrap();
                match opcode {
                    OpCode::Add => add(state, &left, &right),
                    OpCode::Subtract => subtract(state, &left, &right),
                    OpCode::Multiply => multiply(state, &left, &right),
                    OpCode::Divide => divide(state, &left, &right),
                    OpCode::Remainder => remainder(state, &left, &right),
                    _ => unreachable!(),
                };
            }
            opcode @ OpCode::Negate => {
                let value = frame.lock().unwrap().pop().unwrap();
                match opcode {
                    OpCode::Negate => negate(state, &value),
                    _ => unreachable!(),
                };
            }
        }
    }

    if stack_size() < initial_stack_size {
        panic!("stack corrupted");
    }

    stack_size() - initial_stack_size
}
