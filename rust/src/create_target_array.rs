/// @copyright  https://leetcode.com/problems/create-target-array-in-the-given-order/submissions/
/// Leetcode Problem 1389. Create Target Array in the Given Order.
/// Runtime: 0 ms, faster than 100.00%.
/// Memory Usage: 2.1 MB, less than 76.47% of R
pub fn create_target_array(nums: &[i32], indices: &[i32]) -> Vec<i32> {
    let mut target: Vec<i32> = Vec::with_capacity(nums.len());
    let mut iterator = nums.iter();
    for index in indices.iter() {
        target.insert(*index as usize, *iterator.next().unwrap());
    }
    target
}
#[cfg(test)]
mod tests {
    use super::create_target_array;
    #[test]
    fn test_create_target_array() {
        assert_eq!(
            create_target_array(&[0, 1, 2, 3, 4], &[0, 1, 2, 2, 1]),
            [0, 4, 1, 3, 2]
        );
        assert_eq!(
            create_target_array(&[1, 2, 3, 4, 0], &[0, 1, 2, 3, 0]),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(create_target_array(&[1], &[0]), [1]);
    }
}
