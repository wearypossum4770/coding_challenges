pub fn interesting_polygon(num:u32)->u32{
let exponential = num.pow(2);
2*exponential-2*num+1
}

#[cfg(test)]
mod tests {
    use super::interesting_polygon;
    #[test]
    fn test_interesting_polygon() {
        assert_eq!(interesting_polygon(1), 1);
        assert_eq!(interesting_polygon(2), 5);
        assert_eq!(interesting_polygon(3), 13);
        assert_eq!(interesting_polygon(4), 25);
        assert_eq!(interesting_polygon(5), 41);
    }
}