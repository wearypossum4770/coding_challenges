import subtractProductAndSum from "../src/subtractProductAndSum.js";

test.each([
  [4421, 21],
  [234, 15],
])("test should work", (num, output) => {
  let func = subtractProductAndSum(num);
  expect(func).toBe(output);
});
