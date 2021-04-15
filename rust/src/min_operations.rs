///
/// leet code April 2021 challenge https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3698/
///
pub fn min_operations(n: i32) -> i32 {
    if n % 2 == 0 {
        n.pow(2) / 4
    } else {
        (n.pow(2) - 1) / 4
    }
}
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(3), 2);
        assert_eq!(min_operations(6), 9);
        assert_eq!(min_operations(7), 12);
    }
}
