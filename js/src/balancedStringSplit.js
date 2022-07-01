/**
 *
 * @param {string} s
 */
export default function balancedStringSplit(s) {
  let first = s[0],
    first_change = false,
    second_change = false;
  for (let index = 0; index < s.length; index++) {
    console.log(first === s[index]);
  }
}
console.log(balancedStringSplit("RLRRLLRLRL"));
console.log(balancedStringSplit("RLLLLRRRLR"));
console.log(balancedStringSplit("LLLLRRRR"));
console.log(balancedStringSplit("RLRRRLLRLL"));
