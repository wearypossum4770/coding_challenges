pub fn convert_age_to_days(age: i32) -> i32 {
    age * 365
}
#[cfg(test)]
mod tests {
    use super::convert_age_to_days;
    #[test]
    fn test_convert_age_to_days() {
        assert_eq!(convert_age_to_days(10), 3650);
        assert_eq!(convert_age_to_days(0), 0);
        assert_eq!(convert_age_to_days(73), 26645);
    }
}
