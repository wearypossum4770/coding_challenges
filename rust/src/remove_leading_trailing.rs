pub fn remove_leading_and_trailing(num: &str)->String{
    
}
#[cfg(test)]
mod tests {
    use super::remove_leading_and_trailing;
    #[test]
    fn test_remove_leading_and_trailing() {
        assert_eq!(remove_leading_and_trailing("230.000"), "230");
        assert_eq!(remove_leading_and_trailing("00402"), "402");
        assert_eq!(remove_leading_and_trailing("03.1400"), "3.14");
        assert_eq!(remove_leading_and_trailing("30"), "30");
        assert_eq!(remove_leading_and_trailing("30.0000"), "30");
        assert_eq!(remove_leading_and_trailing("24340"), "24340");
        assert_eq!(remove_leading_and_trailing("0404040"), "404040");
        assert_eq!(remove_leading_and_trailing("0"), "0");
        assert_eq!(remove_leading_and_trailing("00"), "0");
        assert_eq!(remove_leading_and_trailing("0.000000"), "0");
        assert_eq!(remove_leading_and_trailing("0000.000"), "0");
        assert_eq!(remove_leading_and_trailing("00.0001"), "0.0001");
        assert_eq!(remove_leading_and_trailing("10000"), "10000");
        assert_eq!(remove_leading_and_trailing("1345"), "1345");
        assert_eq!(remove_leading_and_trailing("30.000020"), "30.00002");
        assert_eq!(remove_leading_and_trailing("00200.1900001"), "200.1900001");
    }
}