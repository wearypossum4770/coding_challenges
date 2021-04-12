///
/// https://edabit.com/challenge/YT2kXSMEtACPPk35R
///
pub fn int_within_bounds(num: i32, lower: i32, upper: i32) -> bool {
    num >= lower && num < upper
}
#[cfg(test)]
mod tests {
    use super::int_within_bounds;
    #[test]
    fn test_int_within_bounds() {
        assert_eq!(int_within_bounds(3, 1, 9), true);
        assert_eq!(int_within_bounds(6, 1, 6), false);
        assert_eq!(int_within_bounds(5, 3, 8), true);
        assert_eq!(int_within_bounds(-5, -10, 6), true);
        assert_eq!(int_within_bounds(4, 0, 0), false);
        assert_eq!(int_within_bounds(10, 9, 11), true);
        assert_eq!(int_within_bounds(6, 2, 6), false);
        assert_eq!(int_within_bounds(6, 2, 10), true);
        assert_eq!(int_within_bounds(9, 2, 3), false);
        assert_eq!(int_within_bounds(9, 9, 9), false);
        assert_eq!(int_within_bounds(-3, -5, -2), true);
        assert_eq!(int_within_bounds(-3, -5, -3), false);
        assert_eq!(int_within_bounds(-3, -10, 10), true);
        assert_eq!(int_within_bounds(0, -3, 3), true);
        assert_eq!(int_within_bounds(0, 0, 1), true);
        assert_eq!(int_within_bounds(7, 7, 12), true);
    }
}
