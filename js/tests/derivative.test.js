test.each([
  [[1, 4], 1],
  [[3, -2], 12],
  [[4, -3], -108],
  [[9, -5], 3515625],
  [[1254, 0], 0],
  [[-2, 10], -0.002],
])("correctly adds numbers", (test_input, expected) => {
  const func = addition(...test_input);
  expect(func).toEqual(expected);
});
