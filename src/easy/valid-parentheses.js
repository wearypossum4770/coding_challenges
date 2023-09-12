/**
 * 
 * @copyright https://leetcode.com/problems/valid-parentheses/description/
 * @description
 * Leet Code #20 Valid Parentheses
 * @param {string} s
 * @return {boolean}
 */

const isValidParentheses = (s) => {
    const [stack, parens] = [[], "() {} []"]
    let i = 0;
    while(i< s.length) {
        stack.push(s[i])
        i++
        let [open, closed] = [stack.at(-2), stack.at(-1)]
        if (parens.includes(open+closed)) {
            stack.pop(), stack.pop()
        }
    }
    return stack.length === 0;
}

export default isValidParentheses;
// console.log(isValidParentheses("()")) // true
// console.log(isValidParentheses("()[]{}")) // true
// console.log(isValidParentheses("(]")) // false
// console.log(isValidParentheses("){")) // false