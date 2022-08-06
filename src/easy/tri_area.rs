pub fn tri_area(base: i32, height: i32) -> i32 {
    base * height / 2
}
#[cfg(test)]
mod tests {
    use super::tri_area;
    #[test]
    fn test_tri_area() {
        assert_eq!(tri_area(3, 2), 3);
        assert_eq!(tri_area(5, 4), 10);
        assert_eq!(tri_area(10, 10), 50);
        assert_eq!(tri_area(0, 60), 0);
        assert_eq!(tri_area(12, 11), 66);
        assert_eq!(tri_area(7, 4), 14);
    }
}
