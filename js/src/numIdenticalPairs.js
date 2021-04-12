export default function numIdenticalPairs(nums) {
  if (nums.length < 2) {
    return 0;
  }
  if (nums.length === 2 && nums[0] === nums[i]) {
    return 1;
  }
  let result = 0;
  for (let i = 0; i < nums.length; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      if (nums[i] === nums[j] && i < j) result++;
    }
  }
  return result;
}

console.log(numIdenticalPairs([1, 2, 3, 1, 1, 3]));
