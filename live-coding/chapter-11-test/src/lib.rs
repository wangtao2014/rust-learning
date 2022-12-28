pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let greeting = greeting("YYY");
        assert!(
            greeting.contains("YYY"),
            "Greeting didn't contain name '{}'",
            greeting
        );
    }

    #[test]
    fn test_return_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }
}
