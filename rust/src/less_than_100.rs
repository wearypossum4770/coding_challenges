// The function returns if the sum of two numbers is less than 100.
// #Examples
// ```
// let five_plus_fifty_seven = doccomments::less_than_100(5,57);
// assert_eq!(five_plus_fifty_seven, true);
// ```

//
pub fn less_than_100(a: i32, b: i32) -> bool {
    a + b < 100
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
