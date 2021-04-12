/// Takes an number to multiply (integer) and a length (integer) and returns a vector of the multiples of number.
/// # Examples
/// ```
/// let function  = array_of_multiples(7,5);
/// let answer = vec![7, 14, 21, 28, 35];
///
/// assert_eq!(function, answer);
/// ```
/// https://edabit.com/challenge/ebcd4Xu8TLizaj6dm
pub fn array_of_multiples(num: u32, length: u32) -> Vec<u32> {
    let mut array: Vec<u32> = Vec::new();
    for i in 1..=length {
        array.push(num * i);
    }
    array
}
#[cfg(test)]
mod tests {
    use super::array_of_multiples;
    #[test]
    fn test_array_of_multiples() {
        assert_eq!(array_of_multiples(7, 5), vec![7, 14, 21, 28, 35]);
        assert_eq!(
            array_of_multiples(12, 10),
            vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120]
        );
        assert_eq!(
            array_of_multiples(17, 7),
            vec![17, 34, 51, 68, 85, 102, 119]
        );
        assert_eq!(
            array_of_multiples(630, 14),
            vec![630, 1260, 1890, 2520, 3150, 3780, 4410, 5040, 5670, 6300, 6930, 7560, 8190, 8820]
        );
        assert_eq!(array_of_multiples(140, 3), vec![140, 280, 420]);
        assert_eq!(
            array_of_multiples(7, 8),
            vec![7, 14, 21, 28, 35, 42, 49, 56]
        );
        assert_eq!(
            array_of_multiples(11, 21),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 110, 121, 132, 143, 154, 165, 176, 187, 198,
                209, 220, 231
            ]
        );
    }
}
