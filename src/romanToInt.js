/**
 * @copyright edabit - https://edabit.com/challenge/qaJQMgCRftrh5kJyx
 * @copyright leetcode - https://leetcode.com/problems/roman-to-integer/
 * Leet code Challenge #13
 * @param {string} roman
 */
export default function romanToInt(roman) {
  const dictionary = Object.fromEntries([
    ["I", 1],
    ["V", 5],
    ["X", 10],
    ["L", 50],
    ["C", 100],
    ["D", 500],
    ["M", 1000],
  ]);
}
