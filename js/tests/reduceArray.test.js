import reduceArray from "../src/reduceArray.js";

test("gives correct summation", () => {
  const func = reduceArray([1, 2, 3, 4, 10, 11]);
  expect(func).toEqual(31);
});
