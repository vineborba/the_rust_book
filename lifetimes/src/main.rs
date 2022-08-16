struct ImportantExcertpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcertpt<'a> {
    // Rule 3
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let r;

    {
        let _x = 5;
        // x will be dropped, r can't borrow the reference
        // r = &x;
    }
    // can borrow
    let x = 5;
    r = &x;

    println!("r: {}", r);

    let str1 = String::from("abcd");
    let str2 = String::from("xyz");

    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is: {}", result);

    let str3 = String::from("efgh");
    let result2;
    {
        let str4 = String::from("uvw");

        // result2 and 3 lifetime is the same as str4
        // result2 = longest(str3.as_str(), str4.as_str()); // not allowed -> str4 is dropped before it is used
        let result3 = longest(str3.as_str(), str4.as_str());
        println!("The longest string is: {}", result3);

        // can be used outside, only str3 lifetime is important here
        result2 = longest_v2(str3.as_str(), str4.as_str())
    }
    // result2 is valid here
    println!("The longest string is: {}", result2);

    let string = String::from("Call me Kevin. The 3mil subscriber youtuber...");
    let first_sentence = string.split(".").next().expect("Could not find sentence");
    // i lifetime is bound to firs_sentence lifetime
    let i = ImportantExcertpt {
        part: first_sentence,
    };
    println!("first word is: {}", first_word(first_sentence));
    i.return_part("Hello!");

    let static_lifetime: &'static str = "I have a static lifetime.";
}

// Returns a reference with the smallest lifetime of parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_v2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// fn longest_v3<'a>(_x: &str, _y: &str) -> &'a str {
//     let result = String::from("Wow, this is a really long string, isn't it?");
//     this is illegal, you can only return references of parameters outside the function
//     scope, as local variables are dropped at the end of the function
//     result.as_str()
// }

fn longest_v4<'a>(_x: &str, _y: &str) -> String {
    let result = String::from("Wow, this is a really long string, isn't it?");
    // allowed, because the ownership is taken
    result
}

// Lifetime annotations aren't necessary here, following the rules:
// 1. Each parameter that is a referencence get its own lifetime parameter

// 2. If there is exactly one input lifetime parameter,
//    that lifetime if assigned to all output lifetime parameters

// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
//    the lifetime of self is assigned to all output lifetime parameters
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
