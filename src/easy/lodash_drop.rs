/// https://edabit.com/challenge/NMdKxEradTmpNnomZ
pub fn lodash_drop(param: &[u8], elements: usize) -> &[u8] {
    if param.len() < elements {
        &[]
    } else {
        &param[elements..]
    }
}
#[cfg(test)]
mod tests {
    use super::lodash_drop;
    #[test]
    fn test_lodash_drop() {
        assert_eq!(lodash_drop(&[1, 2, 3], 1), &[2, 3]);
        assert_eq!(lodash_drop(&[1, 2, 3], 2), &[3]);
        assert_eq!(lodash_drop(&[1, 2, 3], 5), &[]);
        assert_eq!(lodash_drop(&[1, 2, 3], 0), &[1, 2, 3]);
        // assert_eq!(lodash_drop(&["banana", "orange", "watermelon", "mango"], 2), ["watermelon", "mango"]);
        assert_eq!(lodash_drop(&[], 2), &[]);
    }
}
