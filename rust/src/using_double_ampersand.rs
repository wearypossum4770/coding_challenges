pub fn using_double_ampersand(a: bool, b: bool) -> bool {
    a && b
}

#[cfg(test)]
mod tests {
    use super::using_double_ampersand;
    #[test]
    fn test_using_double_ampersand() {
        assert_eq!(using_double_ampersand(true, true), true);
        assert_eq!(using_double_ampersand(true, false), false);
        assert_eq!(using_double_ampersand(false, true), false);
        assert_eq!(using_double_ampersand(false, false), false);
    }
}
