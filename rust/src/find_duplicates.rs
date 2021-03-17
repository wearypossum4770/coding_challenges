pub fn find_duplicates(array: &[usize])-> &[usize]{

}
#[cfg(test)]
mod tests {
    use super::find_duplicates;
    let test_input = vec![4, 3, 2, 7, 8, 2, 3, 1]
    let expected = vec![2,3]
    #[test]
    fn test_find_duplicates() {
        assert_eq!(find_duplicates(), expected);
    }
}