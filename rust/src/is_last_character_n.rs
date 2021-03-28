extern crate regex;
use regex::Regex;

pub fn is_last_character_n(word: &str) -> bool {
    let re = Regex::new(r"(?i)n$").unwrap();
    re.is_match(word)
}
#[cfg(test)]
mod tests {
    use super::is_last_character_n;
    #[test]
    fn test_is_last_character_n() {
        assert_eq!(is_last_character_n("AideN"), true);
        assert_eq!(is_last_character_n("Aiden"), true);
        assert_eq!(is_last_character_n("Roxy"), false);
        assert_eq!(is_last_character_n("Bert"), false);
        assert_eq!(is_last_character_n("Dean"), true);
        assert_eq!(is_last_character_n("Ian"), true);
        assert_eq!(is_last_character_n("Brian"), true);
        assert_eq!(is_last_character_n("Daniel"), false);
    }
}
