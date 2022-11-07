/**
 *
 * @copyright https://leetcode.com/problems/plus-one/
 * Challenge 66. Plus One
 * @param {Array.<number>} digits
 * @returns {Array.<number>}
 */
const plusOne = (digits) => {
  let target = parseInt(digits.join(""));
  target += 1;
  return target.toString().split("");
};
export default plusOne;
// console.log(plusOne([6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,3]))
