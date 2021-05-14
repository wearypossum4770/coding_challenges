///
/// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
pub fn check_if_pangram(word: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut is_pangram = true;
    for alpha in alphabet.chars() {
        if !word.contains(alpha) {
            is_pangram = false;
        }
    }
    is_pangram
}
#[cfg(test)]
mod tests {
    use super::check_if_pangram;
    #[test]
    fn test_check_if_pangram() {
        assert_eq!(
            check_if_pangram("thequickbrownfoxjumpsoverthelazydog"),
            true
        );
        assert_eq!(check_if_pangram("leetcode"), false);
    }
}
