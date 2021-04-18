pub fn cutting_paper_squares(n: u64, m: u64) -> u64 {
    n * m - 1
}
#[cfg(test)]
mod tests {
    use super::cutting_paper_squares;
    #[test]
    fn test_cutting_paper_squares() {
        assert_eq!(cutting_paper_squares(3, 1), 2);
        assert_eq!(cutting_paper_squares(1, 1), 0);
        assert_eq!(cutting_paper_squares(12, 12), 143);
        assert_eq!(
            cutting_paper_squares(689715240, 759842301),
            524074814996367239
        );
    }
}
