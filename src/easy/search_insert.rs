/*
Runtime: 0 ms, faster than 100.00% .
Memory Usage: 2 MB, less than 91.08% .
@copyright https://leetcode.com/problems/search-insert-position/submissions/
*/
pub fn search_insert(nums: Vec<i32>, target: i32) -> u16 {
    for (pos, num) in nums.iter().enumerate() {
        if num >= &target {
            return pos as u16;
        }
    }
    nums.len() as u16
}

#[cfg(test)]
mod tests {
    use super::search_insert;
    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 5), 2);
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 2), 1);
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 7), 4);
    }
}
