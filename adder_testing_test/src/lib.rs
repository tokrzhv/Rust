use std::fmt::format;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//....................................
struct Guess {
    value: i32,
}
impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Must be greater than or equal to 1, got: {value}");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got: {value}");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_ca_hold_smaller() {
        let larger = Rectangle{
            width: 10,
            height: 20,
        };
        let smaller = Rectangle{
            width: 4,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(1));
        //assert_ne!(3, add_two(1));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains(" "),
            "Greeting doesn't contain name, value was '{}'",
            result
        )
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(900);
    }

}

pub fn greeting(name: &str) -> String {
    format!("Hi")
}
pub fn add_two(a: i32) -> i32 {
    a + 3
}



