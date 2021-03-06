pub fn fizz_buzz(){

}
#[cfg(test)]
mod tests {
    use super::fizz_buzz;
    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz("1 2 3 4 10 11"), 31);
    }
}