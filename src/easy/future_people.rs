/// https://edabit.com/challenge/DcmB9Ycm58FdkPe7k
pub fn future_people(population: i32, num: i32) -> i32 {
    population + 360 * num
}
#[cfg(test)]
mod tests {
    use super::future_people;
    #[test]
    fn test_future_people() {
        assert_eq!(future_people(256, 2), 976);
        assert_eq!(future_people(3_248, 6), 5_408);
        assert_eq!(future_people(5_240, 3), 6_320);
    }
}
