/// https://edabit.com/challenge/2tcuBxn37oouMeErN
pub fn string_length_odd_or_even(word: &str) -> bool {
    word.len() % 2 == 0
}
#[cfg(test)]
mod tests {
    use super::string_length_odd_or_even;
    #[test]
    fn test_string_length_odd_or_even() {
        assert_eq!(string_length_odd_or_even("apples"), true);
        assert_eq!(string_length_odd_or_even("banana"), true);
        assert_eq!(string_length_odd_or_even("cherry"), true);
        assert_eq!(string_length_odd_or_even("mango"), false);
        assert_eq!(string_length_odd_or_even("peach"), false);
        assert_eq!(string_length_odd_or_even("pears"), false);
    }
}
