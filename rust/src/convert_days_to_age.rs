pub fn convert_days_to_age(age: i32) -> i64 {
    let total = age * 365;
    total.into()
}
#[cfg(test)]
mod tests {
    use super::convert_days_to_age;
    #[test]
    fn test_convert_days_to_age() {
        assert_eq!(convert_days_to_age(10), 3650);
        assert_eq!(convert_days_to_age(0), 0);
        assert_eq!(convert_days_to_age(73), 26645);
    }
}
