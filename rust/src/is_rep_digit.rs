pub fn is_rep_digit(num: i32) -> bool {
    let mut target = true;
    if num < 0 {
        target = false;
    } else {
        let mut num_string: String = num.to_string();
        let val = num_string.pop().expect("");
        for n in num_string.chars() {
            if val != n {
                target = false;
            }
        }
    }
    target
}
#[cfg(test)]
mod tests {
    use super::is_rep_digit;
    #[test]
    fn test_is_rep_digit() {
        assert_eq!(is_rep_digit(0), true);
        assert_eq!(is_rep_digit(6), true);
        assert_eq!(is_rep_digit(66), true);
        assert_eq!(is_rep_digit(777), true);
        assert_eq!(is_rep_digit(7777), true);
        assert_eq!(is_rep_digit(1001), false);
        assert_eq!(is_rep_digit(-11), false);
    }
}
