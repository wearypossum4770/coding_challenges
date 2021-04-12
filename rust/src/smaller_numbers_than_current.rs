#[allow(clippy::single_char_pattern)]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();
    for num in nums {
        let mut counter = 0i32;
        for n in nums.iter() {
            if n < num {
                counter += 1;
            }
        }
        array.push(counter);
    }
    array
}
#[cfg(test)]
mod tests {
    use super::smaller_numbers_than_current;
    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            smaller_numbers_than_current(&[8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(&[6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(&[7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
