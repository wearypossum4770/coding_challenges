pub fn tetrahedral_number(num: u32) -> u32 {
    let mut tetrahedral = 0u32;
    for i in 1..=num {
        let current: u32 = (i * (i + 1)) / 2;
        tetrahedral += current;
    }
    tetrahedral
}
#[cfg(test)]
mod tests {
    use super::tetrahedral_number;
    #[test]
    fn test_tetrahedral_number() {
        assert_eq!(tetrahedral_number(1), 1);
        assert_eq!(tetrahedral_number(2), 4);
        assert_eq!(tetrahedral_number(3), 10);
        assert_eq!(tetrahedral_number(4), 20);
        assert_eq!(tetrahedral_number(5), 35);
        assert_eq!(tetrahedral_number(9), 165);
    }
}
