#![allow(unused)]
#![feature(int_log)]
pub fn num_of_digits(num: i64) -> u32 {
    if num < 10 {
        1
    } else {
        num.abs().log10()
    }
}

#[cfg(test)]
mod tests {
    use super::num_of_digits;
    #[test]
    fn test_num_of_digits() {
        assert_eq!(num_of_digits(13124), 5);
        assert_eq!(num_of_digits(0), 1);
        assert_eq!(num_of_digits(-12381428), 8);
        assert_eq!(num_of_digits(12), 2);
        assert_eq!(num_of_digits(42), 2);
        assert_eq!(num_of_digits(1000), 4);
        assert_eq!(num_of_digits(136), 3);
        assert_eq!(num_of_digits(1000000000), 10);
        assert_eq!(num_of_digits(2147483647), 10);
        assert_eq!(num_of_digits(-2147483647), 10);
    }
}
