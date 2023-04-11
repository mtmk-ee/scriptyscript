# ScriptyScript!

A small but fun toy scripting language written in Rust.

## Language Example

The syntax is very simple and similar to other languages.

This code:
```
print_hello = fn() {
    print("Hello there!");
};

multiply_numbers = fn(a, b, c) {
    return a * b * c;
};

print_hello();
print("The product is: " + to_string(multiply_numbers(2, 3, 4)));
```

Will output:
```
Hello there!
The product is: 24
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
