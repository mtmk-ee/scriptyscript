# ScriptyScript!

A small but fun toy scripting language written in Rust.

## Example Script

The syntax is very simple and similar to other languages.

```
>> x = (1 + 2) * 3 - 4 / -5.0;
>> print("The value is " + to_string(x));
The value is 9.8
```

## Interactive REPL

The REPL is entered if no file arguments are passed:

```
cargo run --release
```

## Running the Example Scripts

There are a few example scripts written in ScriptyScript located in the `examples` folder. This one runs the math script:

```
cargo run --release examples\math.ss
```
