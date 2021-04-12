pub fn check_palindrome(word:&str)->bool{
let length = word.len();
let mut string = String::from(word);
println!("{:?}", string.remove(0));
if length ==  1{
    true
}else{
    
    false
}

}
#[cfg(test)]
mod tests {
    use super::check_palindrome;
    #[test]
    fn test_check_palindrome() {
        // assert_eq!(check_palindrome("aabaa"), true);
        // assert_eq!(check_palindrome("abac"), false);
        assert_eq!(check_palindrome("a"), true);
        // assert_eq!(check_palindrome("az"), false);
        // assert_eq!(check_palindrome("abacaba"), true);
        // assert_eq!(check_palindrome("z"), true);
        // assert_eq!(check_palindrome("aaabaaaa"), false);
        // assert_eq!(check_palindrome("zzzazzazz"), false);
        // assert_eq!(check_palindrome("hlbeeykoqqqqokyeeblh"), true);
        // assert_eq!(check_palindrome("hlbeeykoqqqokyeeblh"), true);
    }
}