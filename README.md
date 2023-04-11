# ScriptyScript!

A small but fun toy scripting language written in Rust.

It's not fast, and it eats more memory than it should, but it's fun to play with!

## Language Features

Current language features include:
- Variables
- Functions
    - Recursion
    - Bindings for Rust-side functions
- Loops
    - `while`
    - `for`
    - `loop` (infinite loop)
- If/else-if/else statements
- Comments (single line and multi-line)
- Arbitrary expressions

Currently there is no concept of exception handling, meaning that certain
invalid operations will cause the program to panic. Exception handling is planned
for the future.


## Language Example

The syntax is very simple and similar to other languages.

This code:
```
fib = fn(n) {
    if n <= 1 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
};

for (n = 0; n < 10; n = n + 1) {
    print("fib_" + to_string(n) + " = " + to_string(fib(n)));
}
```

Will output:
```
fib_0 = 0
fib_1 = 1
fib_2 = 1
fib_3 = 2
fib_4 = 3
fib_5 = 5
fib_6 = 8
fib_7 = 13
fib_8 = 21
fib_9 = 34
```

## Interactive REPL

The REPL is entered if no file arguments are passed:

```
cargo run --release
```

Currently inputs are limited to single lines.

## Running the Example Scripts

There are a few example scripts written in ScriptyScript located in the `examples` folder. This one runs the math script:

```
cargo run --release examples\math.ss
```
