import { runningSum } from "../src/easy/runningSum.js";

test.each([
  [
    [1, 2, 3, 4],
    [1, 3, 6, 10],
  ],
  [
    [1, 1, 1, 1, 1],
    [1, 2, 3, 4, 5],
  ],
  [
    [3, 1, 2, 10, 1],
    [3, 4, 6, 16, 17],
  ],
])("The running sum of the array ", (testInput, testOutput) => {
  let testCase = runningSum(testInput);
  expect(testCase).toStrictEqual(testOutput);
});
