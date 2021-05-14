import createTargetArray from "../src/createTargetArray.js";

test.each([
  [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 2, 1],
    [0, 4, 1, 3, 2],
  ],
  [
    [1, 2, 3, 4, 0],
    [0, 1, 2, 3, 0],
    [0, 1, 2, 3, 4],
  ],
  [[1], [0], [1]],
])("create and organize an array", (nums, indices, output) => {
  let func = createTargetArray(nums, indices);
  expect(func).toStrictEqual(output);
});
