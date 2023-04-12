// Print either "Fizz", "Buzz", "FizzBuzz", or the number itself
// depending on the factors of the number.
fizzbuzz = fn(n) {
    fizz = n % 3 == 0;
    buzz = n % 5 == 0;
    if fizz and buzz {
        print("FizzBuzz");
    } else if fizz {
        print("Fizz");
    } else if buzz {
        print("Buzz");
    } else {
        print(n);
    }
};

// Read a single integer from the user, or exit if the user does not enter anything.
// While continue reading until valid input is provided.
read_int_or_exit = fn() {
    loop {
        str = input("Enter a number: ");
        if str == "" {
            exit();
        }
        result = int(str);
        if result == nil {
            print("Hey, that's not a number!");
            continue;
        }
        return int(str);
    }
};

// Continually plays FizzBuzz with the user.
cli = fn() {
    loop {
        fizzbuzz(read_int_or_exit());
    }
};

cli();
