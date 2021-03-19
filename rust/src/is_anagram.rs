pub fn is_anagram(string1:&str, string2:&str) ->bool{
    let first:String = string1.trim().to_lowercase().chars().rev().filter(|c| c.is_alphanumeric()).collect::<String>();
        // let mut first = String::from(string1).iter().rev().collect::<String>();
    // for x in &first {
    //     
    // }
    // let mut second = String::new(); 
    println!("{:?}", first);
    let second:String = string2.trim().to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
return true
}
#[cfg(test)]
mod tests {
    use super::is_anagram;
    #[test]
    fn test_is_anagram() {
        // assert_eq!(is_anagram("charm", "march") , true);
        assert_eq!(is_anagram("zach", "attack") , false);
        // assert_eq!(is_anagram("CharM", "mARcH") , true);
        // assert_eq!(is_anagram("abcde2", "c2abed") , true);
    }
}