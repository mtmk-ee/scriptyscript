
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
