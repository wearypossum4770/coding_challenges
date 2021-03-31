import tetrahedralNumber from "../src/tetrahedralNumber.js";

test.each([
  [1, 1],
  [2, 4],
  [3, 10],
  [4, 20],
  [5, 35],
  [9, 165],
])("should calculate tetrahedral number", (testInput, testOutput) => {
  let func = tetrahedralNumber(testInput);
  expect(func).toBe(testOutput);
});
