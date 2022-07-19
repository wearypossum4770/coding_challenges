use std::collections::HashMap;

pub fn perimeter(shape: &str, value: u32) -> f64 {
    let factor: HashMap<&str, f64> =
        HashMap::from([("s", 4.0), ("S", 4.0), ("C", 6.28), ("c", 6.28)]);
    factor[shape] * value as f64
}

#[cfg(test)]
mod tests {
    use super::perimeter;
    #[test]
    fn test_perimeter() {
        assert_eq!(perimeter("s", 1), 4.0);
        assert_eq!(perimeter("s", 4), 16.0);
        assert_eq!(perimeter("s", 9), 36.0);
        assert_eq!(perimeter("s", 13), 52.0);
        assert_eq!(perimeter("s", 30), 120.0);
        assert_eq!(perimeter("c", 1), 6.28);
        assert_eq!(perimeter("c", 4), 25.12);
        assert_eq!(perimeter("c", 9), 56.52);
        assert_eq!(perimeter("c", 13), 81.64);
        assert_eq!(perimeter("c", 30), 188.4);
    }
}
