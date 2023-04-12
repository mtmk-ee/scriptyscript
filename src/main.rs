//! Executable for the scriptyscript interpreter.
//!
//! Can be run without arguments to enter the REPL, or with a file path to run a script.

use std::path::{Path, PathBuf};

use clap::Parser;

use scriptyscript::runtime::{executor::execute_source, state::State};

/// Program arguments.
#[derive(clap::Parser)]
struct Arguments {
    /// Script file to run
    file: Option<PathBuf>,
    /// Show compiler output for the given file
    #[arg(short, long, default_value_t = false)]
    bytecode: bool,
}

fn main() {
    let args = Arguments::parse();
    let mut state = State::new();

    if let Some(file) = args.file {
        if args.bytecode {
            show_bytecode(file);
        } else {
            run_file(&mut state, file);
        }
    } else {
        repl::run(&mut state);
    }
}

/// Run a script file on the given state.
fn run_file(state: &mut State, file: impl AsRef<Path>) {
    let source = std::fs::read_to_string(file).unwrap();
    execute_source(state, &source).unwrap();
}

/// Show the compiled bytecode for a script file.
fn show_bytecode(file: impl AsRef<Path>) {
    let source = std::fs::read_to_string(file).unwrap();
    let bytecode = scriptyscript::compiler::compile(source).unwrap();
    println!("{:?}", bytecode);
}

/// REPL-related functionality.
mod repl {
    use std::io::Write;

    use scriptyscript::{
        runtime::{executor::execute_source, state::State, types::primitive::Primitive},
        stdlib::to_string,
    };

    /// Main entry point for the REPL.
    ///
    /// Runs continuously until the user exits.
    pub fn run(state: &mut State) {
        loop {
            let input = next_statement();

            let pushed_amt = execute_source(state, &input);
            if let Err(e) = pushed_amt {
                println!("Error: {}", e);
                continue;
            }
            display_top(state);
        }
    }

    /// Display the object at the top of the stack.
    ///
    /// Will pop the object from the stack, if it exists.
    fn display_top(state: &mut State) {
        if state.peek().is_some() {
            let pushed_amt = to_string(state, 1);
            assert_eq!(pushed_amt, 1);
            let primitive = state.pop().unwrap().as_primitive();
            match primitive {
                Some(Primitive::String(s)) => println!("{}", s),
                _ => panic!("expected string primitive"),
            }
        }
    }

    /// Read a statement from the user.
    fn next_statement() -> String {
        print!(">> ");
        let _ = std::io::stdout().lock().flush();
        // read input from user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_owned();
        if !input.ends_with(';') {
            input.push(';');
        }
        input
    }
}
