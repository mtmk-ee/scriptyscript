
// For loop
for (i = 0; i < 3; i = i + 1) {
    print(i);
}

// While loop
i = 0;
while i != 3 {
    print(i);
    i = i + 1;
}

// Infinite loop
i = 0;
loop {
    print(i);
    i = i + 1;
    if i == 3 {
        break;
    }
}


// Returning from within a loop
early_exit = fn(max, exit_at) {
    for (i = 0; i < max; i = i + 1) {
        print(i);
        if i == exit_at {
            print("exiting early at " + to_string(i));
            return;
        }
    }
};
early_exit(10, 3);
