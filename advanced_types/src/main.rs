use std::fmt;

// wrapper to implement traits for types outside the crate
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Same data structure, different types
struct Age(u32);
struct ID(u32);

// type alias
type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    assert_eq!(x + y, 10);

    let f: Thunk = returns_long_type();
    takes_long_type(f);
}

fn takes_long_type(param: Thunk) {
    // do stuff here
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hello"))
}
