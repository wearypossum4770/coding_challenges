pub fn return_negative(num: i32) -> i32 {
    if num <= 0 {
        num
    } else {
        -num
    }
}
#[cfg(test)]
mod tests {
    use super::return_negative;
    #[test]
    fn test_return_negative() {
        assert_eq!(return_negative(4), -4);
        assert_eq!(return_negative(15), -15);
        assert_eq!(return_negative(-4), -4);
        assert_eq!(return_negative(42), -42);
        assert_eq!(return_negative(-9), -9);
        assert_eq!(return_negative(0), 0);
        assert_eq!(return_negative(1), -1);
        assert_eq!(return_negative(-1), -1);
    }
}
