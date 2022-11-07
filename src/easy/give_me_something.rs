pub fn give_me_something(param: &str) -> String {
    "something ".to_owned() + param
}

#[cfg(test)]
mod tests {
    use super::give_me_something;
    #[test]
    fn test_give_me_something() {
        assert_eq!(
            give_me_something("is better than nothing"),
            "something is better than nothing".to_string()
        );
        assert_eq!(
            give_me_something("is better than nothing"),
            "something is better than nothing".to_string()
        );
        assert_eq!(
            give_me_something("Bob Jane"),
            "something Bob Jane".to_string()
        );
        assert_eq!(
            give_me_something("something"),
            "something something".to_string()
        );
        assert_eq!(give_me_something("a"), "something a".to_string());
        assert_eq!(
            give_me_something("is cooking"),
            "something is cooking".to_string()
        );
        assert_eq!(
            give_me_something(" is cooking"),
            "something  is cooking".to_string()
        );
    }
}
