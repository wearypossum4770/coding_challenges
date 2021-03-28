pub fn animals(chickens: u32, cows: u32, pigs: u32) -> u32 {
    chickens * 2 + cows * 4 + pigs * 4
}

#[cfg(test)]
mod tests {
    use super::animals;
    #[test]
    fn test_animals() {
        assert_eq!(animals(5, 2, 8), 50);
        assert_eq!(animals(3, 4, 7), 50);
        assert_eq!(animals(1, 2, 3), 22);
        assert_eq!(animals(3, 5, 2), 34);
    }
}
