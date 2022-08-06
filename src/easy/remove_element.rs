pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  let  target = 0i32;
  for (pos, num) in nums.iter().enumerate() {
    if num == &val {
      nums[target as usize] = nums[pos];
      target + 1;
    }
    println!("{:?} {:?}", pos, num);
  }
  1
}

#[cfg(test)]
mod tests {
    use super::remove_element;
    #[test]
    fn test_remove_element() {
        assert_eq!(remove_element(&mut Vec::from([3, 2, 2, 3]), 3), 2) //[2,2];
        // assert_eq!(remove_element(Vec::from([0,1,2,2,3,0,4,2]), 2), [0,1,4,0,3]);
    }
}
