/**
 * Runtime: 74 ms, faster than 77.73% for Remove Element.
 * Memory Usage: 42 MB, less than 59.54%
 * @copyright https://leetcode.com/problems/remove-element/submissions/
 * wasn't able to solve by self. the directions were unclear.
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
const removeElement = (nums, val) => {
  let i = 0;
  for (let j = 0; j < nums.length; j++) {
    if (nums[j] !== val) {
      nums[i] = nums[j];
      i++;
    }
  }
  return i;
};
console.log(removeElement([3, 2, 2, 3], 3));
//  export default removeElement
