/**
 * https://leetcode.com/explore/challenge/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3701/
 * @param {*} digits
 * @returns
 */
export default function letterCombinations(digits) {
  let __digi__ = [...digits];
  let mapper = new Map([
    ["2", ["a", "b", "c"]],
    ["3", ["d", "e", "f"]],
    ["4", ["g", "h", "i"]],
    ["5", ["j", "k", "l"]],
    ["6", ["m", "n", "o"]],
    ["7", ["p", "q", "r", "s"]],
    ["8", ["t", "u", "v"]],
    ["8", ["w", "x", "y", "z"]],
  ]);
  let array = __digi__.map((digit) =>
    mapper.get(digit).map((chars) => digit + chars)
  );
  return array;
  //   digits.forEach(digit=>mapper.get(digit).map(chars=>digit+chars))
}

console.log(letterCombinations("23"));
