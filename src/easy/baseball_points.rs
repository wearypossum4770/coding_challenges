/// https://edabit.com/challenge/Y46Xp2pcvTB77bmdD
pub fn baseball_points(two: u32, three: u32) -> u32 {
    two * 2 + three * 3
}
#[cfg(test)]
mod tests {
    use super::baseball_points;
    #[test]
    fn test_baseball_points() {
        assert_eq!(baseball_points(1, 1), 5);
        assert_eq!(baseball_points(1, 2), 8);
        assert_eq!(baseball_points(2, 1), 7);
        assert_eq!(baseball_points(7, 5), 29);
        assert_eq!(baseball_points(38, 8), 100);
        assert_eq!(baseball_points(2, 2), 10);
        assert_eq!(baseball_points(69, 420), 1398);
    }
}
