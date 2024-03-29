/// https://edabit.com/challenge/iBQYbSHZGhpktLRdn
pub fn divisible_by_five(param1: i64) -> bool {
    param1 % 5 == 0
}
#[cfg(test)]
mod tests {
    use super::divisible_by_five;
    #[test]
    fn test_divisible_by_five() {
        assert_eq!(divisible_by_five(7), false);
        assert_eq!(divisible_by_five(5), true);
        assert_eq!(divisible_by_five(15), true);
        assert_eq!(divisible_by_five(33), false);
        assert_eq!(divisible_by_five(-55), true);
        assert_eq!(divisible_by_five(37), false);
        assert_eq!(divisible_by_five(-18), false);
        assert_eq!(divisible_by_five(999), false);
        assert_eq!(divisible_by_five(2), false);
    }
}
