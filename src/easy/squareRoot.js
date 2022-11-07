/**
 * @copyright https://leetcode.com/explore/learn/card/binary-search/125/template-i/950/
 * Runtime: 192 ms 9.59%
 * Memory Usage: 43.3 MB 79.85%
 * @param {number} x
 * @returns
 */
const squareRoot = (x) => {
  if (x < 1) return 0;
  for (let i = 0; i <= x; i++) {
    if (i * i === x) return i;
    if (i * i > x) return i - 1;
  }
};
console.log(squareRoot(8));
console.log(squareRoot(4));
console.log(squareRoot(1));
