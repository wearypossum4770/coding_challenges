pub fn reduce_to_zero_steps(num: u64) -> u64 {
    let mut count: u64 = 0;
    let mut done: bool = false;
    let mut new_number = num;
    while !done {
        if new_number == 0 {
            done = true;
        } else if new_number % 2 == 0 {
            count += 1;
            new_number /= 2;
        } else {
            count += 1;
            new_number -= 1;
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::reduce_to_zero_steps;
    #[test]
    fn test_reduce_to_zero_steps() {
        assert_eq!(reduce_to_zero_steps(14), 6);
        assert_eq!(reduce_to_zero_steps(8), 4);
        assert_eq!(reduce_to_zero_steps(123), 12);
    }
}
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Number of Steps to Reduce a Number to Zero.
// Memory Usage: 2 MB, less than 45.19% of Rust online submissions for Number of Steps to Reduce a Number to Zero.
