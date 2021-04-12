/// LeetCode # 1281
/// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

pub fn subtract_product_and_sum(num: i32) -> i32 {
    let integer = 32i32;
    let string = num.to_string();
    // let array:Vec<i32> = string.parse.collect();
    println!("{:?}", string);
    println!("{:?}", string.len());
    integer
}

#[cfg(test)]
mod tests {
    use super::subtract_product_and_sum;
    #[test]
    fn test_subtract_product_and_sum() {
        // assert_eq!(subtract_product_and_sum(234), 15);
        // assert_eq!(subtract_product_and_sum(4421), 1);
        assert_eq!(subtract_product_and_sum(4421), 32);
    }
}
