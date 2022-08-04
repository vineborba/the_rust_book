fn main() {
    /*
    !                           Ownership rules
            1. Each value in Rust has a variable that's called its owner.
            2. There can only be one owner at a time.
            3. When the owner goes out of scope, the value will be dropped.
     */

    {
        // x and y are not valid declared;
        let _x = "fixed size, stored in the binary";
        let _y = String::from("variable size, stored in the heap");
        // x and y are valid here
    } // scope is over, x and y are no longer valid

    let _x = 5;
    let _y = _x; // Copy

    let s1 = String::from("hey there");

    // let s2 = s1;
    // println!("{}, world", s1); // Compile time error -- s1 has been moved

    let s2 = s1.clone();
    println!("{}, world", s1); // Cloned value, s1 is usable
    takes_ownership(s2);
    // Compile time error
    // s2 has been moved, when takes_ownership goes out of scope
    // the value gets dropped
    // println!("{}", s2);

    let x = 5;
    makes_copy(x);
    print!("{}", x); // x  was copied, so it can be used again

    let s3 = gives_ownership();
    print!("s3 = {}", s3); // x  was copied, so it can be used again

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    print!("s3 = {} s5 = {}", s3, s5); // x  was copied, so it can be used again

    /*
    !                       Rules of References
            1. At any given time, you can have either one mutable reference
            or any number of immutable references.

            2. References must always be valid.
    */

    let len = calculate_length(&s3);
    println!("The length of '{}' is {}", s3, len);

    let mut s6 = String::from("this is a ");
    change(&mut s6);

    let mut s7 = String::from("hello");
    let _r1 = &mut s7;
    // let r2 = &mut s7; // only 1 mutable reference
    // let r3 = &s7; // can't mix mutable and immutable at the same time
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello world");
    let other_s = "hello world";
    let _hello = &s[..5];
    let _world = &s[6..];
    let _complete = &[..];

    let word = first_word(&s);
    let _other_word = first_word(&other_s); // also works!
    // s.clear(); // error -> s is borrowed as a immutable reference
    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4];
    let _slice = &a[1..2];
}

fn takes_ownership(random_string: String) {
    println!("{}", random_string);
}

fn makes_copy(random_integer: i32) {
    println!("{}", random_integer);
}

fn gives_ownership() -> String {
    let new_string = String::from("hi");

    new_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(a_string: &mut String) {
    a_string.push_str("mutable reference");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
