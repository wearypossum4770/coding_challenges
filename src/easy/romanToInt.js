/**
 * @copyright edabit - https://edabit.com/challenge/qaJQMgCRftrh5kJyx
 * @copyright leetcode - https://leetcode.com/problems/roman-to-integer/
 * Leet code Challenge #13
 * Runtime: 270 ms, 16.53% faster.
 * Memory Usage: 48.4 MB, 28.56% less memory.
 * @param {string} roman
 */
const romanMap = new Map([
  ["M", 1000],
  ["D", 500],
  ["C", 100],
  ["L", 50],
  ["X", 10],
  ["V", 5],
  ["I", 1],
]);
const romanToInt = (roman) =>
  roman.split("").reduce((accum, curr, index, array) => {
    if (romanMap.get(curr) < romanMap.get(array[index + 1])) {
      accum -= romanMap.get(curr);
    } else {
      accum += romanMap.get(curr);
    }
    return accum;
  }, 0);
export default romanToInt;
