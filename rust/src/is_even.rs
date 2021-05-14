pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}
#[cfg(test)]
mod tests {
    use super::is_even;
    #[test]
    fn test_is_even() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(10), true);
        assert_eq!(is_even(31), false);
        assert_eq!(is_even(666), true);
        assert_eq!(is_even(777), false);
        assert_eq!(is_even(3482034), true);
        assert_eq!(is_even(3482035), false);
    }
}
