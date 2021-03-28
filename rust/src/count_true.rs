/// Create a method(function) that counts true values in a mutable array.
/// ```
///let result = doccomments::count_true(&mut [true, true, false, false, true]);
/// assert_eq!(result,3)
/// ```

pub fn count_true(arr: &mut [bool]) -> i32 {
    let mut num = 0;
    for value in arr {
        if *value {
            num += 1;
        }
    }
    num
}
#[cfg(test)]
mod tests {
    use super::count_true;
    #[test]
    fn test_count_true() {
        assert_eq!(count_true(&mut [true, false, false, true, false]), 2);
        assert_eq!(count_true(&mut [false, false, false, false]), 0);
        assert_eq!(count_true(&mut []), 0);
        assert_eq!(
            count_true(&mut [
                false, false, true, true, false, false, false, true, true, true, true, false, true,
                true, false
            ]),
            8
        );
        assert_eq!(
            count_true(&mut [true, false, true, true, false, false, false, false, false]),
            3
        );
        assert_eq!(
            count_true(&mut [
                false, true, true, false, true, true, false, true, false, true, false, true, false,
                true, false
            ]),
            8
        );
        assert_eq!(
            count_true(&mut [true, false, true, true, true, false, true, true, false, false]),
            6
        );
        assert_eq!(
            count_true(&mut [
                false, false, false, false, true, false, true, false, true, false, false
            ]),
            3
        );
        assert_eq!(
            count_true(&mut [
                true, false, false, false, true, false, false, true, false, false, false
            ]),
            3
        );
        assert_eq!(
            count_true(&mut [true, true, false, true, false, false, false, false, true, false]),
            4
        );
        assert_eq!(
            count_true(&mut [
                true, false, true, true, false, true, true, true, true, false, true, false, true,
                false
            ]),
            9
        );
        assert_eq!(
            count_true(&mut [
                true, false, true, true, true, true, false, true, true, false, true, false, false,
                false, false
            ]),
            8
        );
        assert_eq!(
            count_true(&mut [
                true, true, false, false, false, false, true, false, true, true, false, true
            ]),
            6
        );
    }
}
// m
