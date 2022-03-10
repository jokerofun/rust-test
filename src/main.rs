fn main() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // testt
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_incorrect() {
        assert_ne!(add(1, 2), 4)
    }
}
