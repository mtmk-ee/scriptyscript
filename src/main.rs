use std::io::Write;

use scriptyscript::{
    ast::parse,
    builtin::to_string,
    compiler::compile_node,
    object::Primitive,
    state::{execute, State},
};

fn interactive() {
    let mut state = State::new();

    loop {
        let input = next_statement();

        let ast = parse(&input);
        if let Err(e) = ast {
            println!("Error: {}", e);
            continue;
        }

        let bytecode = compile_node(&ast.unwrap()).expect("failed to compile ast");
        let pushed_amt = execute(&mut state, bytecode);
        if pushed_amt != 0 {
            let pushed_amt = to_string(&mut state, pushed_amt);
            assert_eq!(pushed_amt, 1);
            let primitive = state.pop().unwrap().as_primitive();
            match primitive {
                Some(Primitive::String(s)) => println!("{}", s),
                _ => panic!("expected string primitive"),
            }
        }
    }
}

fn next_statement() -> String {
    print!(">> ");
    let _ = std::io::stdout().lock().flush();
    // read input from user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_owned();
    if !input.ends_with(";") {
        input.push(';');
    }
    input
}

fn main() {
    interactive();
}
