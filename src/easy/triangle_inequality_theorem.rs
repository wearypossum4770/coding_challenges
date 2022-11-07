/// https://edabit.com/challenge/nhXofMMyrowMyr9Nv
pub fn triangle_inequality_theorem(side1: u16, side2: u16) -> u16 {
    side1 + side2 - 1
}

#[cfg(test)]
mod tests {
    use super::triangle_inequality_theorem;
    #[test]
    fn test_triangle_inequality_theorem() {
        assert_eq!(triangle_inequality_theorem(5, 4), 8);
        assert_eq!(triangle_inequality_theorem(8, 3), 10);
        assert_eq!(triangle_inequality_theorem(7, 9), 15);
        assert_eq!(triangle_inequality_theorem(10, 4), 13);
        assert_eq!(triangle_inequality_theorem(7, 2), 8);
    }
}
