///
/// https://edabit.com/challenge/RMZiERz2cbjmbXruY
///
pub fn triangular_numbers(num: i32) -> i32 {
    (num * (num + 1)) / 2
}
#[cfg(test)]
mod tests {
    use super::triangular_numbers;
    #[test]
    fn test_triangular_numbers() {
        assert_eq!(triangular_numbers(1), 1);
        assert_eq!(triangular_numbers(2), 3);
        assert_eq!(triangular_numbers(3), 6);
        assert_eq!(triangular_numbers(8), 36);
        assert_eq!(triangular_numbers(2153), 2318781);
    }
}
