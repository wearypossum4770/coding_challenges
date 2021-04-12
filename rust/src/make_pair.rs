pub fn make_pair(a:i32, b:i32)->Vec<i32>{

}

#[cfg(test)]
mod tests {
    use super::make_pair;
    #[test]
    fn test_make_pair() {
        assert_eq!(make_pair(1, 2), [1, 2]);
        assert_eq!(make_pair(21, 82), [21, 82]);
        assert_eq!(make_pair(4213, 526), [4213, 526]);    }
}