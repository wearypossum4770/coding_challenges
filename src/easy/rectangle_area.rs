/// https://edabit.com/challenge/g6b9HqkXqWu6GpfTo
pub fn rectangle_area(param1: i32, param2: i32) -> i32 {
    if param1 < 1 || param2 < 1 {
        -1
    } else {
        param1 * param2
    }
}
#[cfg(test)]
mod tests {
    use super::rectangle_area;
    #[test]
    fn test_rectangle_area() {
        assert_eq!(rectangle_area(5, 3), 15);
        assert_eq!(rectangle_area(3, 4), 12);
        assert_eq!(rectangle_area(10, 11), 110);
        assert_eq!(rectangle_area(-1, 5), -1);
        assert_eq!(rectangle_area(0, 2), -1);
        assert_eq!(rectangle_area(8, 5), 40);
        assert_eq!(rectangle_area(5, 4), 20);
        assert_eq!(rectangle_area(2, 3), 6);
        assert_eq!(rectangle_area(10000, 10000), 100000000);
        assert_eq!(rectangle_area(-2, -5), -1);
        assert_eq!(rectangle_area(0, 3), -1);
        assert_eq!(rectangle_area(5, -3), -1);
        assert_eq!(rectangle_area(0, 1), -1);
        assert_eq!(rectangle_area(-1, 0), -1);
    }
}
