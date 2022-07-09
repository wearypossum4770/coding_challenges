use std::collections::HashMap;

pub fn get_vote_count(votes: HashMap<&str, i32>) -> i32 {
    votes["upvotes"] - votes["downvotes"]
}

#[cfg(test)]
mod tests {
    use super::get_vote_count;
    use std::collections::HashMap;
    #[test]
    fn test_get_vote_count() {
        assert_eq!(
            get_vote_count(HashMap::from([("upvotes", 13), ("downvotes", 0)])),
            13
        );
        assert_eq!(
            get_vote_count(HashMap::from([("upvotes", 2), ("downvotes", 33)])),
            -31
        );
        assert_eq!(
            get_vote_count(HashMap::from([("upvotes", 132), ("downvotes", 132)])),
            0
        );
        assert_eq!(
            get_vote_count(HashMap::from([("upvotes", 0), ("downvotes", 0)])),
            0
        );
        assert_eq!(
            get_vote_count(HashMap::from([("downvotes", 4), ("upvotes", 1)])),
            -3
        );
    }
}
