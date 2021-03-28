pub fn is_same_number(x: &<T>, y:&<T>) -> bool {
    x == y
}
#[cfg(test)]
mod tests {
    use super::is_same_number;
    #[test]
    fn test_is_same_number() {
assert_eq!(is_same_number(4, 8), false);
assert_eq!(is_same_number(2, 2), true);
assert_eq!(is_same_number(0, 6), false);
assert_eq!(is_same_number(2, "2"), false);
    }
}