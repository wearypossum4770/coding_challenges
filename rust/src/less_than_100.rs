pub fn less_than_100(num1: i32, num2: i32) -> bool {
    num1 + num2 < 100
}
#[cfg(test)]
mod tests {
    use super::less_than_100;
    #[test]
    fn test_less_than_100() {
        assert_eq!(less_than_100(5, 57), true);
        assert_eq!(less_than_100(77, 30), false);
        assert_eq!(less_than_100(0, 59), true);
        assert_eq!(less_than_100(78, 35), false);
        assert_eq!(less_than_100(63, 11), true);
        assert_eq!(less_than_100(37, 99), false);
        assert_eq!(less_than_100(52, 11), true);
        assert_eq!(less_than_100(82, 95), false);
        assert_eq!(less_than_100(17, 44), true);
        assert_eq!(less_than_100(74, 53), false);
        assert_eq!(less_than_100(3, 77), true);
        assert_eq!(less_than_100(25, 80), false);
        assert_eq!(less_than_100(59, 28), true);
        assert_eq!(less_than_100(69, 87), false);
        assert_eq!(less_than_100(10, 45), true);
        assert_eq!(less_than_100(43, 58), false);
        assert_eq!(less_than_100(50, 44), true);
        assert_eq!(less_than_100(74, 89), false);
        assert_eq!(less_than_100(3, 27), true);
        assert_eq!(less_than_100(21, 79), false);
    }
}
