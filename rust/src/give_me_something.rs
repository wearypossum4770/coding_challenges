pub fn give_me_something(input: &str) -> String {
    let mut target = String::from("something ");
    target.push_str(input);
    target
}
#[cfg(test)]
mod tests {
    use super::give_me_something;
    #[test]
    fn test_give_me_something() {
        assert_eq!(give_me_something("a"), "something a");
        assert_eq!(give_me_something("is cooking"), "something is cooking");
        assert_eq!(give_me_something(" is cooking"), "something  is cooking");
    }
}
