use std::slice;

// unsafe functions need to be called with the right params or undefined behaviour will occur
unsafe fn danger() {
    // this block is fully unsafe
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// static variables have always the same memory address
// and need to have a static lifetime
static HELLO_WORD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    // counter can only be modified inside unsafe blocks
    unsafe {
        COUNTER += inc;
    }
}

// traits are unsafe whenever there is an unsafe method for it
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    let mut num = 5;

    // raw pointers
    // raw pointers allow to bypass borrow rules
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // raw pointers can only be dereferenced inside unsafe blocks
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);

        // unsafe funcntions too
        danger();

        // and also functions defined in extern
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("message is: {}", HELLO_WORD);

    add_to_counter(3);

    unsafe {
        // counter can only be read inside unsafe blocks
        println!("Counter: {}", COUNTER);
    }
}

// safe abstraction of unsafe operation
fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // slice::from_raw_parts_mut and ptr.add are unsafe
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("This function is called from C code!");
}
