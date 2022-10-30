use macros::myvec;

fn main() {
    let a = myvec!(1,2,3);

    println!("my vec macro: {:?}", a);
}
