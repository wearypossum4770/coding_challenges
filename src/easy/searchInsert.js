/**
 * @copyright https://leetcode.com/problems/search-insert-position/description/
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
const searchInsert = (nums, target) => {
  for (let i=0; i<nums.length;i++){
if (nums[i] >= target) return i
}
return nums.length
}

export default searchInsert

//[[1,3,5,6], 5, 2],
//[[1,3,5,6],2, 1],
//[[1,3,5,6], 7 ,4],
