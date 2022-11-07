/// https://edabit.com/challenge/Y46Xp2pcvTB77bmdD
pub fn count_points(two: u32, three: u32) -> u32 {
    two * 2 + three * 3
}

#[cfg(test)]
mod tests {
    use super::count_points;
    #[test]
    fn test_count_points() {
        assert_eq!(count_points(1, 1), 5);
        assert_eq!(count_points(1, 2), 8);
        assert_eq!(count_points(2, 1), 7);
        assert_eq!(count_points(2, 2), 10);
        assert_eq!(count_points(69, 420), 1398);
    }
}
