// LeetCode # 1281
// runtime: 80ms top 52.85%
// memory usage: 38.8mb top 44.95%
// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
export default function subtractProductAndSum(num) {
  let array = [...num.toString()];
  return (
    array.reduce((a, b) => parseInt(a) * parseInt(b)) -
    array.reduce((a, b) => parseInt(a) + parseInt(b))
  );
}
console.log(subtractProductAndSum(234));
