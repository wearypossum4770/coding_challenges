pub fn make_array_consecutive(statues:Vec<i32>)->i32{

}

#[cfg(test)]
mod tests {
    use super::make_array_consecutive;
    #[test]
    fn test_make_array_consecutive() {
        assert_eq!(make_array_consecutive(1), 1);
        assert_eq!(make_array_consecutive(2), 5);
        assert_eq!(make_array_consecutive(3), 13);
        assert_eq!(make_array_consecutive(4), 25);
        assert_eq!(make_array_consecutive(5), 41);
    }
}