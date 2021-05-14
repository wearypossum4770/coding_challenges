pub fn count_characters(uncounted_string: &str, query: &str) -> u32 {
    let mut count = 0u32;
    for letter in uncounted_string.chars() {
        if query.contains(letter) {
            count += 1;
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::count_characters;
    #[test]
    fn test_count_characters() {
        assert_eq!(count_characters("aaabbc", "a"), 3);
        assert_eq!(count_characters("aaabbc", "a",), 3);
        assert_eq!(count_characters("aaabbc", "b",), 2);
        assert_eq!(count_characters("aaabbc", "c",), 1);
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "a",),
            6
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "e",),
            4
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "l",),
            3
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "p",),
            3
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "w",),
            2
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "d",),
            2
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "o",),
            2
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "t",),
            2
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "y",),
            2
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "k",),
            1
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "h",),
            1
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "i",),
            1
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "c",),
            1
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "n",),
            1
        );
        assert_eq!(
            count_characters("rplyoacadawpettlleaodeeywapkniha", "r",),
            1
        );
    }
}
