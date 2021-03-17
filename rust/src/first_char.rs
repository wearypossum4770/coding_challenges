pub fn first_char(input: &str) -> String {
    let my_vec: Vec<char> = input.chars().collect();
    my_vec[0].into()
}
#[cfg(test)]
mod tests {
    use super::first_char;
    #[test]
    fn test_first_char() {
        assert_eq!(first_char("pokhara"), "p");
        assert_eq!(first_char("biratnagar"), "b");
        assert_eq!(first_char("nepal"), "n");
        assert_eq!(first_char("apple"), "a");
        assert_eq!(first_char("cherry"), "c");
        assert_eq!(first_char("plum"), "p");
        assert_eq!(first_char("damak"), "d");
        assert_eq!(first_char("itahari"), "i");
        assert_eq!(first_char("rasuwa"), "r");
        assert_eq!(first_char("rolpa"), "r");
    }
}
