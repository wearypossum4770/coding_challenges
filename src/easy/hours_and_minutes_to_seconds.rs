/// https://edabit.com/challenge/JesaFi5ntBEbGT8bu
pub fn hours_and_minutes_to_seconds(hours: u32, minutes: u32) -> u32 {
    hours * 3_600 + minutes * 60
}
#[cfg(test)]
mod tests {
    use super::hours_and_minutes_to_seconds;
    #[test]
    fn test_hours_and_minutes_to_seconds() {
        assert_eq!(hours_and_minutes_to_seconds(1, 0), 3600);
        assert_eq!(hours_and_minutes_to_seconds(1, 3), 3780);
        assert_eq!(hours_and_minutes_to_seconds(0, 30), 1800);
        assert_eq!(hours_and_minutes_to_seconds(1, 3), 3_780);
        assert_eq!(hours_and_minutes_to_seconds(2, 0), 7_200);
        assert_eq!(hours_and_minutes_to_seconds(0, 0), 0);
    }
}
