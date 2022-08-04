struct User {
    username: String,
    email: String,
    active: bool,
}

// Same values types, but different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("some.email@gmail.com"),
        username: String::from("User One"),
        active: true,
    };

    let _name = user1.username;
    user1.username = String::from("User 1");

    let user2 = create_user(
        String::from("second.email2gmail.com"),
        String::from("User Two"),
    );

    let _user3 = User {
        email: String::from("hello@gmail.com"),
        username: String::from("User Three"),
        ..user2
    };

    let rect = Rectangle {
        width: 32,
        height: 25,
    };
    println!("The area is: {}", rect.area());
    println!("rect: {:?}", rect);
    println!("pretty rect: {:#?}", rect);

    let small = Rectangle {
        width: 27,
        height: 24,
    };

    let big = Rectangle {
        width: 40,
        height: 85,
    };

    println!("rect can hold small: {}", rect.can_hold(&small));
    println!("rect can hold big: {}", rect.can_hold(&big));

    let _square = Rectangle::square(40);
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
    }
}
