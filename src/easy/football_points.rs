/// https://edabit.com/challenge/GwvwXHWCThHZrR7xu
pub fn football_points(wins: u32, draws: u32, losses: u32) -> u32 {
    wins * 3 + draws * 1 + losses * 0
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
        assert_eq!(football_points(3, 4, 2), 13);
        assert_eq!(football_points(5, 0, 2), 15);
        assert_eq!(football_points(0, 0, 1), 0);
    }
}
