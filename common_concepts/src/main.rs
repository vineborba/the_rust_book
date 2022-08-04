fn main() {
    // ! VARIABLES

    // ? Mutability
    // let mut x = 5;

    // ? shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    // ? Constant values
    const CONSTANT_COUNTER: u32 = 100_000;
    println!("Constant value is: {}", CONSTANT_COUNTER);

    // ! DATA TYPES

    // ? Integers
    let decimal = 98_222;
    println!("Decimal value, {}", decimal);
    let hex = 0xff;
    println!("Hex value, {}", hex);
    let octal = 0o77;
    println!("Octal value, {}", octal);
    let binary = 0b1111_0000;
    println!("Binary value, {}", binary);
    let byte = b'A';
    println!("Byte value, {}", byte);

    let example: u8 = 255;
    println!("Max unsigned 8 bits integer: {}", example);
    // let example: u8 = 256 -> error at development time
    // overflow -> resets to minimum in production build, panic in debug build
    // example += 1;
    // println!("Overflow: {}", example);

    // ? Floating-point numbers
    let float = 2.0;
    println!("Float value, {}", float);
    let float: f32 = 3.0;
    println!("Float value, {}", float);

    // * operations
    let sum = 5 + 10;
    println!("Sum value, {}", sum);
    let diff = 56.4 - 9.7;
    println!("Diff value, {}", diff);
    let product = 56 * 12;
    println!("Product value, {}", product);
    let division = 75.1 / 95.3;
    println!("Division value, {}", division);
    let remainder = 7 % 4;
    println!("Remainder value, {}", remainder);

    // ? Booleans
    let t = true;
    print!("This is {}", t);

    let f = false;
    println!("...and this is {}", f);

    // ? Characters
    let character = 'V';
    let emoji = '😁';
    println!("{} says: {}", character, emoji);

    // ! Commpound types

    // ? Tuples
    let tup = ("Hours of sleep", 8.0);
    // * destructuring
    let (_fuel, _value) = tup;
    // * dot notation
    let _value = tup.1;

    // ? Arrays
    let response_codes = [200, 404, 500];
    let _ok = response_codes[0];
    // let out_of_bounds = response_codes[4]; -> error in development time

    let _byte = [0; 8];

    // ! Functions
    // ? Function with returned value
    let sum = other_function();
    println!("Returned sum: {}", sum);

    // ? Function with parameter
    second_function("my friend");

    // ! Control flow

    // ? If
    let number = 5;
    if number < 10 {
        println!("First conditon true");
    } else if number < 20 {
        println!("Second conditon true");
    } else {
        println!("All conditons false");
    }

    let cond = true;
    let number = if cond { 4 } else { 3 };
    println!("Number is now: {}", number);

    // ? Loop
    let mut counter = 0;
    let result = loop {
        println!("Eat, sleep, code, repeat");
        counter += 1;
        if counter == 10 {
            // returns value after break
            break counter;
        }
    };
    println!("Looped {} times", result);

    // ? While
    let mut number = 4;
    while number != 0 {
        println!("Countdown: {}!", number);
        number -= 1;
    }
    println!("LIFTOFF");

    // ? For (in) loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Current for value: {}", element);
    }

    for number in 1..5 {
        println!("Number: {}", number);
    }
}

fn other_function() -> i32 {
    println!("Other function");
    // classic return
    // return 1 + 2;
    // return expression value
    1 + 2
}

fn second_function(name: &str) {
    println!("Hello, {}", name);
}
