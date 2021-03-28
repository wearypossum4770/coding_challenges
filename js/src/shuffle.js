export default function shuffle(nums, n) {
  let target = [];
  let intermediate = [];
  while (nums.length) {
    intermediate.push(nums.splice(0, n));
  }
  for (let arr = 0; arr < n; arr++) {
    target.push(intermediate[0][arr]);
    target.push(intermediate[1][arr]);
  }
  return target;
}

console.log(shuffle([1, 1, 2, 2], 2));
