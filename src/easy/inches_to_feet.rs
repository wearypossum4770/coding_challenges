/// https://edabit.com/challenge/fZqTozX8XHZxqcdA8
pub fn inches_to_feet(inches: i64) -> i64 {
    if inches < 12 {
        0
    } else {
        inches / 12
    }
}
#[cfg(test)]
mod tests {
    use super::inches_to_feet;
    #[test]
    fn test_inches_to_feet() {
        assert_eq!(inches_to_feet(12), 1);
        assert_eq!(inches_to_feet(360), 30);
        assert_eq!(inches_to_feet(3_612), 301);
        assert_eq!(inches_to_feet(324), 27);
        assert_eq!(inches_to_feet(3_012), 251);
        assert_eq!(inches_to_feet(11), 0);
    }
}
