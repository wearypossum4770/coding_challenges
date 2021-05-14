pub fn ascending_descending_none(arr: &[i32], method: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = arr.to_vec();
    if method == "Asc" {
        nums.sort_unstable();
    }
    if method == "Des" {
        nums.sort_by(|a, b| b.cmp(a));
    }
    nums.to_vec()
}
#[cfg(test)]
mod tests {
    use super::ascending_descending_none;
    #[test]
    fn test_ascending_descending_none() {
        assert_eq!(
            ascending_descending_none(&[4, 3, 2, 1], "Asc"),
            [1, 2, 3, 4]
        );
        assert_eq!(
            ascending_descending_none(&[7, 8, 11, 66], "Des"),
            [66, 11, 8, 7]
        );
        assert_eq!(
            ascending_descending_none(&[1, 2, 3, 4], "None"),
            [1, 2, 3, 4]
        );
        assert_eq!(
            ascending_descending_none(&[1, 0, 1, 0], "Asc"),
            [0, 0, 1, 1]
        );
        assert_eq!(
            ascending_descending_none(&[1, 2, 2, 2, 2, 2, 2], "Des"),
            [2, 2, 2, 2, 2, 2, 1]
        );
        assert_eq!(
            ascending_descending_none(&[9, 7, 43, 11, 16, 111, 19], "Asc"),
            [7, 9, 11, 16, 19, 43, 111]
        );
    }
}
