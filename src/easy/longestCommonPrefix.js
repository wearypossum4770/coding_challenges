/**
 * @copyright https://leetcode.com/problems/longest-common-prefix/submissions/
 * @param {Array<string>} strs
 * @returns
 */
const longestCommonPrefix = (strs) => {
  let target = [];
  const compare = strs[0];
  let max = Math.max(...strs.map((str) => str.length));
  if (strs[0][0] !== strs.slice(-1)[0][0]) return target;
  for (let i = 0; i < max; i++) {
    for (let j = 0; j < strs[0].length; j++) {
      console.log(strs[i]);
      // console.log(compare[j], strs[i][j])
      // if (compare[j] === strs[0][j]) {
      // target.push(compare[i]);
      // }
    }
  }
  console.log(compare, target);
  if (target.length >= strs[0].length) return strs[0];
  return target;
};

console.log(`\n newline ${longestCommonPrefix(["dog", "racecar", "car"])}`);
console.log(
  `\n newline ${longestCommonPrefix(["flower", "flower", "flower", "flower"])}`
);
console.log(`\n newline ${longestCommonPrefix(["flower", "flow", "flight"])}`);
export default longestCommonPrefix;
