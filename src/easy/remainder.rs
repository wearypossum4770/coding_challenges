/// https://edabit.com/challenge/Q2j5FTFtsk7PdzrQk
pub fn remainder(param1: i32, param2: i32) -> i32 {
    param1 % param2
}

#[cfg(test)]
mod tests {
    use super::remainder;
    #[test]
    fn test_remainder() {
        assert_eq!(remainder(7, 2), 1);
        assert_eq!(remainder(3, 4), 3);
        assert_eq!(remainder(-9, 45), -9);
        assert_eq!(remainder(5, 5), 0);
    }
}
