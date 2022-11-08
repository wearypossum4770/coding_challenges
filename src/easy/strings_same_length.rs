/// https://edabit.com/challenge/yHGowWucg3k2kJdZ4
pub fn strings_same_length(param1: &str, param2: &str) -> bool {
    param1.len() == param2.len()
}

#[cfg(test)]
mod tests {
    use super::strings_same_length;
    #[test]
    fn test_strings_same_length() {
        assert_eq!(strings_same_length("AB", "CD"), true);
        assert_eq!(strings_same_length("ABC", "DE"), false);
        assert_eq!(strings_same_length("hello", "edabit"), false);
        assert_eq!(strings_same_length("meow", "woof"), true);
        assert_eq!(strings_same_length("jrnvjrnnt", "cvjknfjvmfvnfjn"), false);
        assert_eq!(strings_same_length("jkvnjrt", "krnf"), false);
        assert_eq!(strings_same_length("Facebook", "Snapchat"), true);
    }
}
