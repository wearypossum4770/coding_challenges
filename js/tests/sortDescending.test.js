import { sortDescending } from "../src/sortDescending.js";

test.each([
  [123, 321],
  [1254859723, 9875543221],
  [73065, 76530],
  [123, 321],
  [670276097, 977766200],
  [2619805, 9865210],
  [81294, 98421],
  // [0000000, 0000000],
  [321, 321],
  [628904, 986420],
  [289327560, 987653220],
  [6456, 6654],
  [444111888555333, 888555444333111],
])("Unordered numbers %s are Ordered as such %s", (testInput, funcOutput) => {
  let func = sortDescending(testInput);
  expect(func).toEqual(funcOutput);
});
