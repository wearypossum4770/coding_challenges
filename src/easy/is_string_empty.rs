/// https://edabit.com/challenge/EzbfiquDoAc2Zc9FL
pub fn is_string_empty(val: &str) -> bool {
    val.len() < 1
}
#[cfg(test)]
mod tests {
    use super::is_string_empty;
    #[test]
    fn test_is_string_empty() {
        assert_eq!(is_string_empty(""), true);
        assert_eq!(is_string_empty("a"), false);
        assert_eq!(is_string_empty(" "), false);
        assert_eq!(is_string_empty("            "), false);
        assert_eq!(is_string_empty("38215"), false);
        assert_eq!(is_string_empty("afjabsdf"), false);
        assert_eq!(is_string_empty("!?@&"), false);
    }
}
