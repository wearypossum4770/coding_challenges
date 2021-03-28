pub fn get_first_value(arr: Vec<i64>) -> i64 {
    arr[0]
}
#[cfg(test)]
mod tests {
    use super::get_first_value;
    #[test]
    fn test_get_first_value() {
        assert_eq!(get_first_value(vec![1, 2, 3]), 1);
        assert_eq!(get_first_value(vec![80, 5, 100]), 80);
        assert_eq!(get_first_value(vec![-500, 0, 50]), -500);
        assert_eq!(get_first_value(vec![5, 2, 3]), 5);
        assert_eq!(get_first_value(vec![75675, 5, 100]), 75675);
        assert_eq!(get_first_value(vec![-52320, 0, 50]), -52320);
    }
}
