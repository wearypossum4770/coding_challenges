import balancedStringSplit from "../src/balancedStringSplit.js";

test.each([
  ["RLRRLLRLRL", 4],
  ["RLLLLRRRLR", 3],
  ["LLLLRRRR", 1],
  ["RLRRRLLRLL", 2],
])("Should count substrings", (input, output) => {
  let func = balancedStringSplit(input);
  expect(func).toEqual(output);
});
