/// https://edabit.com/challenge/JfB9mWmbwYHbupxCB
pub fn divides_evenly(a: i32, b: i32) -> bool {
    a % b == 0
}
#[cfg(test)]
mod tests {
    use super::divides_evenly;
    #[test]
    fn test_divides_evenly() {
        assert_eq!(divides_evenly(98, 7), true);
        assert_eq!(divides_evenly(87, 49), false);
        assert_eq!(divides_evenly(34, 14), false);
        assert_eq!(divides_evenly(78, 6), true);
        assert_eq!(divides_evenly(30, 4), false);
        assert_eq!(divides_evenly(87, 73), false);
        assert_eq!(divides_evenly(74, 7), false);
        assert_eq!(divides_evenly(87, 29), true);
        assert_eq!(divides_evenly(48, 24), true);
        assert_eq!(divides_evenly(99, 20), false);
        assert_eq!(divides_evenly(98, 49), true);
        assert_eq!(divides_evenly(100, 6), false);
        assert_eq!(divides_evenly(64, 4), true);
        assert_eq!(divides_evenly(70, 35), true);
        assert_eq!(divides_evenly(38, 38), true);
        assert_eq!(divides_evenly(29, 3), false);
        assert_eq!(divides_evenly(20, 8), false);
        assert_eq!(divides_evenly(66, 50), false);
        assert_eq!(divides_evenly(95, 1), true);
        assert_eq!(divides_evenly(58, 2), true);
    }
}
