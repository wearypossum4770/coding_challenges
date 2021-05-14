import checkEquals from "../src/checkEquals.js";
test.each([
  [[1, 2], [1, 3], false],
  [[1, 2], [1, 2], true],
  [[4, 5, 6], [4, 5, 6], true],
  [[4, 7, 6], [4, 5, 6], false],
  [[4, 7, 6], [4, 6, 7], false],
])("Test equality of inputs", (a, b, output) => {
  let func = checkEquals(a, b);
  expect(func).toBe(output);
});
