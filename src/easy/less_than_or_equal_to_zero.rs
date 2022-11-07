/// https://edabit.com/challenge/PTiLYyb4A69KZtBCg
pub fn less_than_or_equal_to_zero(value: i32) -> bool {
    value <= 0
}
#[cfg(test)]
mod tests {
    use super::less_than_or_equal_to_zero;
    #[test]
    fn test_less_than_or_equal_to_zero() {
        assert_eq!(less_than_or_equal_to_zero(5), false);
        assert_eq!(less_than_or_equal_to_zero(0), true);
        assert_eq!(less_than_or_equal_to_zero(-5), true);
    }
}
