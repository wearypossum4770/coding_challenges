use num_traits::pow;
pub fn cubes(a: i64) -> i64 {
    return pow(a, 3);
}

#[cfg(test)]
mod tests {
    use super::cubes;

    #[test]
    fn test_cubes() {
        assert_eq!(cubes(2), 8);
        assert_eq!(cubes(3), 27);
        assert_eq!(cubes(4), 64);
        assert_eq!(cubes(5), 125);
        assert_eq!(cubes(10), 1000);
    }
}
