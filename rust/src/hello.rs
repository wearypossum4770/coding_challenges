pub fn hello(name: &str) -> String {
    return format!("hello {} from edabit.com", name);
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello("John Doe"), "hello John Doe from edabit.com");
    }
}
