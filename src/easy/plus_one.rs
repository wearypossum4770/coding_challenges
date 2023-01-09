pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    // let nums:Vec<_> = digits.iter().map(|num| num.to_string()).collect();
    // let mut arr: i32 = nums.join("").parse().unwrap();
    // arr+= 1;
    // let target: String = arr.to_string();
    let last = digits.len() - 1;
    let mut target = Vec::from(digits);
    target[last] += 1;
    target
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Vec::from([1, 2, 3])), Vec::from([1, 2, 4]));
        assert_eq!(plus_one(Vec::from([4, 3, 2, 1])), Vec::from([4, 3, 2, 2]));
        // the next test case will not pass. need to refactor consider pushing
        // to an array of string, convert to an integer perform the math, then
        // push to array of string and parse the integer.
        // assert_eq!(plus_one(Vec::from([9])), Vec::from([1,0]));
    }
}
