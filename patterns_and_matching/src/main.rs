struct Point {
    x: i32,
    y: i32,
}
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColorAdvanced(Color),
    Hello { id: i32 },
}

fn main() {
    let var = Some(5);

    // matches are exaustive
    match var {
        Some(val) => println!("Matched value: {}", val),
        None => println!("No value!"),
    }

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // but `if let` isn't
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while let iterates until pop() returns None
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // enumerate returns a tuple, which can be destructured like this
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // destructuring tuples
    let (_x, _y, _z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
    // it is possible to match literals
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // catch all case, when no other case is matched
    }

    let x = Some(5);
    let y = 10;

    // match creates a new scope, so `y` is usable inside it
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 5;

    // matching ranges
    match x {
        1..=5 => println!("one through five"),
        // 1 | 2 | 3 | 4 | 5 => println!("one through five"), // same as above
        _ => println!("something else"),
    }

    let x = 'c';

    // also possible with chars
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    // destructuring structs
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // it is possible to change names when destructuring
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // and also match cases with properties
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // matching enums
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        _ => println!("Something different"),
    }

    let msg = Message::ChangeColorAdvanced(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColorAdvanced(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColorAdvanced(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let numbers = (2, 4, 8, 16, 32);

    // ignoring values with `_`, multiple values can be ignored
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    // ignoring properties when destructuring
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    // destructuring only first and last
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let num = Some(5);

    // match guards allow to further extend the use of match branches
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let msg = Message::Hello { id: 5 };

    // @ operator bind the value to a variable when testing it
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable), // value of id is binded to id_variable
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range") // value of id isn't known
        }
        Message::Hello { id } => println!("Found some other id: {}", id), // value is binded to id, but without matching any range
        _ => println!("Other messagens"),
    }
}

// destructuring tuples as function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
