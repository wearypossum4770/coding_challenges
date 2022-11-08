/// https://edabit.com/challenge/rGsgEswWuW339yNxY
pub fn string_to_integer(param: &str) -> i32 {
    param.parse::<i32>().unwrap()
}
#[cfg(test)]
mod tests {
    use super::string_to_integer;
    #[test]
    fn test_string_to_integer() {
        assert_eq!(string_to_integer("6"), 6);
        assert_eq!(string_to_integer("2"), 2);
        assert_eq!(string_to_integer("10"), 10);
        assert_eq!(string_to_integer("667"), 667);
        assert_eq!(string_to_integer("12"), 12);
        assert_eq!(string_to_integer("1000"), 1_000);
    }
}
