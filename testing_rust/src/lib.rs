pub fn print_and_returns_10(value: i32) -> i32 {
    println!("I got value {}", value);
    10
}

pub fn add_two(value: i32) -> i32 {
    internal_adder(value, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passing_test() {
        let value = print_and_returns_10(5);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore = "just an example"]
    fn failing_test() {
        let value = print_and_returns_10(7);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore = "just an example"]
    fn expensive_test() {
        // code that takes too much time to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
