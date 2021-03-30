pub fn convert_boolean_to_string(flag:bool)->String{
    if flag{
        "true".to_string()
    }else {
        "false".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::convert_boolean_to_string;
    #[test]
    fn test_convert_boolean_to_string() {
        assert_eq!(convert_boolean_to_string(true), "true");
        assert_eq!(convert_boolean_to_string(false), "false");
        }
}