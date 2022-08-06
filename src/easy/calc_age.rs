/// @copyright https://edabit.com/challenge/bL7hSc6Zh4zZJzGmw
pub fn calc_age(age: u8) -> u32 {
    age as u32 * 365
}

#[cfg(test)]
mod tests {
    use super::calc_age;
    #[test]
    fn test_calc_age() {
        assert_eq!(calc_age(65), 23_725);
        assert_eq!(calc_age(10), 3_650);
        assert_eq!(calc_age(20), 7_300);
        assert_eq!(calc_age(0), 0);
        assert_eq!(calc_age(73), 26_645);
    }
}
