use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sushi;

fn main() {
    Sushi::hello_macro();
}