///
/// https://edabit.com/challenge/6R6gReGTGwzpwuffD
///
pub fn seven_boom(arr: Vec<i32>) -> String {
    let mut target = String::new();
    let mut found: bool = false;
    for num in arr {
        if num.to_string().contains('7') {
            found = true;
            break;
        }
    }
    if found {
        target.push_str("Boom!");
    } else {
        target.push_str("there is no 7 in the array");
    }
    target
}
#[cfg(test)]
mod tests {
    use super::seven_boom;
    #[test]
    fn test_seven_boom() {
        assert_eq!(seven_boom(vec![2, 6, 7, 9, 3]), "Boom!");
        assert_eq!(
            seven_boom(vec![33, 68, 400, 5]),
            "there is no 7 in the array"
        );
        assert_eq!(
            seven_boom(vec![86, 48, 100, 66]),
            "there is no 7 in the array"
        );
        assert_eq!(seven_boom(vec![76, 55, 44, 32]), "Boom!");
        assert_eq!(seven_boom(vec![35, 4, 9, 37]), "Boom!");
    }
}
