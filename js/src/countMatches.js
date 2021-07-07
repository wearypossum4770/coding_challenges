/**
 * 1773.
 * @param {string[][]} items
 * @param {string} ruleKey
 * @param {string} ruleValue
 * @return {number}
 * @copyright https://leetcode.com/problems/count-items-matching-a-rule/submissions/
 * Leet Code
 * Runtime: 108 ms, faster than 6.18%.
 * Memory Usage: 43.9 MB, less than 6.99%.
 * After refactoring:
 * Runtime: 84 ms, faster than 94.64%.
 * Memory Usage: 42.4 MB, less than 71.33%.
 */
export default function countMatches(items, ruleKey, ruleValue) {
  let count = 0;
  function counter(position) {
    items.forEach((item) => (item[position] === ruleValue ? count++ : count));
  }
  switch (ruleKey) {
    case "type":
      counter(0);
      break;
    case "color":
      counter(1);
      break;
    default:
      counter(2);
      break;
  }
  return count;
}
console.log(
  countMatches(
    [
      ["phone", "blue", "pixel"],
      ["computer", "silver", "phone"],
      ["phone", "gold", "iphone"],
    ],
    "type",
    "phone"
  )
);
console.log(
  countMatches(
    [
      ["phone", "blue", "pixel"],
      ["computer", "silver", "lenovo"],
      ["phone", "gold", "iphone"],
    ],
    "color",
    "silver"
  )
);
