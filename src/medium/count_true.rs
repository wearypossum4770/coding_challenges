pub fn count_true(arr: Vec<bool>) -> u32 {
    let mut count = 0u32;
    for value in arr {
        if value {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::count_true;
    #[test]
    fn test_count_true() {
        assert_eq!(count_true(Vec::from([true, false, false, true, false])), 2);
        assert_eq!(count_true(Vec::from([false, false, false, false])), 0);
        assert_eq!(count_true(Vec::from([])), 0);
        assert_eq!(
            count_true(Vec::from([
                false, false, true, true, false, false, false, true, true, true, true, false, true,
                true, false
            ])),
            8
        );
        assert_eq!(
            count_true(Vec::from([
                true, false, true, true, false, false, false, false, false
            ])),
            3
        );
        assert_eq!(
            count_true(Vec::from([
                false, true, true, false, true, true, false, true, false, true, false, true, false,
                true, false
            ])),
            8
        );
        assert_eq!(
            count_true(Vec::from([
                true, false, true, true, true, false, true, true, false, false
            ])),
            6
        );
        assert_eq!(
            count_true(Vec::from([
                false, false, false, false, true, false, true, false, true, false, false
            ])),
            3
        );
        assert_eq!(
            count_true(Vec::from([
                true, false, false, false, true, false, false, true, false, false, false
            ])),
            3
        );
        assert_eq!(
            count_true(Vec::from([
                true, true, false, true, false, false, false, false, true, false
            ])),
            4
        );
        assert_eq!(
            count_true(Vec::from([
                true, false, true, true, false, true, true, true, true, false, true, false, true,
                false
            ])),
            9
        );
        assert_eq!(
            count_true(Vec::from([
                true, false, true, true, true, true, false, true, true, false, true, false, false,
                false, false
            ])),
            8
        );
        assert_eq!(
            count_true(Vec::from([
                true, true, false, false, false, false, true, false, true, true, false, true
            ])),
            6
        );
    }
}
