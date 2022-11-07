/**
 * @copyright https://leetcode.com/problems/length-of-last-word/
 * Challenge #58. Length of Last Word
 * Runtime: 66 ms, faster than 88.69% .
 * Memory Usage: 41.9 MB, less than 64.41% .
 * @param {string} s
 * @returns {number}
 */
const lengthOfLastWord = (s) => [...s.match(/\w+/g)].pop().length;
export default lengthOfLastWord;
