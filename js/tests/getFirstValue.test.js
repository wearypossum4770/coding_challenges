import getFirstValue from "../src/getFirstValue.js";
test.each([
  [[1, 2, 3], 1],
  [[80, 5, 100], 80],
  [[-500, 0, 50], -500],
  [[5, 2, 3], 5],
  [[75675, 5, 100], 75675],
  [[-52320, 0, 50], -52320],
])("returns the first element in an array", (testInput, testOutput) => {
  let func = getFirstValue(testInput);
  expect(func).toBe(testOutput);
});
