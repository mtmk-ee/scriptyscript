

print_hello = fn() {
    print("Hello there!");
};

multiply_numbers = fn(a, b, c) {
    return a * b * c;
};

print_hello();
print("The product is: " + to_string(multiply_numbers(2, 3, 4)));
