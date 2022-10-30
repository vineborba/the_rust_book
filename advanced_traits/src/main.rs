use std::fmt;
use std::ops::Add;

struct Counter {}

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// associate type allow only one implementation
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// generics allow multiple implementations
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// operator overload
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);

    fn land();
}

trait Wizzard {
    fn fly(&self);

    fn land();
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }

    fn land() {
        println!("*laying down*")
    }
}

impl Wizzard for Human {
    fn fly(&self) {
        println!("Up!")
    }

    fn land() {
        println!("Down!")
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }

    fn land() {
        println!("We about to land.")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 2, y: 2 },
        Point { x: 3, y: 3 }
    );

    let human = Human;
    // human implementation for fly will be called
    human.fly();
    // calling other implementations
    Pilot::fly(&human);
    Wizzard::fly(&human);

    // associated functions doesn't use self as a parameter
    Human::land();
    <Human as Wizzard>::land();
    <Human as Pilot>::land();
}
