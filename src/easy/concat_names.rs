/// https://edabit.com/challenge/RQwdZmtrW8mCnuCMN
pub fn concat_names(first: &str, last: &str) -> String {
    String::from(last) + ", " + first
}
#[cfg(test)]
mod tests {
    use super::concat_names;
    #[test]
    fn test_concat_names() {
        assert_eq!(concat_names("John", "Doe"), "Doe, John".to_string());
        assert_eq!(concat_names("First", "Last"), "Last, First".to_string());
        assert_eq!(concat_names("A", "B"), "B, A".to_string());

        // In case someone is making odd assumptions about comma characters.
        assert_eq!(concat_names(",", ","), ",, ,".to_string());
    }
}
