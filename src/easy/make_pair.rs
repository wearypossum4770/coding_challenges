/// https://edabit.com/challenge/BFnsRqe8PFvEwcRNt
pub fn make_pair(param1: i64, param2: i64) -> [i64; 2] {
    [param1, param2]
}
#[cfg(test)]
mod tests {
    use super::make_pair;
    #[test]
    fn test_make_pair() {
        assert_eq!(make_pair(1, 2), [1, 2]);
        assert_eq!(make_pair(51, 21), [51, 21]);
        assert_eq!(make_pair(21, 82), [21, 82]);
        assert_eq!(make_pair(512124, 215), [512124, 215]);
        assert_eq!(make_pair(4213, 526), [4213, 526]);
    }
}
