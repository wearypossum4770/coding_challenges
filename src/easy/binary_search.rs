/// https://leetcode.com/problems/binary-search/
/// Problem: 704 Binary Search
/// Runtime: 2 ms, faster than 97.26%
/// Memory Usage: 2.2 MB, less than 42.05%
pub fn binary_search(nums: &[i32], target: i32) -> i32 {
    let mut value: i32 = -1;
    for (index, num) in nums.iter().enumerate() {
        if *num == target {
            value = index as i32;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
