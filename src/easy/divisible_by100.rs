/// https://edabit.com/challenge/qMr6wYGr6NaXAPQGF
pub fn divisible_by100(param: i64) -> bool {
    param % 100 == 0
}
#[cfg(test)]
mod tests {
    use super::divisible_by100;
    #[test]
    fn test_divisible_by100() {
        assert_eq!(divisible_by100(1), false);
        assert_eq!(divisible_by100(100), true);
        assert_eq!(divisible_by100(1000), true);
        assert_eq!(divisible_by100(111000), true);
        assert_eq!(divisible_by100(-1), false,);
        assert_eq!(divisible_by100(0), true,);
        assert_eq!(divisible_by100(-100), true,);
    }
}
