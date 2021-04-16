///
/// https://app.codesignal.com/arcade/intro/level-2/xzKiBHjhoinnpdh6m
///
#[allow(dead_code)]
pub fn adjacent_elements_product(arr: &[i32]) -> i32 {
    let mut target = arr[0]*arr[1];
    let mut product = 0i32;
    for num in arr{
        product=*num;
        println!("{:?}",product);
    }
    target
}
#[cfg(test)]
mod tests {
    use super::adjacent_elements_product;
    #[test]
    fn test_adjacent_elements_product() {
        assert_eq!(adjacent_elements_product(&[3, 6, -2, -5, 7, 3]), 21);

        // assert_eq!(adjacent_elements_product(&[-1, -2]), 2);

        // assert_eq!(adjacent_elements_product(& [5, 1, 2, 3, 1, 4]), 6);
        // assert_eq!(adjacent_elements_product(& [5, 1, 2, 3, 1, 4]), 6);
    }
}
