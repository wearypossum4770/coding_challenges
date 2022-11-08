/// https://edabit.com/challenge/ghbHrRnRiDz9fvQNF
pub fn profitable_gamble(prob: f64, prize: f64, pay: f64) -> bool {
    prob * prize > pay
}
#[cfg(test)]
mod tests {
    use super::profitable_gamble;
    #[test]
    fn test_profitable_gamble() {
        assert_eq!(profitable_gamble(0.2, 50.0, 9.0), true);
        assert_eq!(profitable_gamble(0.9, 1.0, 2.0), false);
        assert_eq!(profitable_gamble(0.9, 3.0, 2.0), true);
        assert_eq!(profitable_gamble(0.33, 10.0, 3.30), true);
        assert_eq!(profitable_gamble(0.0, 1000.0, 0.01), false);
        assert_eq!(profitable_gamble(0.1, 1000.0, 7.0), true);
        assert_eq!(profitable_gamble(0.0, 0.0, 0.0), false);
    }
}
