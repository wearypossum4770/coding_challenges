pub fn convert_hour_minutes_to_seconds(hours: u16, minutes: u16) -> u16 {
    hours * 3600 + minutes * 60
}
#[cfg(test)]
mod tests {
    use super::convert_hour_minutes_to_seconds;
    #[test]
    fn test_convert_hour_minutes_to_seconds() {
        assert_eq!(convert_hour_minutes_to_seconds(1, 0), 3600);
        assert_eq!(convert_hour_minutes_to_seconds(1, 3), 3780);
        assert_eq!(convert_hour_minutes_to_seconds(0, 30), 1800);
    }
}
