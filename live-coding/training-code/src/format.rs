pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// cargo test -- format --nocapture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let sum = add(1, 2);
        assert_eq!(sum, 3);
    }
}