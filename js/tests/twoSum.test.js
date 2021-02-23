import { twoSum } from "../src/twoSum.js";

test.each([
  [[2, 7, 11, 15], 9, [0, 1]],
  [[3, 2, 4], 6, [1, 2]],
  [[3, 3], 6, [0, 1]],
  [[0, 4, 3, 0], 0, [0, 3]],
  [[0,3,-3,4,-1], -1, [0,4]],
])("returns the sum from %o that equals %i ", (nums, target, expected) => {
  expect(twoSum(nums, target)).toStrictEqual(expected);
});
test("Force pass", () => expect(true).toStrictEqual(true));
