/**
 *
 *  @param {Array.<number>} nums  - a mutable array, whose length changes.
 *  @param {Array.<number>} index  - immutable array
 *  @copyright https://leetcode.com/problems/create-target-array-in-the-given-order/
 *  Leetcode Problem 1389
 *  Runtime: 80 ms, faster than 50.48%
 *  Memory Usage: 38.6 MB, less than 58.13%
 */
export default function createTargetArray(nums, index) {
  let target = Array.from(nums.length);
  for (let i = 0; i < index.length; i++) {
    target.splice(index[i], 0, nums[i]);
  }
  return target;
}
