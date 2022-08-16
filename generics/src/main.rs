#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct Square<T> {
    side: T,
    area: T,
}

impl<T> Square<T> {
    fn side(&self) -> &T {
        &self.side
    }
}

impl Square<f64> {
    fn area(&self) -> f64 {
        self.area
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 1];

    let largest = find_largest_generic(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['h', 'i', 't', 'h', 'e', 'r', 'e'];

    let largest = find_largest_generic(char_list);

    println!("The largest char is {}", largest);

    let sqr = Square {
        side: 5,
        area: 5 * 5,
    };
    println!("sqr.side(): {}", sqr.side());
    println!("sqr.area(): not available");

    let other_sqr = Square {
        side: 12.5,
        area: 12.5 * 12.5,
    };
    println!("other_sqr.side(): {}", other_sqr.side());
    println!("other_sqr.area(): {}", other_sqr.area());

    let p1 = Point { x: 1, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = p1.mixup(p2);

    println!("p3: {:?}", p3);
}

fn find_largest_generic<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest_char(char_list: Vec<char>) -> char {
    let mut largest = char_list[0];
    for c in char_list {
        if c > largest {
            largest = c;
        }
    }
    largest
}
