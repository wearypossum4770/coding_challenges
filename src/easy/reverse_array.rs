/// https://edabit.com/challenge/kJQYTCCWSnzhXG9dn
pub fn reverse_array(arr: &[u8]) -> Vec<u8> {
    let mut target: Vec<u8> = Vec::with_capacity(arr.len());
    for num in arr.iter().rev() {
        target.push(*num)
    }
    target
}
#[cfg(test)]
mod tests {
    use super::reverse_array;
    #[test]
    fn test_reverse_array() {
        assert_eq!(reverse_array(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(reverse_array(&[5, 6, 7]), vec![7, 6, 5]);
        assert_eq!(reverse_array(&[9, 9, 2, 3, 4]), vec![4, 3, 2, 9, 9]);
        assert_eq!(reverse_array(&[3, 3]), vec![3, 3]);
        assert_eq!(reverse_array(&[]), vec![]);
    }
}
