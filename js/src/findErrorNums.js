export default function findErrorNums(nums) {
  let target = [];
  for (let index = 0; index < nums.length; index++) {
    if (nums[index + 1] === nums[index + 2]) {
        console.log(index+1)
        console.log(index+2)
      if (nums[index + 1] !== undefined) {
        target.push(index + 1);
      }
      if (index && index > 0) {
        target.push(index);
      }
    }
  }
  return target;
}

console.log(findErrorNums([1, 2, 2, 4]));
console.log(findErrorNums([1, 1]));