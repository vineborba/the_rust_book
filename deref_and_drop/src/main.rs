use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // same as
    // assert_eq!(5, *(y.deref()));

    // implicit deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str
    // same as
    // hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
