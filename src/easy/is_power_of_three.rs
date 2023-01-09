pub fn is_power_of_three(n: i32) -> bool {
    if n == 0 {
        false
    }else {
        n.pow(1/2) as i32 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::is_power_of_three;
    #[test]
    fn test_is_power_of_three() {
        assert_eq!(is_power_of_three(27), true);
        assert_eq!(is_power_of_three(0), false);
        assert_eq!(is_power_of_three(9), true);
        assert_eq!(is_power_of_three(45), false);

    }
}
