use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // no default implementation, required to be defined
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}
// same as above
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize())
// }

// pub fn dispatch_notify(item1: &(impl Summary + Display), item2: &impl Summary) {
// execute code
// }
// same as above, but both item1 and item2 are the same type
pub fn dispatch_notify<T: Summary + Display>(_item1: &T, _item2: &T) {
    // execute code
}

// pub fn some_function<T: Summary + Display, U: Clone + Debug>(item1: &T, item2: &U) -> i32 {
//     // execute code
//     42
// }

// same as above, but more readable
pub fn some_function<T, U>(_item1: &T, _item2: &U) -> i32
where
    T: Summary + Display,
    U: Clone + Debug,
{
    // execute code

    42
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rust_guy"),
        content: String::from("Rust is the future!"),
        reply: false,
        retweet: false,
    }
}

// Not allowed
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Over the horizon!"),
//             author: String::from("Some random dude"),
//             content: String::from("Some content, idk what lol"),
//         }
//     } else {
//         Tweet {
//             username: String::from("rust_guy"),
//             content: String::from("Rust is the future!"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("X is larger: {}", self.x)
        } else {
            println!("Y is larger: {}", self.y)
        }
    }
}

// ToString for every T with Display trait
// impl<T: Display> ToString for T {
    // code here
// }

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Traits are nice!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Look out!"),
        content: String::from("The Sun is out there!"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    notify(&article);
    println!("{}", returns_summarizable().summarize());
    let point = Pair::new(1, 4);
    point.cmp_display();
}
