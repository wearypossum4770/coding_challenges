/// https://edabit.com/challenge/ZNwHGgHvsdnYwJ5WK
pub fn concat_string(name: &str) -> String {
    format!("{}Edabit", name)
}

#[cfg(test)]
mod tests {
    use super::concat_string;
    #[test]

    fn test_concat_string() {
        assert_eq!(concat_string("Mubashir"), "MubashirEdabit".to_string());
        assert_eq!(concat_string("Matt"), "MattEdabit".to_string());
        assert_eq!(concat_string("javaScript"), "javaScriptEdabit".to_string());
        assert_eq!(concat_string("Airforce"), "AirforceEdabit".to_string());
    }
}
