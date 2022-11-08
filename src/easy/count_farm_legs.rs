/// https://edabit.com/challenge/8Qg78sf5SNDEANKti
pub fn count_farm_legs(chickens: u32, cows: u32, pigs: u32) -> u32 {
    chickens * 2 + cows * 4 + pigs * 4
}
#[cfg(test)]
mod tests {
    use super::count_farm_legs;
    #[test]
    fn test_count_farm_legs() {
        assert_eq!(count_farm_legs(5, 2, 8), 50);
        assert_eq!(count_farm_legs(2, 3, 5), 36);
        assert_eq!(count_farm_legs(3, 4, 7), 50);
        assert_eq!(count_farm_legs(1, 2, 3), 22);
        assert_eq!(count_farm_legs(3, 5, 2), 34);
    }
}
