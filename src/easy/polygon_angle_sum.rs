/// https://edabit.com/challenge/fBJyQSe5Jmbm9hPAG
pub fn polygon_angle_sum(sides: u32) -> u32 {
    (sides - 2) * 180
}

#[cfg(test)]
mod tests {
    use super::polygon_angle_sum;
    use rstest::rstest;
    #[rstest]
    #[case(3, 180)]
    #[case(4, 360)]
    #[case(5, 540)]
    #[case(6, 720)]
    #[case(7, 900)]
    #[case(8, 1080)]
    #[case(9, 1260)]
    #[case(10, 1440)]
    #[case(11, 1620)]
    #[case(12, 1800)]
    #[case(13, 1980)]
    #[case(14, 2160)]
    #[case(15, 2340)]
    #[case(16, 2520)]
    #[case(17, 2700)]
    #[case(18, 2880)]
    #[case(19, 3060)]
    #[case(20, 3240)]
    fn test_polygon_angle_sum(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(polygon_angle_sum(input), expected);
    }
}
