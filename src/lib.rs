mod javascript_parser;
mod java_parser;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let result = add_one(5);
        assert_eq!(result, 6);
    }
}
