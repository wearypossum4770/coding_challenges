#![feature(option_result_contains)]
pub fn is_rep_digit(num: i32) -> bool {
    if num < 0 {
        false
    } else if num == 0 {
        true
    } else {
        let mut target = true;
        let mut num_string: String = num.to_string();
        let val = num_string.pop().expect("");
        let mut array: Vec<bool> = Vec::with_capacity(num_string.len());

        for n in num_string.chars() {
            if val != n{
                target=false;
            }        
        }
        target
    }
}
#[cfg(test)]
mod tests {
    use super::is_rep_digit;
    #[test]
    fn test_is_rep_digit() {
        assert_eq!(is_rep_digit(6), true);
        assert_eq!(is_rep_digit(66), true);
        assert_eq!(is_rep_digit(777), true);
        assert_eq!(is_rep_digit(7777), true);
        assert_eq!(is_rep_digit(1001), false);
        assert_eq!(is_rep_digit(-11), false);
    }
}
