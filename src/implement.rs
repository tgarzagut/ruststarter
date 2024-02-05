pub fn add(a:i32, b:i32) -> i32 {
    a + b
}

pub fn subtract(a:i32, b:i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(2,3), 5);
    }

    #[test]
    fn test_subtract(){
        assert_eq!(subtract(3,2), 1);
    }

}
//cargo test 2>&1 | grep "test result" | awk '{print $4, "tests passed, Score:", $4*25 "/100"}'
//cargo test
