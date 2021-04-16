/// Leet Code 1528
/// https://leetcode.com/problems/shuffle-string/
///
///
pub fn shuffle_string(word: &str, arr: &[u8]) -> String {
    let mut new_word:Vec<char> = Vec::with_capacity(word.len());
    let mut target:String=String::new();
    // let mut new_word = String::new();
    for (i, val) in word.chars().enumerate() {
        new_word.insert(arr[i].into(),val);
        println!("suffle string");
        println!("{:?}", val);
        println!("{:?}", arr[i]);
        println!("suffle string");
    }
    target=new_word.into_iter().map(|i| i.to_string()).collect();
target
}
#[cfg(test)]
mod tests {
    use super::shuffle_string;
    #[test]
    fn test_shuffle_string() {
        assert_eq!(
            shuffle_string("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode"
        );
        assert_eq!(shuffle_string("abc", &[0, 1, 2]), "abc");
        assert_eq!(shuffle_string("aiohn", &[3, 1, 4, 2, 0]), "nihao");
        assert_eq!(
            shuffle_string("aaiougrt", &[4, 0, 2, 6, 7, 3, 1, 5]),
            "arigatou"
        );
        assert_eq!(shuffle_string("art", &[1, 0, 2]), "rat");
    }
}
