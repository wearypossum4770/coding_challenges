pub fn profitable_gamble(prob: f32, prize: u16, pay: f32) -> bool {
    prob * prize as f32 > pay
}
#[cfg(test)]
mod tests {
    use super::profitable_gamble;
    #[test]
    fn test_profitable_gamble() {
        assert_eq!(profitable_gamble(0.2, 50, 9.0), true);
        assert_eq!(profitable_gamble(0.9, 1, 2.0), false);
        assert_eq!(profitable_gamble(0.9, 3, 2.0), true);
        assert_eq!(profitable_gamble(0.33, 10, 3.30), true);
        assert_eq!(profitable_gamble(0.0, 1000, 0.01), false);
        assert_eq!(profitable_gamble(0.1, 1000, 7.0), true);
        assert_eq!(profitable_gamble(0.0, 0, 0.0), false);
    }
}
