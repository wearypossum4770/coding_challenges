///
/// https://edabit.com/challenge/Yj2QWQG7oHREM6DRo
pub fn nth_even(num: i32) -> i32 {
    2 * num - 2
}
#[cfg(test)]
mod tests {
    use super::nth_even;
    #[test]
    fn test_nth_even() {
        assert_eq!(nth_even(1), 0);
        assert_eq!(nth_even(2), 2);
        assert_eq!(nth_even(3), 4);
        assert_eq!(nth_even(100), 198);
        assert_eq!(nth_even(1298734), 2597466);
    }
}
