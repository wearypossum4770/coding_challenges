pub fn shift_to_right(num1: i32, num2: i32) -> i32 {
    num1 >> num2
}
#[cfg(test)]
mod tests {
    use super::shift_to_right;
    #[test]
    fn test_shift_to_right() {
        assert_eq!(shift_to_right(80, 3), 10);
        assert_eq!(shift_to_right(-24, 2), -6);
        assert_eq!(shift_to_right(-5, 1), -3);
        assert_eq!(shift_to_right(38, 0), 38);
        assert_eq!(shift_to_right(192, 4), 12);
        assert_eq!(shift_to_right(1024, 5), 32);
        assert_eq!(shift_to_right(4666, 6), 72);
        assert_eq!(shift_to_right(3777, 6), 59);
        assert_eq!(shift_to_right(-512, 10), -1);
    }
}
