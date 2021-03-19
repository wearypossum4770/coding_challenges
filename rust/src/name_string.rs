pub fn name_string(name: &str)->String{
    let mut target:String = String::from(name);
    target.push_str("Edabit");
    target
    

}
#[cfg(test)]
mod tests {
    use super::name_string;
    #[test]
    fn test_name_string() {
assert_eq!(name_string("Mubashir"), "MubashirEdabit");
assert_eq!(name_string("Matt"), "MattEdabit");
assert_eq!(name_string("javaScript"), "javaScriptEdabit");
assert_eq!(name_string("Airforce"), "AirforceEdabit");
    }
}