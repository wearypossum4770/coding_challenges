#[allow(clippy::int_plus_one)]
pub fn possible_bonus(player1: u8, player2: u8) -> bool {
    player1 + 1 <= player2 && player2 <= player1 + 6
}
#[cfg(test)]
mod tests {
    use super::possible_bonus;
    #[test]
    fn test_possible_bonus() {
        assert_eq!(possible_bonus(3, 7), true);
        assert_eq!(possible_bonus(0, 6), true);
        assert_eq!(possible_bonus(1, 6), true);
        assert_eq!(possible_bonus(2, 6), true);
        assert_eq!(possible_bonus(3, 6), true);
        assert_eq!(possible_bonus(4, 6), true);
        assert_eq!(possible_bonus(5, 6), true);
        assert_eq!(possible_bonus(6, 6), false);
        assert_eq!(possible_bonus(7, 6), false);
        assert_eq!(possible_bonus(23, 27), true);
        assert_eq!(possible_bonus(1, 9), false);
        assert_eq!(possible_bonus(5, 3), false);
    }
}
