/// https://edabit.com/challenge/DcmB9Ycm58FdkPe7k
pub fn should_serve_drinks(age: i32, on_break: bool) -> bool {
    if !on_break {
        age >= 18
    } else {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::should_serve_drinks;
    #[test]
    fn test_should_serve_drinks() {
        assert_eq!(should_serve_drinks(17, true), false);
        assert_eq!(should_serve_drinks(19, false), true);
        assert_eq!(should_serve_drinks(30, true), false);
        assert_eq!(should_serve_drinks(24, false), true);
        assert_eq!(should_serve_drinks(18, false), true);
        assert_eq!(should_serve_drinks(3, false), false);
    }
}
