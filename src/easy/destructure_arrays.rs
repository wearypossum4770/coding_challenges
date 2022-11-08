///https://edabit.com/challenge/sWAEoTbXA4bexBPb6
pub fn destructure_arrays(arr: Vec<i64>) -> [i64; 2] {
    let a = arr[0];
    let b = arr[1];
    let target: [i64; 2] = [a, b];
    target
}
#[cfg(test)]
mod tests {
    use super::destructure_arrays;
    #[test]
    fn test_destructure_arrays() {
        assert_eq!(destructure_arrays(vec![1, 2, 3, 4, 5, 6]), [1, 2])
    }
}
