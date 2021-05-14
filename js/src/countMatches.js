/**
 * @param {string[][]} items
 * @param {string} ruleKey
 * @param {string} ruleValue
 * @return {number}
 * @copyright https://leetcode.com/problems/count-items-matching-a-rule/submissions/
 * Leet Code
 * Runtime: 108 ms, faster than 6.18%.
 * Memory Usage: 43.9 MB, less than 6.99%.
 */
export default function countMatches(items, ruleKey, ruleValue) {
    let count = 0;
  items.forEach((array) => {
    if (ruleKey === "type" && array[0] === ruleValue) {
      count += 1;
    } else if (ruleKey === "color" && array[1] === ruleValue) {
      console.log("type");
      count += 1;
    } else if (ruleKey === "name" && array[2] === ruleValue) {
      count += 1;
    }
  });
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
