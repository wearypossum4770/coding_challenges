/**
 * Test cases passed but exceeded max time.
 * https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3697/
 */

export default function isIdealPermutation(nums) {
  let gInversions = 0,
    lInversions = 0,
    n = nums.length;
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] > nums[i + 1] && i >= 0 && i < n - 1) {
      lInversions += 1;
    }
    for (let j = 0; j < nums.length; j++) {
      if (nums[i] > nums[j] && i >= 0 && i < j && j < n) {
        gInversions += 1;
      }
    }
  }
  return lInversions === gInversions;
}
console.log(isIdealPermutation([1, 0, 2])); //true
console.log(isIdealPermutation([1, 2, 0])); //false
