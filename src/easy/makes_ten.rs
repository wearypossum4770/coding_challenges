///https://edabit.com/challenge/5erCDJ8eJDrXkmwTK
pub fn makes_ten(a: i32, b: i32) -> bool {
    if a == 10 || b == 10 {
        true
    } else {
        a + b == 10
    }
}
#[cfg(test)]
mod tests {
    use super::makes_ten;
    use rstest::rstest;
    #[rstest]
    #[case(9, 10, true)]
    #[case(9, 9, false)]
    #[case(1, 9, true)]
    #[case(10, 1, true)]
    #[case(10, 10, true)]
    #[case(8, 2, true)]
    #[case(8, 3, false)]
    #[case(10, 42, true)]
    #[case(12, -2, true)]
    fn test_makes_ten(#[case] param1: i32, #[case] param2: i32, #[case] expected: bool) {
        assert_eq!(makes_ten(param1, param2), expected);
    }
}
