/// https://edabit.com/challenge/654ABGmNS5GqscE8C
use std::collections::HashMap;

pub fn get_vote_count(votes: HashMap<&str, i32>) -> i32 {
    votes["upvotes"] - votes["downvotes"]
}

#[cfg(test)]
mod tests {
    use super::get_vote_count;
    use rstest::rstest;
    use std::collections::HashMap;
    #[rstest]
    #[case(13, 0, 13)]
    #[case(2, 33, -31)]
    #[case(132, 132, 0)]
    #[case(0, 0, 0)]
    #[case(4, 1, 3)]
    fn test_get_vote_count(#[case] upvotes: i32, #[case] downvotes: i32, #[case] expected: i32) {
        let value = [("upvotes", upvotes), ("downvotes", downvotes)];
        assert_eq!(get_vote_count(HashMap::from(value)), expected);
    }
}
