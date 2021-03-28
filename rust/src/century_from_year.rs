pub fn century_from_year(year: u16) -> u16 {
    let mut century: u16 = 0;
    if 0 < year && year <= 100 {
        century += 1;
    }
    let string_year:String=year.to_string();
    let start:String=&string_year[..-2];
    let end:String = &string_year[]
    century
}
#[cfg(test)]
mod tests {
    use super::century_from_year;
    #[test]
    fn test_century_from_year() {
        assert_eq!(century_from_year(1905), 20);
        assert_eq!(century_from_year(1700), 17);
        assert_eq!(century_from_year(1988), 20);
        assert_eq!(century_from_year(2000), 20);
        assert_eq!(century_from_year(2001), 21);
        assert_eq!(century_from_year(200), 2);
        assert_eq!(century_from_year(374), 4);
        assert_eq!(century_from_year(45), 1);
        assert_eq!(century_from_year(8), 1);
    }
}
