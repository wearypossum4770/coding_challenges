pub fn how_many_walls(bucket: u32, width: u32, length: u32) -> u32 {
    bucket / (width * length)
}

#[cfg(test)]
mod tests {
    use super::how_many_walls;
    #[test]
    fn test_how_many_walls() {
        assert_eq!(how_many_walls(100, 4, 5), 5);
        assert_eq!(how_many_walls(10, 15, 12), 0);
        assert_eq!(how_many_walls(41, 3, 6), 2);
        assert_eq!(how_many_walls(50, 11, 5), 0);
        assert_eq!(how_many_walls(1, 1, 1), 1);
        // Author: Joshua Señoron
    }
}
