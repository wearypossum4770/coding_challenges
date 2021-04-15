///
/// https://app.codesignal.com/arcade/intro/level-1/s5PbmwxfECC52PWyQ
///
pub fn check_palindrome(word: &str) -> bool {
    let num_string = word.to_string();
    let half = num_string.len() / 2;
    num_string
        .bytes()
        .take(half)
        .eq(num_string.bytes().rev().take(half))
}
#[cfg(test)]
mod tests {
    use super::check_palindrome;
    #[test]
    fn test_check_palindrome() {
        assert_eq!(check_palindrome("aabaa"), true);
        assert_eq!(check_palindrome("abac"), false);
        assert_eq!(check_palindrome("a"), true);
        assert_eq!(check_palindrome("az"), false);
        assert_eq!(check_palindrome("abacaba"), true);
        assert_eq!(check_palindrome("z"), true);
        assert_eq!(check_palindrome("aaabaaaa"), false);
        assert_eq!(check_palindrome("zzzazzazz"), false);
        assert_eq!(check_palindrome("hlbeeykoqqqqokyeeblh"), true);
        assert_eq!(check_palindrome("hlbeeykoqqqokyeeblh"), true);
    }
}
