
use scriptyscript::{ast::{parse}, compiler::compile_node};



fn main() {
    let ast = parse("x = (5 + 5) * 5;").unwrap();
    println!("{:?}", compile_node(&ast).unwrap());
}
