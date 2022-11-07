/// https://edabit.com/challenge/XnJ24rWW7iJkNrtsh
pub fn rectangle_perimeter(length: u16, width: u16) -> u16 {
    2 * length + 2 * width
}
#[cfg(test)]
mod tests {
    use super::rectangle_perimeter;
    #[test]
    fn test_rectangle_perimeter() {
        assert_eq!(rectangle_perimeter(6, 7), 26);
        assert_eq!(rectangle_perimeter(20, 10), 60);
        assert_eq!(rectangle_perimeter(2, 9), 22);
    }
}
