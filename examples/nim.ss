// A simple turn-based game, where the user plays agains the computer.
//
// There is a strategy for this game where the player who goes second
// always win. Since the computer makes use of this strategy, the player
// can never win.
//
// Rules:
// - There are initially 12 tokens in the pile
// - Players take turns taking 1, 2, or 3 tokens from the pile
// - Whoever takes the last token wins


// Read a single integer from the user, or exit if the user does not enter anything.
// While continue reading until valid input is provided.
//
// Returns:
//     The integer that the user entered.
read_int_or_exit = fn() {
    loop {
        str = input("Enter number of tokens to take: ");
        if str == "" {
            exit();
        }
        result = int(str);
        if result == nil {
            print("   Hey, that's not a number!");
            continue;
        }
        return int(str);
    }
};

// Ask the user for some number of tokens to take.
//
// Args:
//     tokens: The number of tokens left in the pile
//
// Returns:
//     The number of tokens left in the pile after the user takes some.
user_turn = fn(tokens) {
    print("========== Your Turn ==========");
    print("There are " + string(tokens) + " tokens left. You may pick 1 to 3 tokens.");
    loop {
        take = read_int_or_exit();
        if take < 1 or take > 3 {
            print("    You can only take 1, 2, or 3 tokens.");
            continue;
        }
        break;
    }
    print("    You take " + string(take) + " tokens.");
    return tokens - take;
};

// Run the computer turn.
//
// Args:
//     tokens: The number of tokens left in the pile
//
// Returns:
//     The number of tokens left in the pile after the computer takes some.
computer_turn = fn(tokens) {
    print("========== Computer's Turn ==========");
    take = tokens % 4;
    print("Computer takes " + string(take) + " tokens.");
    print("There are " + string(tokens - take) + " tokens left.");
    return tokens - take;
};


// ============================= Game Loop =============================
print("Welcome to the game of Nim!");
print("On each turn, the player or computer may take 1, 2, or 3 tokens from the pile");
print("Whoever who takes the last token wins.");

tokens = 12;
while tokens > 0 {
    tokens = user_turn(tokens);
    if tokens == 0 {
        print("You win!");
        break;
    }
    tokens = computer_turn(tokens);
    if tokens == 0 {
        print("You lose!");
        break;
    }
}
