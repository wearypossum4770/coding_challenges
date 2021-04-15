export default function longestValidParentheses(s) {
  let count = 0;
  for (let index = 0; index < s.length; index++) {
    if (s[index] === "(" && s[index + 1] === ")") {
      count += 2;
    }
  }
  return count;
}

console.log(longestValidParentheses("(()"));
console.log(longestValidParentheses(")()())"));
console.log(longestValidParentheses("()(())"));
