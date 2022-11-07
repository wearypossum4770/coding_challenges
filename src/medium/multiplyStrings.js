/**
 * not scores from my solution 
 * Runtime: 104 ms, faster than 66.12%
 * Memory Usage: 42.5 MB, less than 81.81%
 * @copyright https://leetcode.com/problems/multiply-strings/submissions/
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */
 const multiply = (num1, num2) =>  {
  const arr = Array(Math.max(num1.length, num2.length))
  let left = num1.split('').map(num => parseInt(num))
  let right = num2.split('').map(num => parseInt(num))

}


multiply("123456789","987654321")