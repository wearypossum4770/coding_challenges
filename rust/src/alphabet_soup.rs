pub fn alphabet_soup(input: &str) -> String {
    let mut vec = Vec::new();
    for c in input.chars() {
        if c != ' ' {
            vec.push(c);
        }
    }
    vec.sort_unstable();
    let sorted: String = vec.into_iter().collect();
    sorted
}
#[cfg(test)]
mod tests {
    use super::alphabet_soup;
    #[test]
    fn test_alphabet_soup() {
        assert_eq!(alphabet_soup("hello"), "ehllo");
        assert_eq!(alphabet_soup("edabit"), "abdeit");
        assert_eq!(alphabet_soup("hacker"), "acehkr");
        assert_eq!(alphabet_soup("geek"), "eegk");
        assert_eq!(alphabet_soup("javascript"), "aacijprstv");
        assert_eq!(alphabet_soup("credibility"), "bcdeiiilrty");
        assert_eq!(alphabet_soup("apostrophe"), "aehoopprst");
        assert_eq!(alphabet_soup("determination"), "adeeiimnnortt");
        assert_eq!(alphabet_soup("win"), "inw");
        assert_eq!(alphabet_soup("synthesis"), "ehinsssty");
    }
}
