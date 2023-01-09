/**
 * @copyright https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=hot&query=
 * @param {string} parentheses
 * @returns
 */
const isValidParentheses = (parentheses) => {
  let stack = Array(parentheses.length);
  /**
   * If the first character is a closing bracket, the condition
   * is not meet. False should be returned.
   */
  if (!["(", "{", "["].includes(parentheses[0])) return false;
  return stack;
};
console.log(isValidParentheses(""));
console.log(isValidParentheses("()"));
console.log(isValidParentheses("()[]{}"));
// console.log(isValidParentheses("(]"))
