
test_code = fn(code) {
    print(code, "=", string(exec(code + ";")));
};

print("---------- Conversions ----------");
test_code("string(5)");
test_code("string(5.5) + \"1\"");
test_code("int(\"6\") + 5");
test_code("int(6.5)");


print("---------- Max/Min ----------");
test_code("max(5, 10)");
test_code("max(5, 15, 10)");
test_code("min(5, 10)");

print("---------- Misc Numeric ----------");
test_code("round(6.5)");
test_code("abs(5)");
test_code("abs(5)");

print("---------- Wrapping it Up ----------");
print("Exiting...");
exit(0);
print("This will never run!");
