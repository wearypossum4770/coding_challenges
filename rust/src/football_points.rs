pub fn football_points(wins: u16, draws: u16, _losses: u16) -> u16 {
    wins * 3 + draws
}
#[cfg(test)]
mod tests {
    use super::football_points;
    #[test]
    fn test_football_points() {
        assert_eq!(football_points(1, 2, 3), 5);
        assert_eq!(football_points(5, 5, 5), 20);
        assert_eq!(football_points(1, 0, 0), 3);
        assert_eq!(football_points(0, 7, 0), 7);
        assert_eq!(football_points(0, 0, 15), 0);
    }
}
