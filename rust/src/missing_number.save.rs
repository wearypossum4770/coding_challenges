pub fn missing_number(arr: Vec<u32>) -> u32 {
    let mut missing = 0u32;
    let array_iter = arr.iter();
    let max_value = array_iter.max();
    match max_value {
        Some(max) => missing = *max,
        None => println!("Vector is empty"),
    }
    missing
}
// #[cfg(test)]
// mod tests {
//     use super::missing_number;
//     #[test]
//     fn test_missing_number() {
//         assert_eq!(missing_number(vec![3, 0, 1]), 2);
//         assert_eq!(missing_number(vec![0, 1]), 2);
//         assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
//         assert_eq!(missing_number(vec![0]), 1);
//     }
// }
