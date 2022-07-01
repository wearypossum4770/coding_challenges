/**
 * Leetcode #9
 * Runtime: 176 ms, faster than 91.77% of JavaScript online submissions for Palindrome Number.
 * Memory Usage: 48.8 MB, less than 41.48% of JavaScript online submissions for Palindrome Number.
 */
export default function isPalindrome(x) {
  if (x < 0) {
    return false;
  } else {
    let string = x.toString();
    return string === string.split("").reverse().join("");
  }
}
