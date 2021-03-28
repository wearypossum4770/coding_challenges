pub fn find_perimeter(length: i64, width: i64) -> i64 {
    2 * length + 2 * width
}

#[cfg(test)]
mod tests {
    use super::find_perimeter;
    #[test]
    fn test_find_perimeter() {
        assert_eq!(find_perimeter(6, 7), 26);
        assert_eq!(find_perimeter(20, 10), 60);
        assert_eq!(find_perimeter(2, 9), 22);
    }
}
