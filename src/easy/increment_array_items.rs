/// https://edabit.com/challenge/jipHTDkabftop5irE
pub fn increment_array_items(items: [i8; 4]) -> [i8; 4] {
    items
        .iter()
        .map(|n| n + 1)
        .collect::<Vec<i8>>()
        .try_into()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::increment_array_items;
    #[test]
    fn test_increment_array_items() {
        assert_eq!(increment_array_items([0, 1, 2, 3]), [1, 2, 3, 4]);
        assert_eq!(increment_array_items([2, 4, 6, 8]), [3, 5, 7, 9]);
        assert_eq!(increment_array_items([-1, -2, -3, -4]), [0, -1, -2, -3]);
    }
}
