import cardHide from "../src/cardHide.js";

test.each([
  ["1234123456785678", "************5678"],
  ["8754456321113213", "************3213"],
  ["35123413355523", "**********5523"],
])("should mask the credit card number", (testInput, testOutput) => {
  let func = cardHide(testInput);
  expect(func).toEqual(testOutput);
});
