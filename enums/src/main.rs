#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeCollor(i32, i32, i32),
}

impl Message {
    fn some_func() {
        println!("Enums have functions!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("Some string"));
    route(localhost);
    route(six);
    Message::some_func();

    let some_number = Some(5);
    let _some_string = Some("string");

    let a_number = 4;
    let _absent_number: Option<i32> = None;
    // let sum = a_number + absent_number; // Compile time error
    let _sum = a_number + some_number.unwrap_or(0);

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Arizona));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // equivalents
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three!"),
        _ => (),
    }
    if let Some(3) = some_value {
        println!("three!");
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // default match
    }
}
