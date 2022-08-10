use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // fixed size array
    let _a = [1, 2, 3];

    // variable size -> stored in the heap
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // v2 isn't defined
    {
        // macros are awesome
        let mut v2 = vec![1, 2, 3, 4, 5];
        // v2 is real

        let third = &v2[2];
        // v2.push(6); // error -> borrowing a mutable ref having an immutable ref (third)
        println!("third element: {}", third);

        match v.get(20) {
            Some(index) => println!("index element: {}", index),
            None => println!("there is no element here."),
        }

        // looping and mutating v2 values
        for i in &mut v2 {
            print!("{} ", i);
            *i += 50;
            println!("{}", i);
            v.push(*i);
        }
    } // v2 an its values are dropped

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("i {}", i),
        SpreadsheetCell::Float(f) => println!("f {}", f),
        SpreadsheetCell::Text(f) => println!("t {}", f),
    }

    // String are stored as a collection of UFT-8 encoded bytes
    let mut s1 = String::new();
    let s2 = "initial contents";
    let _s3 = s2.to_string();
    let s4 = String::from("नमस्ते");
    let s5 = String::from("😎");

    s1.push_str("bar");
    s1.push('!');
    println!("{}", s1);

    // concatenating
    // let s6: String = s1 + &s5;
    // println!("{}", s6);
    // println!("{}", s1); // error -> value has been moved to s6

    // concatenating with macros
    let s6 = format!("{}{}", s1, s5);
    println!("{}", s6);
    println!("{}", s1); // no error -> format doesn't take ownership

    // indexing
    // let c: char = s4[0];  // error -> strings can't be indexed this way

    // s4 is represented as

    // Bytes -> u8 values
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]

    for b in s4.bytes() {
        println!("b {}", b);
    }

    // Scallar values -> char values, whole letter or part of it
    // ['न', 'म', 'स', '्', 'त', 'े']

    for c in s4.chars() {
        println!("c {}", c);
    }

    // Grapheme clusters -> the whole letter
    // ["न", "म", "स्", "ते"]

    for g in s4.graphemes(true) {
        println!("g {}", g);
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    // <key, value> inferred
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // println!("{}", blue) // error -> value was mooved
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(val) => println!("Retrieved value: {}", val),
        None => println!("No value"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Orange"), 3);
    scores.insert(String::from("Orange"), 20); // overrides original value
    println!("{:?}", scores);

    scores.entry(String::from("Green")).or_insert(90);
    scores.entry(String::from("Green")).or_insert(10); // no overrides
    println!("{:?}", scores);

    let text = "hello world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
