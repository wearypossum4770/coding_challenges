#![allow(unused)]
pub fn remove_first_char(input: &str) -> String {
    let string_input = String::from(input);
    let string_slice = &string_input[1..];
    string_slice.to_string()
}
#[cfg(test)]
mod tests {
    use super::remove_first_char;
    #[test]
    fn test_remove_first_char() {
        assert_eq!(remove_first_char("pokhara"), "okhara");
        assert_eq!(remove_first_char("biratnagar"), "iratnagar");
        assert_eq!(remove_first_char("nepal"), "epal");
        assert_eq!(remove_first_char("apple"), "pple");
        assert_eq!(remove_first_char("cherry"), "herry");
        assert_eq!(remove_first_char("plum"), "lum");
        assert_eq!(remove_first_char("damak"), "amak");
        assert_eq!(remove_first_char("itahari"), "tahari");
        assert_eq!(remove_first_char("rasuwa"), "asuwa");
        assert_eq!(remove_first_char("rolpa"), "olpa");
    }
}
