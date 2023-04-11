use std::{io::Write, time::Instant};

use scriptyscript::{
    ast::parse,
    builtin::to_string,
    compiler::compile_node,
    object::Primitive,
    state::{execute, State},
};

/// Fun macro for timing an expression
macro_rules! timeit {
    ($name:expr, $expr:expr) => {{
        let start = Instant::now();
        let result = $expr;
        println!("{}: {} us", $name, start.elapsed().as_micros());
        result
    }};
}

/// Main entry point for the REPL.
fn repl() {
    let mut state = State::new();

    loop {
        let input = next_statement();

        let pushed_amt = timeit!("total (including prints)", run(&mut state, &input));
        if let Err(e) = pushed_amt {
            println!("Error: {}", e);
            continue;
        }
        display_top(&mut state, pushed_amt.unwrap());
    }
}

fn display_top(state: &mut State, pushed_amt: usize) {
    if pushed_amt != 0 {
        let pushed_amt = to_string(state, pushed_amt);
        assert_eq!(pushed_amt, 1);
        let primitive = state.pop().unwrap().as_primitive();
        match primitive {
            Some(Primitive::String(s)) => println!("{}", s),
            _ => panic!("expected string primitive"),
        }
    }
}

/// Parse, compile, and run the input string on the given state.
///
/// Returns the number of objects pushed onto the stack.
///
/// # Errors
/// anyhow::Error if there is a problem parsing or compiling the input.
fn run(state: &mut State, input: &str) -> Result<usize, anyhow::Error> {
    let ast = timeit!("parsing", parse(&input))?;
    let bytecode = timeit!("compilation", compile_node(&ast))?;
    let pushed_amt = timeit!("execution", execute(state, bytecode));
    Ok(pushed_amt)
}

/// Read a statement from the user.
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
    repl();
}
