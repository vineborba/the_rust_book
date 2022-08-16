#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 8,
        };
        let smaller = Rectangle {
            height: 5,
            width: 4,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 8,
        };
        let smaller = Rectangle {
            height: 5,
            width: 4,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4)
    }

    #[test]
    fn it_does_not_add_three() {
        assert_ne!(add_two(2), 5)
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Vini");
        assert!(
            result.contains("Vini"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 is not 4"))
        }
    }
}
