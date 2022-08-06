///@copyright https://leetcode.com/problems/reverse-integer/
/// not solved, has an error.
pub fn reverse_integer(x: i32) -> i32 {
    let temp: String = x.to_string();
    let mut target: String = String::with_capacity(temp.len());
    if temp.contains("-") {
        target.push('-');
    }
    for (pos, i) in temp.bytes().rev().enumerate() {
        if i > 48 && i <= 57 {
            target.push(i as char);
        }
    }
    target.parse().unwrap()
}
#[cfg(test)]
mod tests {
    use super::reverse_integer;
    #[test]
    fn test_reverse_integer() {
        assert_eq!(reverse_integer(123), 321);
        assert_eq!(reverse_integer(-123), -321);
        assert_eq!(reverse_integer(120), 21);
    }
}
