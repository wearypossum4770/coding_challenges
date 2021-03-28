pub fn maximum_wealth(accounts: Vec<Vec<i64>>) -> i64 {
    let mut maximum = Vec::new();
    for account in accounts {
        let sum: i64 = account.iter().sum();
        maximum.push(sum);
    }
    let target = maximum.iter().max().unwrap();
    *target
}

// #[cfg(test)]
// mod tests {
//     use super::maximum_wealth;
//     #[test]
//     fn test_maximum_wealth() {
//         assert_eq!(maximum_wealth(vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]]), 10);

//     }
// }
